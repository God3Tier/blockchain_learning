pub mod utxo; 

use std::{
    sync::Arc, 
    collections::HashMap
};
use crate::{
    utxo_set::utxo::Utxo, 
    blockchain_features::scripts::{UnlockingScript, LockingScript, KeyPair},
    blockchain_features::hash::HashStruct,
};
use crate::Error;
pub struct UtxoSet {
    set: HashMap<(u32, HashStruct), Arc<Utxo>>
}

impl UtxoSet {
    pub fn initialise() -> UtxoSet { 
        UtxoSet {
            set: HashMap::new()
        }
    }

    pub fn get_valid_utxo_ids(&self, key_pair: &KeyPair, amount: i32) -> Option<(Vec<(u32, HashStruct)>, i32, LockingScript)> {
        let mut res = Vec::new();
        let mut current_quantity = 0;
        let mut locking_script = LockingScript::default(); 
        for (utxo_id, utxo) in &self.set {
            if current_quantity >= amount {
                break;
            }
            if utxo.verify_locking_key(key_pair) {
                res.push(utxo_id.clone());
                current_quantity += utxo.value;
                locking_script = utxo.locking_key.clone();
            }
        }
        
        if current_quantity < amount {
            return None
        }

        Some((res, current_quantity - amount, locking_script))
    }

    pub fn remove_utxo(&mut self, utxo_id: &(u32, HashStruct), unlocking_script: &UnlockingScript) -> Result<(), Error> {
        if let Some(utxo) = self.set.get(utxo_id) {
            if !utxo.verify_locking_string(unlocking_script) {
                return Err("Invalid locking key method. Exiting now".into())
            }
            self.set.remove(utxo_id);
            return Ok(())
        }
        
        Err("utxo not found".into())
    }

    pub fn hash(&self) -> HashStruct {
        let mut res = HashStruct::default();
        for (key, val) in &self.set {
            let mut buf = vec!();
            buf.extend_from_slice(&key.0.to_le_bytes());
            buf.extend_from_slice(&key.1.as_bytes());
            let key_hash = HashStruct::generate_hash(String::from_utf8(buf).unwrap());
            let new_hash = HashStruct::rehash_from_2(&key_hash, &val.hash());
            res = HashStruct::rehash_from_2(&res, &new_hash);
        }
        res
    }
    
    pub fn add_utxo(&mut self, utxo_id: (u32, HashStruct), amount: i32, locking_script: LockingScript) -> Result<(), Error> { 
        self.set.insert(utxo_id, Utxo::new(amount, locking_script));
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialise_creates_empty_set() {
        let s = UtxoSet::initialise();
        assert!(s.set.is_empty());
    }

    #[test]
    fn add_and_remove_utxo_behaviour() {
        let mut s = UtxoSet::initialise();
        let id = (1u32, HashStruct::default());
        // add a default locking script
        let res = s.add_utxo(id.clone(), 10, LockingScript::Default);
        assert!(res.is_ok());
        // removing with UnlockingScript::Default should fail verification and return Err
        let rem = s.remove_utxo(&id, &UnlockingScript::Default);
        assert!(rem.is_err());
    }
}