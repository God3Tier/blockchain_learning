
use ed25519_dalek::{VerifyingKey, Verifier, Signature, Signer, SigningKey}; 

use super::{poa::{Poa, SlotDigest}, grandpa::{GrandpaResult}, Consensus};
use crate::blockchain_features::header::Header;

pub struct Hybrid {
    pub babe: Poa,
    pub finality_threshold: f64
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct HybridDigest {
    pub babe_digest: SlotDigest, 
    pub grandpa: Option<GrandpaResult>
}

impl super::Digest for HybridDigest {
    fn genesis() -> Self {
        HybridDigest {
            babe_digest: SlotDigest::genesis(), 
            grandpa: None
        }
    }
    fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend_from_slice(&self.babe_digest.as_bytes());
        if let Some(grandpa) = &self.grandpa {
            res.extend_from_slice(&grandpa.as_bytes())
        }
        
        res
    }

}

impl Hybrid {
    fn validate_grandpa(&self, result: &GrandpaResult) -> bool {
        
        for vote in &result.votes {
            if !self.babe.contains_key(&vote.validator) {
                return false;
            }

            if vote.block_hash != result.resulting_hash {
                return false;
            }

            let vote_bytes = vote.block_hash.to_vec();
            let signature = Signature::from_slice(&vote.sig);
            
            if signature.is_err() {
                return false;
            }
            
            if vote.validator.verify(&vote_bytes, &signature.unwrap()).is_err() {
                return false;
            }
        }

        true
    }
}

impl Consensus for Hybrid {
    type Digest = HybridDigest; 

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        let temp_header = header.from_digest(header.consensus_digest.babe_digest.clone());
        if !self.babe.validate(&parent_digest.babe_digest, &temp_header) {
            return false
        }

        if let Some(result) = &header.consensus_digest.grandpa {
            return self.validate_grandpa(result)
        }
        
        false
    }
    
    fn seal(&self, parent_digest: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        let seal = self.babe.seal(&parent_digest.babe_digest, partial_header.clone())?;
        Some(Header::new(partial_header.parent.clone(), partial_header.merkle_root, partial_header.state_root, HybridDigest {babe_digest: seal.consensus_digest, grandpa: None} , partial_header.block_number))
    }
    
}