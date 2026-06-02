use std::sync::Arc;
use crate::{engine::BlockchainStateMachine, consensus_engine::forked::Forked};

pub struct Node {
    pub state_machine: Arc<tokio::sync::RwLock<BlockchainStateMachine>>, 
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
    #[ignore]
    fn new_is_unimplemented() {
        // Ignored until `Node::new` is implemented
        let _ = Node::new();
    }

    #[test]
    #[ignore]
    fn create_dummy_node_is_unimplemented() {
        // Ignored until `Node::create_dummy_node` is implemented
        let _ = Node::create_dummy_node();
    }

    // Concurrency specifications for future state-machine integration.
    // These tests are ignored now and act as a blueprint for how the real
    // network code should behave when multiple peers mutate the shared state.
    #[tokio::test]
    #[ignore]
    async fn concurrent_state_mutations_spec() {
        use std::sync::Arc;
        use tokio::sync::RwLock;

        // Mock shared state: a simple counter representing applied transitions.
        // Replace `Arc<RwLock<u32>>` with `Arc<RwLock<BlockchainState>>` in real tests.
        let state = Arc::new(RwLock::new(0u32));
        let mut handles = Vec::new();

        // Spawn many concurrent writers (simulating concurrent incoming actions)
        for _ in 0..200usize {
            let s = state.clone();
            handles.push(tokio::spawn(async move {
                // In a real implementation this would be a call to the state machine
                // applying a `Transition` under a write lock.
                let mut w = s.write().await;
                *w += 1;
            }));
        }

        for h in handles { h.await.unwrap(); }

        // Final state should reflect all applied transitions
        let final_value = *state.read().await;
        assert_eq!(final_value, 200u32);
    }

    #[tokio::test]
    #[ignore]
    async fn concurrent_read_write_mixture_spec() {
        use std::sync::Arc;
        use tokio::sync::RwLock;

        // Mock shared state
        let state = Arc::new(RwLock::new(0u32));

        // Spawn writers
        let mut writers = Vec::new();
        for _ in 0..50usize {
            let s = state.clone();
            writers.push(tokio::spawn(async move {
                let mut w = s.write().await;
                *w += 2; // simulate larger mutation
            }));
        }

        // Spawn readers that sample the state concurrently
        let mut readers = Vec::new();
        for _ in 0..100usize {
            let s = state.clone();
            readers.push(tokio::spawn(async move {
                let r = s.read().await;
                // readers should never observe a corrupted value; they should see a u32
                *r
            }));
        }

        // Wait for all writers then readers
        for w in writers { w.await.unwrap(); }

        let mut samples = Vec::new();
        for r in readers { samples.push(r.await.unwrap()); }

        // Ensure all reads succeeded and are within an expected range
        for v in samples {
            assert!(v <= 100u32); // readers may observe values up to writers' cumulative effect
        }
    }
}