use super::holofuel_types::Ledger;
use anyhow::Result;
use holochain_types::prelude::{ExternIO, FunctionName, ZomeName};
use holofuel_connect::HolofuelAgent;

pub async fn get() -> Result<()> {
    let mut agent = HolofuelAgent::connect().await?;
    let result = agent
        .zome_call(
            ZomeName::from("transactor"),
            FunctionName::from("get_ledger"),
            ExternIO::encode(())?,
        )
        .await?;

    let ledger: Ledger = rmp_serde::from_slice(result.as_bytes())?;

    println!("===================");
    println!("Your Ledger is: ");
    println!("{:?}", ledger);
    println!("===================");

    Ok(())
}
