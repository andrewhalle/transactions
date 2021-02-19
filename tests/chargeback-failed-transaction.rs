use transactions::{
    account::AccountError,
    process::{self, TransactionProcessingError},
    Account, State, Transaction, TransactionType,
};

#[test]
fn chargeback_failed_transaction() {
    let mut state = State::new();

    process::process_one(
        &mut state,
        Transaction {
            r#type: TransactionType::Deposit,
            amount: Some(100000),
            client: 1,
            tx: 1,
            disputed: false,
        },
    )
    .unwrap();

    // this transaction fails because the account does not have enough balance
    assert_eq!(
        process::process_one(
            &mut state,
            Transaction {
                r#type: TransactionType::Withdrawal,
                amount: Some(1000000),
                client: 1,
                tx: 2,
                disputed: false,
            },
        ),
        Err(
            TransactionProcessingError::TransactionProcessingAccountError {
                source: AccountError::NotEnoughAvailable
            }
        )
    );

    // this needs to fail because the earlier transaction was not processed
    assert_eq!(
        process::process_one(
            &mut state,
            Transaction {
                r#type: TransactionType::Dispute,
                amount: None,
                client: 1,
                tx: 2,
                disputed: false,
            },
        ),
        Err(TransactionProcessingError::TransactionDoesNotExist)
    );

    assert_eq!(
        state.accounts.get(&1).unwrap(),
        &Account {
            id: 1,
            available: 100000,
            held: 0,
            total: 100000,
            locked: false
        }
    );
}
