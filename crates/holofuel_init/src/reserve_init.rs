use anyhow::Result;
use holochain_types::prelude::{ExternIO, FunctionName, ZomeName};
use hpos_hc_connect::holofuel_types::{Reserve, ReserveSalePrice, ReserveSetting};
use hpos_hc_connect::HolofuelAgent;
use tracing::log::trace;
use tracing::{info, log::warn};

pub async fn set_up_reserve(mut agent: HolofuelAgent) -> Result<()> {
    trace!("Setting up reserve settings...");
    match ReserveSetting::load_happ_file() {
        Ok(reserve_settings) => {
            trace!("Getting all reserve account details");
            let result = agent
                .zome_call(
                    ZomeName::from("reserves"),
                    FunctionName::from("get_all_reserve_accounts_details"),
                    ExternIO::encode(())?,
                )
                .await?;
            let reserve: Vec<Reserve> = rmp_serde::from_slice(result.as_bytes())?;
            if reserve.is_empty() {
                trace!("Setting reserve details");
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
            } else {
                info!("Reserve settings: {:?}", reserve);
            }
        }
        Err(e) => warn!("{}", e),
    }

    Ok(())
}
