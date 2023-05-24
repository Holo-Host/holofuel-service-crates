use anyhow::{Context, Result};
use holochain_types::prelude::{
    holochain_serial, AgentPubKeyB64, CapSecret, EntryHashB64, SerializedBytes,
};
use holochain_types::prelude::{ActionHashB64, AnyDhtHashB64};
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;
use tracing::debug;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct Ledger {
    pub balance: String,
    pub promised: String,
    pub fees: String,
    pub available: String,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct Pending {
    pub invoice_pending: Vec<Transaction>,
    pub promise_pending: Vec<Transaction>,
    pub invoice_declined: Vec<Transaction>,
    pub promise_declined: Vec<Transaction>,
    pub accepted: Vec<Transaction>,
}
#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct Actionable {
    pub invoice_actionable: Vec<Transaction>,
    pub promise_actionable: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct Transaction {
    pub id: EntryHashB64,
    pub amount: String,
    pub fee: String,
    pub created_date: String,
    pub completed_date: Option<String>,
    pub transaction_type: TransactionType,
    pub counterparty: AgentPubKeyB64,
    pub direction: TransactionDirection,
    pub status: TransactionStatus,
    pub note: Option<String>,
    pub proof_of_service_token: Option<CapSecret>,
    pub url: Option<String>,
    pub expiration_date: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub enum TransactionType {
    Request, //Invoice
    Offer,   //Promise
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub enum TransactionDirection {
    Outgoing, // To(Address),
    Incoming, // From(Address),
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub enum TransactionStatus {
    Actionable, // tx that is create by 1st instance and waiting for counterparty to complete the tx
    Pending,    // tx that was created by 1st instance and second instance
    Accepted,   // tx that was accepted by counterparty but has yet to complete countersigning.
    Completed,
    Declined,
    Expired,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct Profile {
    pub agent_address: AgentPubKeyB64,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub uniqueness: AnyDhtHashB64,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct Reserve {
    pub reserve_id: ActionHashB64,
    pub pub_key: AgentPubKeyB64,
    pub info: ReserveSetting,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct ReserveSetting {
    pub external_reserve_currency: String,
    pub external_account_number: String,
    pub external_signing_key: [u8; 32],
    pub default_promise_expiry: Duration,
    pub min_external_currency_tx_size: String,
    pub max_external_currency_tx_size: String,
    note: Option<String>,
}
impl ReserveSetting {
    pub fn load_happ_file() -> Result<Self> {
        use std::fs::File;
        debug!("loading happ file");
        let path = std::env::var("REGISTER_RESERVE")
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
    pub inputs_used: Vec<String>,
}
