use serde::{Deserialize, Deserializer};

fn money_string_to_i64(s: String) -> i64 {
    let mut pieces = s.split(".");

    let whole = pieces.next().expect("expected chunk separated by .");
    let whole = whole.parse::<i64>().expect("could not parse integer");

    // support no fractional part
    let fractional = pieces.next().expect("expected chunk separated by .");
    let fractional = fractional.parse::<i64>().expect("could not parse integer");

    if pieces.next().is_some() {
        // TODO: don't panic
        panic!("too many .");
    }

    if fractional > 9999 {
        // TODO: don't panic
        panic!("too large a fractional part");
    }

    (whole * 10000) + fractional
}

fn amount_deserializer<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
    let buf = Option::<String>::deserialize(d)?;

    Ok(buf.map(money_string_to_i64))
}

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
    pub r#type: TransactionType,
    // would consider using fixed-point if needed to do anything more complex than adding and
    // subtracting
    #[serde(deserialize_with = "amount_deserializer")]
    pub amount: Option<i64>,
    pub client: u16,
    pub tx: u32,
}
