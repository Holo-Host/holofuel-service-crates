use anyhow::{Context, Result};
use holochain_types::prelude::{
    holochain_serial, ExternIO, FunctionName, SerializedBytes, ZomeName,
};
use hpos_hc_connect::HolofuelAgent;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::time::Duration;
use tracing::debug;
use tracing::log::warn;

pub async fn set_up_reserve(mut agent: HolofuelAgent) -> Result<()> {
    match ReserveSetting::load_happ_file() {
        Ok(reserve_settings) => {
            // Setting initial reserve account details
            agent
                .zome_call(
                    ZomeName::from("reserves"),
                    FunctionName::from("register_reserve_account"),
                    ExternIO::encode(reserve_settings)?,
                )
                .await?;

            // Setting reserve sales price to 1
            // Current expectation is a 1 to 1 conversion
            // 1HF = 1HOT
            agent
                .zome_call(
                    ZomeName::from("reserves"),
                    FunctionName::from("set_sale_price"),
                    ExternIO::encode(ReserveSalePrice {
                        latest_unit_price: "1".to_string(),
                        inputs_used: vec![],
                    })?,
                )
                .await?;
        }
        Err(e) => warn!("{}", e),
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct ReserveSetting {
    pub external_reserve_currency: String,
    external_account_number: String,
    external_signing_key: [u8; 32], // X25519PubKey
    pub default_promise_expiry: Duration,
    pub min_external_currency_tx_size: String,
    pub max_external_currency_tx_size: String,
    note: Option<String>,
}
impl ReserveSetting {
    pub fn load_happ_file() -> Result<Self> {
        use std::fs::File;
        debug!("loading happ file");
        let path = env::var("REGISTER_RESERVE")
            .context("Failed to read REGISTER_RESERVE. Is it set in env?")?;
        debug!("got path {}", path);
        let file = File::open(path).context("failed to open file")?;
        debug!("got file: {:?}", file);
        let happ_file =
            serde_yaml::from_reader(&file).context("failed to deserialize YAML as HappsFile")?;
        debug!("happ file {:?}", happ_file);
        Ok(happ_file)
    }
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct ReserveSalePrice {
    pub latest_unit_price: String, // Number of HF units one external currency unit purchases, as determined by periodic (scheduled) runs of the pricing algorithm
    inputs_used: Vec<String>,
}
