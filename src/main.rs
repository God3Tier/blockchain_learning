mod state_machine; 
mod blockchain;
mod utxo_set; 
use crate::blockchain::hash::HashStruct;

fn main() {
    
    let temp = HashStruct::generate_hash("Temp".to_string());
        
    println!("Hello, world!");
}
