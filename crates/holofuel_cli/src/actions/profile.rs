use super::holofuel_types::Profile;
use anyhow::Result;
use holochain_types::prelude::{ExternIO, FunctionName, ZomeName};
use holofuel_connect::HolofuelAgent;

pub async fn get() -> Result<()> {
    let mut agent = HolofuelAgent::connect().await?;
    let result = agent
        .zome_call(
            ZomeName::from("profile"),
            FunctionName::from("get_my_profile"),
            ExternIO::encode(())?,
        )
        .await?;

    let profile: Profile = rmp_serde::from_slice(result.as_bytes())?;

    println!("===================");
    println!("Your Profile details are: ");
    println!("{:?}", profile);
    println!("===================");

    Ok(())
}