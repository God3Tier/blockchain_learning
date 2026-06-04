mod state_machine;

use crate::{
    Error,
    blockchain_features::transaction::Transaction,
    blockchain_features::{block::Block, hash::HashStruct},
    consensus_engine::forked::ForkedDigest,
    engine::state_machine::StateMachine,
    state::State,
};

use std::collections::HashMap;

/// Requests used during initialisation flows.
pub enum InitRequest {
    Full,
    Snapshot,
    CommencingNode,
}

/// The set of transitions the `StateMachine` can accept.
/// Add or remove variants as your consensus and runtime require.
pub enum BlockchainTransition {
    /// Initialise chain state (full reindex, snapshot restore, or starting node)
    Init(InitRequest),
    /// Apply a transaction to the mempool/state (may be validated first)
    ApplyTransaction(Transaction),
    /// Add a sealed block to the chain
    AddBlock(Block<ForkedDigest>),
    /// Validate a block without applying it (fork choice checks)
    ValidateBlock(Block<ForkedDigest>),
    /// Reorganise the chain: remove a range of blocks (by their parent hashes) and apply replacements
    Reorg {
        removed: Vec<HashStruct>,
        added: Vec<Block<ForkedDigest>>,
    },
    /// Create a serialized snapshot of the current state
    Snapshot,
    /// Restore state from a serialized snapshot blob
    RestoreSnapshot(Vec<u8>),
    /// Roll back N blocks
    Rollback(u64),
    /// Mark block number as finalized (consensus-dependent)
    Finalize(u64),
    /// Graceful shutdown of the state machine
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
            BlockchainTransition::Init(init_req) => match init_req {
                InitRequest::Full => {
                     Ok(starting_state)
                }
                InitRequest::Snapshot => {
                     Ok(starting_state)
                }
                InitRequest::CommencingNode => {
                     Ok(starting_state)
                }
            },
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
                let transactions = &block.body.transactions;

                let mut end_state = starting_state;
                for trans in transactions {
                    end_state = BlockchainStateMachine::switch_state_next(end_state, BlockchainTransition::ApplyTransaction(trans.clone()))?;
                }
                end_state.block_map.insert(block.header.hash(), block);
                Ok(end_state)
            }
            BlockchainTransition::ValidateBlock(block) => {
                
                Ok(starting_state)
            }
            BlockchainTransition::Reorg { removed, added } => {
                for hash in removed {
                    if let Some(block) = starting_state.block_map.remove(&hash) {
                        block.body.transactions.into_iter().for_each(|trans| {
                            trans.inputs.into_iter().for_each(|input| {
                                
                            });
                            trans.outputs.into_iter().for_each(|output| {
                                
                            })
                        })
                    }
                }

                
                Ok(starting_state)
            }
            BlockchainTransition::Snapshot => {
                 Ok(starting_state)
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
