use std::sync::{
    Arc, 
    RwLock
};

use crate::{
    Error, 
    utxo_set::{
        UtxoSet, 
    }, 
    blockchain_features::{hash::HashStruct, scripts::{LockingScript, UnlockingScript, KeyPair}}
};

#[derive(Clone)]
pub struct TxInput {
    pub utxo_id: (u32, HashStruct),
    pub unlocking_script: Option<UnlockingScript>, 
}

#[derive(Clone)]
pub struct TxOutput {
    pub amount: i32,
    pub locking_script: LockingScript
}

#[derive(Default, Clone)]
pub struct Transaction {
    pub transaction_id: HashStruct,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {

    fn new(inputs: &mut Vec<TxInput>, outputs: &mut Vec<TxOutput>, user_key: KeyPair) -> Self {
        let mut buf = vec![];

        for input in inputs.iter_mut() {
            buf.extend_from_slice(&input.utxo_id.0.to_le_bytes());
            buf.extend_from_slice(&input.utxo_id.1.as_bytes());
        }

        for output in outputs.iter_mut() {
            buf.extend_from_slice(&output.amount.to_le_bytes());
            buf.extend_from_slice(&output.locking_script.as_bytes());
        }

        let transaction_id = HashStruct::generate_hash(HashStruct::generate_hash(String::from_utf8(buf).unwrap()).to_string());
        
        for input in inputs.iter_mut() {
            input.unlocking_script = Some(UnlockingScript::SimpleHash{
                pubkey: user_key.public_key, 
                signature: user_key.sign(&transaction_id.as_bytes())
            })
        }
        
        Transaction {
            transaction_id, 
            inputs: std::mem::take(inputs), 
            outputs: std::mem::take(outputs)
        }
    }
    
    pub fn create_transaction(utxo_set: Arc<RwLock<UtxoSet>>, amount: i32, receiving_locking_script: LockingScript, user_key: KeyPair) -> Result<Self, Error> {
        if let Ok (set_borrow) = utxo_set.read() {
            if let Some((vec, remainder, locking_script)) = set_borrow.get_valid_utxo_ids(&user_key, amount) {
                let mut inputs: Vec<TxInput> = vec.into_iter().map(|utxo_id| TxInput{utxo_id, unlocking_script: None}).collect();
                let mut outputs = vec!(TxOutput{amount, locking_script: receiving_locking_script});                if remainder > 0 {
                    outputs.push(TxOutput{amount: remainder, locking_script})
                }
                return Ok(Transaction::new(
                    &mut inputs,
                    &mut outputs, 
                    user_key
                ))
            } else {
                return Err("invalid amount of funds for transaction".into())
            }
        }
        
        Err("Unable to write to utxo_set".into())
    }
    
    pub fn get_hash(&self) -> HashStruct {
        self.transaction_id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn transaction_get_hash_is_unimplemented() {
        // Ignored until Transaction::get_hash is implemented
        let tx = Transaction::default();
        let _ = tx.get_hash();
    }
}
