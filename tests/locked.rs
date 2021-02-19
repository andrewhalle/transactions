use transactions::{process, Account, State, Transaction, TransactionType};

#[test]
fn locked_account_rejects_future_activity() {
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
    process::process_one(
        &mut state,
        Transaction {
            r#type: TransactionType::Dispute,
            amount: None,
            client: 1,
            tx: 1,
            disputed: false,
        },
    )
    .unwrap();
    process::process_one(
        &mut state,
        Transaction {
            r#type: TransactionType::Chargeback,
            amount: None,
            client: 1,
            tx: 1,
            disputed: false,
        },
    )
    .unwrap();

    assert!(process::process_one(
        &mut state,
        Transaction {
            r#type: TransactionType::Deposit,
            amount: Some(100000),
            client: 1,
            tx: 1,
            disputed: false,
        },
    )
    .is_err());

    assert_eq!(
        state.accounts.get(&1).unwrap(),
        &Account {
            id: 1,
            available: 0,
            held: 0,
            total: 0,
            locked: true
        }
    );
}
