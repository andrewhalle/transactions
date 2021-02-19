use transactions::{process, Account, State, Transaction, TransactionType};

#[test]
fn dispute_deposit() {
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

    assert_eq!(
        state.accounts.get(&1).unwrap(),
        &Account {
            id: 1,
            available: 0,
            held: 100000,
            total: 100000,
            locked: false
        }
    );
}

#[test]
fn dispute_withdrawal() {
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

    assert_eq!(
        state.accounts.get(&1).unwrap(),
        &Account {
            id: 1,
            available: 100000,
            held: -10000,
            total: 90000,
            locked: false
        }
    );
}
