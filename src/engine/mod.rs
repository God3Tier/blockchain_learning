mod state_machine; 

use crate::{
    consensus_engine::forked::ForkedDigest,
    blockchain_features::{hash::HashStruct, block::Block},
    blockchain_features::transaction::Transaction,
    utxo_set::UtxoSet,
    engine::state_machine::StateMachine
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
    Reorg { removed: Vec<HashStruct>, added: Vec<Block<ForkedDigest>> },
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

pub struct BlockchainState {
    pub blocks: HashMap<HashStruct, Block<ForkedDigest>>,
}

impl StateMachine for BlockchainState {
    type State = UtxoSet; 
    type Transition = BlockchainTransition; 

    fn switch_state_next(starting_state: Self::State, t: &Self::Transition) -> Self::State {
        starting_state
    }
}