use anyhow::Result;
use holochain_types::prelude::{ExternIO, FunctionName, ZomeName};
use hpos_hc_connect::holofuel_types::{ReserveSalePrice, ReserveSetting};
use hpos_hc_connect::HolofuelAgent;
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
