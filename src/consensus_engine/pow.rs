use sha2::{Digest, Sha256};

use crate::{
    consensus_engine::Consensus, 
    blockchain_features::header::Header
}; 

pub struct Pow {
     threshold: u64
}

impl super::Digest for u64 {
    fn genesis() -> Self {
        0
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into_iter().collect()
    }
}

impl Consensus for Pow {
    type Digest = u64; 

    fn validate(&self, _: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        return header.consensus_digest > self.threshold; 
    }

    fn seal(&self, _: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        let mut nonce = partial_header.nonce; 

        let mut hash: [u8; 32] = Sha256::digest(nonce.to_string()).into();
        let mut starts_with = Vec::new();
        for _ in 0..self.threshold {
            starts_with.push(0);
        }
        
        while hash[0..self.threshold as usize] == starts_with {
            nonce += 1;
            hash = Sha256::digest(nonce.to_string()).into();
        }

        return Some(Header::new(partial_header.parent.clone(), partial_header.merkle_root, partial_header.state_root, nonce, partial_header.block_number))
    }

    
}