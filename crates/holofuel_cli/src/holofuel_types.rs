use holochain_types::prelude::AnyDhtHashB64;
use holochain_types::prelude::{
    holochain_serial, AgentPubKeyB64, CapSecret, EntryHashB64, SerializedBytes,
};
use serde::Deserialize;
use serde::Serialize;

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
