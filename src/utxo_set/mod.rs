pub mod utxo; 

use std::{
    sync::Arc, 
    collections::HashMap
};
use crate::{
    utxo_set::utxo::Utxo, 
    blockchain::scripts::{UnlockingScript, LockingScript}
};
use crate::Error;
pub struct UtxoSet {
    set: HashMap<(u32, u32), Arc<Utxo>>
}

impl UtxoSet {
    pub fn initialise() -> UtxoSet { 
        UtxoSet {
            set: HashMap::new()
        }
    }

    pub fn get_valid_utxo_ids(&self, unlocking_script: &UnlockingScript, amount: i32) -> (Vec<(u32, u32)>, i32) {
        let mut res = Vec::new();
        let mut current_quantity = 0;
        
        for (utxo_id, utxo) in &self.set {
            if current_quantity >= amount {
                break;
            }
            if utxo.verify_locking_string(&unlocking_script) {
                res.push(*utxo_id);
                current_quantity += utxo.value;
            }
        }

        (res, current_quantity - amount)
    }

    pub fn remove_utxo(&mut self, utxo_id: (u32, u32), unlocking_script: UnlockingScript) -> Result<(), Error> {
        if let Some(utxo) = self.set.get(&utxo_id) {
            if !utxo.verify_locking_string(&unlocking_script) {
                return Err("Invalid locking key method. Exiting now".into())
            }
            self.set.remove(&utxo_id);
            return Ok(())
        }
        
        Err("utxo not found".into())
    }

    pub fn add_utxo(&mut self, utxo_id: (u32, u32), amount: i32, locking_script: LockingScript) -> Result<(), Error> { 
        self.set.insert(utxo_id, Utxo::new(amount, locking_script));
        Ok(())
    }
    
}