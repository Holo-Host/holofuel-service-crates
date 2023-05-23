use anyhow::{anyhow, Context, Result};
use holochain_types::prelude::{
    hash_type::Agent, holochain_serial, ExternIO, FunctionName, HoloHashB64, SerializedBytes,
    ZomeName,
};
use hpos_hc_connect::HolofuelAgent;
use reserve_init::ReserveSetting;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use tracing::info;
mod reserve_init;

/// Initialize the holofuel app on a holochain instance server
/// Holochain app require one zome call to initialize the init function
/// Holofuel has a fee_collection function scheduled on init
/// This is why we will be setting a profile name for holofuel the holofuel instance
#[tokio::main]
async fn main() -> Result<()> {
    info!("Start initializing the holofuel instance");

    let mut agent = HolofuelAgent::connect().await?;

    #[derive(Serialize, Deserialize, Debug, SerializedBytes)]
    pub struct ProfileInput {
        pub nickname: Option<String>,
        pub avatar_url: Option<String>,
    }

    let (_, apk) = agent.get_cell().await?;
    if let Some(ek) = expect_pubkey() {
        if ek != apk.clone().into() {
            return Err(anyhow!(
                "Unexpected agent {:?} found on this server, expected: {:?} ",
                apk,
                ek
            ));
        }
    }

    let fpk = fee_collector_pubkey()?;
    let mut nickname = Some("Holo Account".to_string());
    if fpk == apk.into() {
        nickname = Some("Holo Fee Collector".to_string());
    }
    if ReserveSetting::load_happ_file().is_ok() {
        nickname = Some("HOT Reserve".to_string());
    }

    if let Ok(_) = agent
        .zome_call(
            ZomeName::from("profile"),
            FunctionName::from("update_my_profile"),
            ExternIO::encode(ProfileInput {
                nickname,
                avatar_url: None,
            })?,
        )
        .await
    {
        info!("Profile name set successfully");
    };

    // initialize reserve details
    reserve_init::set_up_reserve(agent).await?;
    info!("Completed initializing the holofuel instance");
    Ok(())
}

pub fn fee_collector_pubkey() -> Result<HoloHashB64<Agent>> {
    let key = env::var("FEE_COLLECTOR_PUBKEY")
        .context("Failed to read FEE_COLLECTOR_PUBKEY. Is it set in env?")?;
    Ok(HoloHashB64::from_b64_str(&key)?)
}

pub fn expect_pubkey() -> Option<HoloHashB64<Agent>> {
    match env::var("EXPECT_PUBKEY") {
        Ok(key) => {
            Some(HoloHashB64::from_b64_str(&key).expect("unable to deserialized EXPECT_PUBKEY"))
        }
        Err(_) => None,
    }
}
