#![no_main]

sp1_zkvm::entrypoint!(main);

use rollup_core::{Transaction, State};
use sp1_zkvm::io::{read, commit};

pub fn main() {
    let transactions: Vec<Transaction> = read();
    let mut state: State = read();
    for tx in transactions {
        let from_balance = state.balances.get(&tx.from).copied().unwrap_or(0);
        if from_balance >= tx.amount {
            state.balances.insert(tx.from, from_balance - tx.amount);
            *state.balances.entry(tx.to).or_insert(0) += tx.amount;
        }
    }
    commit(&state);
}