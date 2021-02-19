use transactions::{process, Account, State, Transaction, TransactionType};

#[test]
fn resolve_deposit() {
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
            r#type: TransactionType::Resolve,
            amount: None,
            client: 1,
            tx: 1,
            disputed: false,
        },
    )
    .unwrap();

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

#[test]
fn resolve_withdrawal() {
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
            r#type: TransactionType::Withdrawal,
            amount: Some(10000),
            client: 1,
            tx: 2,
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
            tx: 2,
            disputed: false,
        },
    )
    .unwrap();
    process::process_one(
        &mut state,
        Transaction {
            r#type: TransactionType::Resolve,
            amount: None,
            client: 1,
            tx: 2,
            disputed: false,
        },
    )
    .unwrap();

    assert_eq!(
        state.accounts.get(&1).unwrap(),
        &Account {
            id: 1,
            available: 90000,
            held: 0,
            total: 90000,
            locked: false
        }
    );
}
