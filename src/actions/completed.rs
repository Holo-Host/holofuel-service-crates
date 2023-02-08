use super::{hf_connection::HolofuelAgent, holofuel_types::Transaction};
use anyhow::Result;
use holochain_types::prelude::{FunctionName, ZomeName};

pub async fn get() -> Result<()> {
    let mut agent = HolofuelAgent::connect().await?;
    let result = agent
        .zome_call(
            ZomeName::from("transactor"),
            FunctionName::from("get_completed_transactions"),
        )
        .await?;

    let txs: Vec<Transaction> = rmp_serde::from_slice(result.as_bytes())?;

    println!("===================");
    println!("Your Completed List is: ");
    println!("{:?}", txs);
    println!("Number of completed tx: {:?}", txs.len());
    println!("===================");

    Ok(())
}
