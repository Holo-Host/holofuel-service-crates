use std::env;

use anyhow::{Context, Result};
use holochain_types::prelude::{
    hash_type::Agent, holochain_serial, ExternIO, FunctionName, HoloHashB64, SerializedBytes,
    ZomeName,
};
use holofuel_connect::HolofuelAgent;
use serde::Deserialize;
use serde::Serialize;

/// Initialize the holofuel app on a holochain instance server
/// Holochain app require one zome call to initialize the init function
/// Holofuel has a fee_collection function scheduled on init
/// This is why we will be setting a profile name for holofuel the holofuel instance
#[tokio::main]
async fn main() -> Result<()> {
    let mut agent = HolofuelAgent::connect().await?;

    #[derive(Serialize, Deserialize, Debug, SerializedBytes)]
    pub struct ProfileInput {
        pub nickname: Option<String>,
        pub avatar_url: Option<String>,
    }

    let fpk = fee_collector_pubkey()?;
    let (_, apk) = agent.get_cell().await?;
    let mut nickname = Some("Holo Account".to_string());
    if fpk == apk.into() {
        nickname = Some("Holo Fee Collector".to_string());
    }
    agent
        .zome_call(
            ZomeName::from("profile"),
            FunctionName::from("update_my_profile"),
            ExternIO::encode(ProfileInput {
                nickname,
                avatar_url: None,
            })?,
        )
        .await?;
    Ok(())
}

pub fn fee_collector_pubkey() -> Result<HoloHashB64<Agent>> {
    let key = env::var("FEE_COLLECTOR_PUBKEY")
        .context("Failed to read FEE_COLLECTOR_PUBKEY. Is it set in env?")?;
    Ok(HoloHashB64::from_b64_str(&key)?)
}