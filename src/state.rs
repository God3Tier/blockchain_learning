use crate::{
    Error,
    blockchain_features::{block::Block, hash::HashStruct},
    utxo_set::UtxoSet,
    consensus_engine::forked::ForkedDigest
};
use std::collections::HashMap;
use std::sync::Arc;
pub struct State {
    pub utxo_set: UtxoSet,
    pub block_map: HashMap<HashStruct, Arc<Block<ForkedDigest>>>,
    canonical_tip: HashStruct
}

impl State {
    pub fn genesis_state() -> Self {
        let mut block_map = HashMap::new(); 
        let genesis_block: Block<ForkedDigest> = Block::genesis();
        block_map.insert(genesis_block.get_hash(), genesis_block);
        
        Self {
            utxo_set: UtxoSet::initialise(), 
            block_map: HashMap::new(), 
            canonical_tip
        }
    }

    pub fn get_canoinical_chain(&self) -> Vec<HashStruct> {
        let mut chain = vec![];

        let mut current = self.canonical_tip.clone(); 

        loop {
            chain.push(current.clone())
            
        }
    }

    pub fn apply_block(&mut self, block: Arc<Block<ForkedDigest>>) -> Result<(), Error> {
        let transactions = &block.body.transactions;
        for trans in transactions {
            for txn_in in &trans.inputs {
                if let Some(unlocking_script) = &txn_in.unlocking_script {
                    
                    self
                        .utxo_set
                        .remove_utxo(&txn_in.utxo_id, unlocking_script)?;
                }
            }
    
            for (indx, txn_out) in trans.outputs.iter().enumerate() {
                self.utxo_set.add_utxo(
                    (indx as u32, trans.transaction_id.clone()),
                    txn_out.amount,
                    txn_out.locking_script.clone(),
                )?
            }
        }

        self.block_map.insert(block.header.hash(), block);
        
        Ok(())
    }
}