use anyhow::{anyhow, Context, Result};
use holochain_types::prelude::{
    hash_type::Agent, holochain_serial, ExternIO, FunctionName, HoloHashB64, SerializedBytes,
    ZomeName,
};
use hpos_hc_connect::{holofuel_types::ReserveSetting, HolofuelAgent};
use serde::Deserialize;
use serde::Serialize;
use std::env;
use tracing::debug;
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
    debug!("Setting nickname as {:?}", nickname);
    let _a = load_happ_file()?;
    debug!("settings {:?}", _a);
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

pub fn load_happ_file() -> Result<ReserveSetting> {
    debug!("loading happ file");
    let path = std::env::var("REGISTER_RESERVE")
        .context("Failed to read REGISTER_RESERVE. Is it set in env?")?;
    debug!("got path {}", path);
    // let file = File::open(path).context("failed to open file")?;
    let file = std::fs::read(path)?;
    debug!("got file: {:?}", file);
    let happ_file =
        serde_json::from_slice(&file).context("failed to deserialize YAML as HappsFile")?;
    debug!("happ file {:?}", happ_file);
    Ok(happ_file)
}

pub fn fee_collector_pubkey() -> Result<HoloHashB64<Agent>> {
    let key = env::var("FEE_COLLECTOR_PUBKEY")
        .context("Failed to read FEE_COLLECTOR_PUBKEY. Is it set in env?")?;
    Ok(HoloHashB64::from_b64_str(&key)?)
}

pub fn expect_pubkey() -> Option<HoloHashB64<Agent>> {
    if let Ok(key) = env::var("EXPECT_PUBKEY") {
        if let Ok(k) = HoloHashB64::from_b64_str(&key) {
            return Some(k);
        }
    }
    None
}
