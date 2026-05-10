mod account;
mod blockchain;
mod state_machine;
mod utxo_set;

use crate::blockchain::hash::HashStruct;

pub type Error = Box<dyn std::error::Error>;

fn main() {
    println!("Hello, world!");
}
