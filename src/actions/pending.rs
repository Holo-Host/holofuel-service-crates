use super::{hf_connection::HolofuelAgent, holofuel_types::Pending};
use anyhow::Result;
use holochain_types::prelude::{FunctionName, ZomeName};

pub async fn get() -> Result<()> {
    let mut agent = HolofuelAgent::connect().await?;
    let result = agent
        .zome_call(
            ZomeName::from("transactor"),
            FunctionName::from("get_pending_transactions"),
        )
        .await?;

    let txs: Pending = rmp_serde::from_slice(result.as_bytes())?;

    println!("===================");
    println!("Your Pending List is: ");
    println!("{:?}", txs);
    println!(
        "Number of pending: {:?}",
        txs.invoice_pending.len() + txs.promise_pending.len()
    );
    println!(
        "Number of declined: {:?}",
        txs.invoice_declined.len() + txs.promise_declined.len()
    );
    println!(
        "Number of accepted (waiting for scheduler to complete it): {:?}",
        txs.accepted.len()
    );
    println!("===================");

    Ok(())
}
