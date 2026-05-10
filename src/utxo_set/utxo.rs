use std::sync::Arc;

use crate::blockchain::scripts::{UnlockingScript, LockingScript};

pub struct Utxo {
    pub value: i32, 
    locking_key: LockingScript
}

impl Utxo {
    pub fn new(value: i32, locking_key: LockingScript) -> Arc<Self> {
        Arc::new(Utxo{value, locking_key})
    }

    pub fn verify_locking_string(&self, method_locking: &UnlockingScript) -> bool {
        method_locking.verify(&self.locking_key)
    }
} 
