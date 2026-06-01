mod state_machine; 

use crate::{
    consensus_engine::forked::ForkedDigest,
    blockchain_features::{hash::HashStruct, block::Block},
    utxo_set::UtxoSet,
    engine::state_machine::StateMachine
}; 

use std::collections::HashMap;

enum InitialisationRequest {
    Full, 
    Snapshot,
    CommencingNode
}

pub enum BlockchainTransitionEnum {
    Init(InitialisationRequest)
}

pub struct BlockchainState {
    pub blocks: HashMap<HashStruct, Block<ForkedDigest>>,
}

impl StateMachine for BlockchainState {
    type State = UtxoSet; 
    type Transition = BlockchainTransitionEnum; 

    fn switch_state_next(starting_state: Self::State, t: &Self::Transition) -> Self::State {
        starting_state
    }
}