pub mod utxo; 

use std::collections::HashSet;
use crate::utxo_set::utxo::Utxo;

pub struct UtxoSet {
    set: HashSet<Utxo>
}

impl UtxoSet {
    pub fn initialise() -> UtxoSet { 
        UtxoSet {
            set: HashSet::new()
        }
    }

    
    
}