use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    pub id: u16,
    pub available: i64,
    pub held: i64,
    pub total: i64,
    pub frozen: bool,
}

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
}
