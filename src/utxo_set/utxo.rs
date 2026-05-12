use std::sync::Arc;

use crate::blockchain::scripts::{UnlockingScript, LockingScript, KeyPair};

pub struct Utxo {
    pub value: i32, 
    pub locking_key: LockingScript
}

impl Utxo {
    pub fn new(value: i32, locking_key: LockingScript) -> Arc<Self> {
        Arc::new(Utxo{value, locking_key})
    }

    pub fn verify_locking_key(&self, key_pair: &KeyPair) -> bool {
        match &self.locking_key {
            LockingScript::SimpleHash { pubkey_hash } => {
                &key_pair.pubkey_hash() == pubkey_hash
            }
            LockingScript::Default => false
        }
    }

    pub fn verify_locking_string(&self, method_locking: &UnlockingScript) -> bool {
        method_locking.verify(&self.locking_key)
    }
} 
