use std::sync::Arc;
use crate::{engine::BlockchainState, consensus_engine::forked::Forked};

pub struct Node {
    pub state_machine: Arc<tokio::sync::RwLock<BlockchainState>>, 
    pub consensus_enigne: Arc<tokio::sync::RwLock<Forked>>, 
}

impl Node {
    pub fn new () -> Self {
        // Setup default node for the project. This will then start pinging other nodes
        todo!()
    }

    pub fn create_dummy_node() -> Self {

        // Setup node with default values in order to mock simuilation of base node
        todo!()
    }
    
    pub async fn run_server(&self) {
        
    }
}