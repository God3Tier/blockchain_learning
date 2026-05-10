use std::sync::{
    Arc, 
    RwLock
};

use crate::{
    Error, 
    utxo_set::{
        UtxoSet, 
    }, 
    blockchain::{hash::HashStruct, scripts::{LockingScript, UnlockingScript}}
};

pub struct TxInput {
    utxo_id: (u32, u32),
    unlocking_script: UnlockingScript 
}

pub struct TxOutput {
    amount: i32,
    locking_script: LockingScript
}

pub struct Transaction {
    transaction_id: u32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}

impl Transaction {

    pub fn initialise(transaction_id: u32) -> Self {
        Transaction {
            transaction_id, 
            inputs: Vec::new(), 
            outputs: Vec::new()
        }
    }
    
    pub fn create_transaction(&mut self, utxo_set: Arc<RwLock<UtxoSet>>, amount: i32, dest_user_id: u32, unlocking_script: UnlockingScript) -> Result<(), Error> {
        if let Ok(set_borrow) = utxo_set.write() {
            let (vec, remainder) = set_borrow.get_valid_utxo_ids(&unlocking_script, amount);
            for utxo_id in vec {
                self.inputs.push(TxInput{utxo_id, unlocking_script: unlocking_script.clone()})
            }

            // self.outputs
            
            return Ok(())
        }
        
        Err("Unable to write to utxo_set".into())
    }

    pub fn get_hash(&self) -> HashStruct {
        todo!("Implement hash function for the transaction")
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            transaction_id: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
