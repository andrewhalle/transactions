use std::collections::HashMap;
use thiserror::Error;

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

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error)]
pub enum TransactionProcessingError {
    #[error("transaction already processed")]
    TransactionAlreadyProcessed,
    #[error("transaction requires amount but one was not provided")]
    TransactionRequiresAmount,
    #[error("transaction does not exist")]
    TransactionDoesNotExist,
    #[error("transaction disputed")]
    TransactionDisputed,
    #[error("transaction not disputed")]
    TransactionNotDisputed,
    #[error("account locked")]
    AccountLocked,
    #[error("something went wrong")]
    TransactionProcessingAccountError {
        #[from]
        source: crate::account::AccountError,
    },
    #[error("something went wrong")]
    TransactionProcessingTransactionError {
        #[from]
        source: crate::transaction::TransactionError,
    },
}
use TransactionProcessingError::*;

fn insert_if_not_exists(
    txns: &mut HashMap<u32, Transaction>,
    t: Transaction,
) -> Result<(), TransactionProcessingError> {
    #[allow(clippy::map_entry)]
    if txns.contains_key(&t.tx) {
        Err(TransactionAlreadyProcessed)
    } else {
        txns.insert(t.tx, t);

        Ok(())
    }
}

fn get_transaction(
    txns: &mut HashMap<u32, Transaction>,
    id: u32,
) -> Result<&mut Transaction, TransactionProcessingError> {
    txns.get_mut(&id).ok_or(TransactionDoesNotExist)
}

fn get_disputed_transaction(
    txns: &mut HashMap<u32, Transaction>,
    id: u32,
) -> Result<&mut Transaction, TransactionProcessingError> {
    let t = get_transaction(txns, id)?;

    if !t.disputed {
        Err(TransactionNotDisputed)
    } else {
        Ok(t)
    }
}

fn get_undisputed_transaction(
    txns: &mut HashMap<u32, Transaction>,
    id: u32,
) -> Result<&mut Transaction, TransactionProcessingError> {
    let t = get_transaction(txns, id)?;

    if t.disputed {
        Err(TransactionDisputed)
    } else {
        Ok(t)
    }
}

// designed this way so that if transactions were coming in from multiple sources, I could share
// the state by putting it in a Mutex
pub fn process_one(
    state: &mut State,
    transaction: Transaction,
) -> Result<(), TransactionProcessingError> {
    let account = state
        .accounts
        .entry(transaction.client)
        .or_insert_with(|| Account::new(transaction.client));

    if account.locked {
        return Err(AccountLocked);
    }

    match transaction.r#type {
        Deposit => {
            let amount = transaction.amount()?;
            insert_if_not_exists(&mut state.transactions, transaction)?;

            account.deposit(amount);
        }
        Withdrawal => {
            let amount = transaction.amount()?;
            insert_if_not_exists(&mut state.transactions, transaction)?;

            account.withdraw(amount)?;

            // TODO if error occurred, might need to remove the inserted transaction, what if it's
            // later chargedback?
        }
        Dispute => {
            let disputed_transaction =
                get_undisputed_transaction(&mut state.transactions, transaction.tx)?;
            let amount = disputed_transaction.amount()?;
            disputed_transaction.disputed = true;

            // TODO: account.dispute(amount, disputed_transaction.r#type)
            if let Deposit = disputed_transaction.r#type {
                account.hold(amount);
            } else {
                account.release(amount);
            }
        }
        Resolve => {
            let disputed_transaction =
                get_disputed_transaction(&mut state.transactions, transaction.tx)?;
            disputed_transaction.disputed = false;
            let amount = disputed_transaction.amount()?;

            // TODO: account.resolve(amount, disputed_transaction.r#type)
            if let Deposit = disputed_transaction.r#type {
                account.release(amount);
            } else {
                account.hold(amount);
            }
        }
        Chargeback => {
            let disputed_transaction =
                get_disputed_transaction(&mut state.transactions, transaction.tx)?;
            let amount = disputed_transaction.amount()?;

            // TODO: account.chargeback(amount, disputed_transaction.r#type)
            if let Deposit = disputed_transaction.r#type {
                account.release(amount);
            } else {
                account.hold(amount);
            }

            match disputed_transaction.r#type {
                Deposit => account.force_withdraw(amount),
                Withdrawal => account.deposit(amount),
                _ => unreachable!(),
            }
            // end TODO

            account.locked = true;
        }
    }

    Ok(())
}
