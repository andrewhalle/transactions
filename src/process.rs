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

fn check_not_exists(txns: &HashMap<u32, Transaction>, id: u32) {
    if txns.contains_key(&id) {
        // this transaction has already been processed
        // TODO: don't panic
        panic!("already processed transaction");
    }
}

// designed this way so that if transactions were coming in from multiple sources, I could share
// the state by putting it in a Mutex
pub fn process_one(state: &mut State, transaction: Transaction) {
    let account = state
        .accounts
        .entry(transaction.client)
        .or_insert(Account::new(transaction.client));

    match transaction.r#type {
        Deposit => {
            check_not_exists(&state.transactions, transaction.tx);
            let amount = transaction.amount.expect("deposit must have an amount");
            state.transactions.insert(transaction.tx, transaction);

            account.deposit(amount);
        }
        Withdrawal => {
            check_not_exists(&state.transactions, transaction.tx);
            let amount = transaction.amount.expect("credit must have an amount");
            state.transactions.insert(transaction.tx, transaction);

            account.withdraw(amount);
        }
        Dispute => {
            let disputed_transaction = state
                .transactions
                .get_mut(&transaction.tx)
                .expect("disputed transaction does not exist");
            disputed_transaction.disputed = true;

            let mut amount = disputed_transaction.amount.expect("must have amount") as i64;
            if let Withdrawal = disputed_transaction.r#type {
                amount *= -1;
            }

            account.hold(amount);
        }
        Resolve => {
            let disputed_transaction = state
                .transactions
                .get_mut(&transaction.tx)
                .expect("disputed transaction does not exist");
            if !disputed_transaction.disputed {
                // TODO: don't panic
                panic!("transaction not disputed");
            }
            disputed_transaction.disputed = false;

            let mut amount = disputed_transaction.amount.expect("must have amount") as i64;
            if let Withdrawal = disputed_transaction.r#type {
                amount *= -1;
            }

            account.release(amount);
        }
        Chargeback => {
            let disputed_transaction = state
                .transactions
                .get(&transaction.tx)
                .expect("disputed transaction does not exist");
            if !disputed_transaction.disputed {
                // TODO: don't panic
                panic!("transaction not disputed");
            }

            let amount = disputed_transaction.amount.expect("must have amount");
            match disputed_transaction.r#type {
                Deposit => account.withdraw(amount),
                Withdrawal => account.deposit(amount),
                _ => unreachable!(),
            }

            account.frozen = true;
        }
    }
}
