use super::{hf_connection::HolofuelAgent, holofuel_types::Actionable};
use anyhow::Result;
use holochain_types::prelude::{FunctionName, ZomeName};

pub async fn get() -> Result<()> {
    let mut agent = HolofuelAgent::connect().await?;
    let result = agent
        .zome_call(
            ZomeName::from("transactor"),
            FunctionName::from("get_actionable_transactions"),
        )
        .await?;

    let txs: Actionable = rmp_serde::from_slice(result.as_bytes())?;

    println!("===================");
    println!("Your Actionable List is: ");
    println!("{:?}", txs);
    println!(
        "Number of actionable: {:?}",
        txs.invoice_actionable.len() + txs.promise_actionable.len()
    );
    println!("===================");

    Ok(())
}
