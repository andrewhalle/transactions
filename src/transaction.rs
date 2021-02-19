use serde::de;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

fn money_string_to_u64(s: String) -> Result<u64, TransactionError> {
    let mut pieces = s.split('.');

    let whole = pieces.next().ok_or(TransactionAmountImproperlyFormatted)?;
    let whole = whole
        .parse::<u64>()
        .or(Err(TransactionAmountImproperlyFormatted))?;

    let mut fractional = 0;
    if let Some(piece) = pieces.next() {
        if piece.len() > 4 {
            return Err(TransactionAmountImproperlyFormatted);
        }

        let padded = format!("{:0<4}", piece);
        fractional = padded
            .parse::<u64>()
            .or(Err(TransactionAmountImproperlyFormatted))?;
    }

    if pieces.next().is_some() {
        return Err(TransactionAmountImproperlyFormatted);
    }

    Ok((whole * 10000) + fractional)
}

fn amount_deserializer<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
    let buf = String::deserialize(d)?;

    if buf == "" {
        Ok(None)
    } else {
        Ok(Some(money_string_to_u64(buf).map_err(de::Error::custom)?))
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum TransactionError {
    #[error("transaction needs amount")]
    TransactionNeedsAmount,
    #[error("transaction amount improperly formatted")]
    TransactionAmountImproperlyFormatted,
}
use TransactionError::*;

#[derive(Clone, Copy, Debug, Deserialize)]
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
    pub amount: Option<u64>,
    pub client: u16,
    pub tx: u32,
    #[serde(skip_deserializing, default = "bool_false")]
    pub disputed: bool,
}

impl Transaction {
    pub fn amount(&self) -> Result<u64, TransactionError> {
        self.amount.ok_or(TransactionNeedsAmount)
    }
}

// serde requires a function for default values, can't use a literal
const fn bool_false() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_string_to_u64() {
        assert_eq!(money_string_to_u64("10".to_string()).unwrap(), 100000);
        assert_eq!(money_string_to_u64("10.".to_string()).unwrap(), 100000);
        assert_eq!(money_string_to_u64("10.1".to_string()).unwrap(), 101000);
        assert_eq!(money_string_to_u64("10.01".to_string()).unwrap(), 100100);
        assert_eq!(money_string_to_u64("10.001".to_string()).unwrap(), 100010);
        assert_eq!(money_string_to_u64("10.0001".to_string()).unwrap(), 100001);
        assert_eq!(
            money_string_to_u64("10.00001".to_string()),
            Err(TransactionAmountImproperlyFormatted)
        );
    }
}
