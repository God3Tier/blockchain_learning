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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn new_is_unimplemented() {
        // Node::new currently calls todo!(), which should panic
        let _ = Node::new();
    }

    #[test]
    #[should_panic]
    fn create_dummy_node_is_unimplemented() {
        let _ = Node::create_dummy_node();
    }
}