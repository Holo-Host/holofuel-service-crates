use anyhow::Result;
use holochain_types::prelude::{ExternIO, FunctionName, ZomeName};
use hpos_hc_connect::holofuel_types::Ledger;
use hpos_hc_connect::{CoreAppAgent, CoreAppRoleName};

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
pub struct PresentedHappBundle {
    pub id: ActionHashB64,
    pub provider_pubkey: AgentPubKeyB64,
    pub is_draft: bool,
    pub is_paused: bool,
    pub uid: Option<String>,
    pub bundle_url: String,
    pub ui_src_url: Option<String>,
    pub dnas: Vec<DnaResource>,
    pub hosted_urls: Vec<String>,
    pub name: String,
    pub logo_url: Option<String>,
    pub description: String,
    pub categories: Vec<String>,
    pub jurisdictions: Vec<String>,
    pub exclude_jurisdictions: bool,
    pub hosting_prices: HostingPrices,
    pub login_config: LoginConfig,
    pub special_installed_app_id: Option<String>,
}

pub async fn get() -> Result<()> {
    let mut agent = CoreAppAgent::connect().await?;
    let result = agent
        .zome_call(
            CoreAppRoleName::HHA,
            ZomeName::from("hha"),
            FunctionName::from("get_my_happs"),
            ExternIO::encode(())?,
        )
        .await?;

    let happs: Vec<PresentedHappBundle> = rmp_serde::from_slice(result.as_bytes())?;

    println!("===================");
    println!("Your Published Happs is: ");
    println!("{:?}", happs);
    println!("===================");

    Ok(())
}
