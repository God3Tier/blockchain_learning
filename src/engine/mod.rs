mod state_machine;

use crate::{
    Error,
    blockchain_features::transaction::Transaction,
    blockchain_features::{block::Block, hash::HashStruct, block_body::BlockBody},
    consensus_engine::forked::ForkedDigest,
    engine::state_machine::StateMachine,
    state::State,
};

use std::sync::Arc;

use std::collections::HashMap;

/// Requests used during initialisation flows.
pub enum BlockchainTransition {
    // ✅ ACCEPTED: Already validated by network layer
    ApplyTransaction(Transaction),      // Signature checked, UTXO existence pending
    AddBlock(Arc<Block<ForkedDigest>>),       // Header checked, merkle verified
    
    // ✅ ACCEPTED: Chain reorganization (reorg data pre-validated)
    Reorg {
        removed: Vec<HashStruct>,        // Block hashes already in our map
        added: Vec<Arc<Block<ForkedDigest>>>, // Blocks already validated
    },
    
    RestoreSnapshot(Vec<u8>),
    
    // ✅ ACCEPTED: Simple state changes
    Rollback(u64),
    Finalize(u64),
    Shutdown,
}
pub struct BlockchainStateMachine;


impl StateMachine for BlockchainStateMachine {
    type State = State;
    type Transition = BlockchainTransition;

    fn switch_state_next(
        mut starting_state: Self::State,
        t: Self::Transition,
    ) -> Result<Self::State, Error> {
        match t {
            BlockchainTransition::ApplyTransaction(trans) => {
                for txn_in in &trans.inputs {
                    if let Some(unlocking_script) = &txn_in.unlocking_script {
                        
                        starting_state
                            .utxo_set
                            .remove_utxo(&txn_in.utxo_id, unlocking_script)?;
                    }
                }

                for (indx, txn_out) in trans.outputs.iter().enumerate() {
                    starting_state.utxo_set.add_utxo(
                        (indx as u32, trans.transaction_id.clone()),
                        txn_out.amount,
                        txn_out.locking_script.clone(),
                    )?
                }
                 Ok(starting_state)
            }
            BlockchainTransition::AddBlock(block) => {
                let parent_hash = block.header.parent.clone();
                if !starting_state.block_map.contains_key(&parent_hash) {
                    return Err("parent block not found".into())
                }

                let computed_merkle = BlockBody::new(block.body.transactions.clone()).get_hash();
                if computed_merkle != block.header.merkle_root {
                   return Err("merkle root mismatch".into());
               }
               
                let transactions = &block.body.transactions;

                let mut end_state = starting_state;
                for trans in transactions {
                    end_state = BlockchainStateMachine::switch_state_next(end_state, BlockchainTransition::ApplyTransaction(trans.clone()))?;
                }
                end_state.block_map.insert(block.header.hash(), block);
                Ok(end_state)
            }
            BlockchainTransition::Reorg { removed, added } => {
                let mut state = State::genesis_state(); 

                for (hashstruct, block) in starting_state.block_map {
                    if !removed.contains(&hashstruct) {
                        state.apply_block(block)?;
                    }
                }

                for block in added {
                    state = BlockchainStateMachine::switch_state_next(state, BlockchainTransition::AddBlock(block))?;
                }
                
                Ok(state)
            }
            BlockchainTransition::RestoreSnapshot(vec) => {
                
                 Ok(starting_state)
            }
            BlockchainTransition::Rollback(val) => {
                 Ok(starting_state)
            }
            BlockchainTransition::Finalize(val) => {
                 Ok(starting_state)
            }
            BlockchainTransition::Shutdown => {
                
                Ok(starting_state)
            }
        }
        
    }
}
