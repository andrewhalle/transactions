use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

/*
 * I would normally structure this as
 *
 * #[derive(Debug, Deserialize)]
 * #[serde(tag = "type", rename_all = "lowercase")]
 * pub enum Transaction {
 *   Deposit { amount: u64, client: u16, id: u32 },
 *   ...
 * }
 *
 * which works with JSON but doesn't seem to work with CSV because of
 * https://github.com/BurntSushi/rust-csv/issues/211
 */
#[derive(Debug, Deserialize)]
pub struct Transaction {
    r#type: TransactionType,
    amount: Option<String>,
    client: u16,
    #[serde(rename = "tx")]
    id: u32,
}
