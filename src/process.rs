use std::collections::HashMap;

use crate::{transaction::TransactionType::*, Account, Transaction};

pub struct State {
    pub transactions: HashMap<u32, Transaction>,
    pub accounts: HashMap<u16, Account>,
}

impl State {
    pub fn new() -> Self {
        State {
            transactions: HashMap::new(),
            accounts: HashMap::new(),
        }
    }
}

// designed this way so that if transactions were coming in from multiple sources, I could share
// the state by putting it in a Mutex
pub fn process_one(state: &mut State, transaction: Transaction) {
    match transaction.r#type {
        Deposit => {
            let account = state
                .accounts
                .entry(transaction.client)
                .or_insert(Account::new(transaction.client));
            let amount = transaction.amount.expect("deposit must have an amount");

            account.available += amount;
        }
        _ => unimplemented!(),
    }
}
