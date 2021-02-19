use transactions::{process, Account, State, Transaction, TransactionType};

#[test]
fn chargeback_deposit() {
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

#[test]
fn chargeback_withdrawal() {
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
            r#type: TransactionType::Chargeback,
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
            available: 100000,
            held: 0,
            total: 100000,
            locked: true
        }
    );
}
