mod account;
mod blockchain_features;
mod engine;
mod utxo_set;
mod consensus_engine; 

use crate::blockchain_features::hash::HashStruct;

pub type Error = Box<dyn std::error::Error>;

fn main() {
    println!("Hello, world!");
}
