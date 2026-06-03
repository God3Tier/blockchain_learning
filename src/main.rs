mod account;
mod blockchain_features;
mod engine;
mod utxo_set;
mod consensus_engine; 
mod network_connection;
mod state;

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() {
    
    println!("Hello, world!");
}
