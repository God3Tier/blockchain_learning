pub mod pow;
pub mod dictator;
pub mod poa;
pub mod hybrid;
pub mod grandpa;

use crate::blockchain_features::{header::Header};

pub trait Consensus {
    type Digest: Clone + std::fmt::Debug + Eq + PartialEq + std::hash::Hash;

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool; 
    
    fn seal(&self, parent_digest: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>>; 
    
    fn verify_subchain(&self, parent_digest: &Self::Digest, chain: &[Header<Self::Digest>]) -> bool {

        let mut iterator = parent_digest;
        for header in chain {
            if !self.validate(iterator, header) {
                return false;
            }
            iterator = &header.consensus_digest;
        }
        
        true
    }
}