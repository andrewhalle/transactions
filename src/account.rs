use serde::{Serialize, Serializer};
use thiserror::Error;

fn i64_as_money_string<S: Serializer>(val: &i64, s: S) -> Result<S::Ok, S::Error> {
    let mut val = *val;

    let mut negative = false;
    if val < 0 {
        negative = true;
        val *= -1;
    }

    let whole = val / 10000;
    let fractional = val % 10000;

    s.serialize_str(&format!(
        "{}{}.{}",
        if negative { "-" } else { "" },
        whole,
        format!("{:0>4}", fractional)
    ))
}

#[derive(Debug, Serialize)]
pub struct Account {
    #[serde(rename = "client")]
    pub id: u16,
    #[serde(serialize_with = "i64_as_money_string")]
    pub available: i64,
    #[serde(serialize_with = "i64_as_money_string")]
    pub held: i64,
    #[serde(serialize_with = "i64_as_money_string")]
    pub total: i64,
    pub frozen: bool,
}

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("not enough available")]
    NotEnoughAvailable,
}
use AccountError::*;

impl Account {
    pub fn new(id: u16) -> Self {
        Account {
            id,
            available: 0,
            held: 0,
            total: 0,
            frozen: false,
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.available += amount as i64;
        self.total += amount as i64;
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<(), AccountError> {
        if self.available < amount as i64 {
            Err(NotEnoughAvailable)
        } else {
            self.force_withdraw(amount);

            Ok(())
        }
    }

    pub fn force_withdraw(&mut self, amount: u64) {
        self.available -= amount as i64;
        self.total -= amount as i64;
    }

    pub fn hold(&mut self, amount: u64) {
        self.available -= amount as i64;
        self.held += amount as i64;
    }

    pub fn release(&mut self, amount: u64) {
        self.available += amount as i64;
        self.held -= amount as i64;
    }
}
