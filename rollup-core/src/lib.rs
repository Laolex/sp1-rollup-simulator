use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub from: u64,
    pub to: u64,
    pub amount: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct State {
    pub balances: BTreeMap<u64, u64>,
}