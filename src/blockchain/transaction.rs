use crate::blockchain::hash::HashStruct;

pub struct TxInput {
    
}

pub struct TxOutput {
    
}

pub struct Transaction {
    transaction_id: u32, 
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>
}

impl Transaction {
    pub fn get_hash(&self) -> HashStruct {
        todo!("Implement hash function for the transaction")
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            transaction_id: 1, 
            inputs: Vec::new(),
            outputs: Vec::new()        
        }
    }
}


