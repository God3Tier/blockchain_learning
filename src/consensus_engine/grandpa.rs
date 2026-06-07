use ed25519_dalek::{VerifyingKey, Verifier, Signature, Signer, SigningKey}; 

use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct GrandpaVote {
    pub block_hash: [u8; 32], 
    pub validator: VerifyingKey, 
    pub sig: Vec<u8>
}

impl GrandpaVote {
    fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend_from_slice(&self.block_hash);
        res.extend_from_slice(self.validator.as_bytes());
        res.extend_from_slice(&self.sig);

        res
    }
    
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct GrandpaResult {
    pub votes: Vec<GrandpaVote>,
    pub resulting_hash: [u8; 32]
}

impl GrandpaResult {
    pub fn new(votes: Vec<GrandpaVote>, threshold: f64) -> Option<Self> {
        let mut store: HashMap<[u8; 32], u64> = HashMap::new();

        for vote in &votes {
            store.insert(vote.block_hash, store[&vote.block_hash] + 1);
        }

        let mut resulting_hash = None;

        for (hash, amount) in store {
            if amount as f64 / votes.len() as f64 >= threshold {
                resulting_hash = Some(hash)
            }
        }

        if resulting_hash.is_none() {
            return None; 
        }                
        
        Some(GrandpaResult {
            votes, 
            resulting_hash :resulting_hash.unwrap()
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new(); 
        
        for grandpa in &self.votes {
            res.extend_from_slice(&grandpa.as_bytes())
        }

        res.extend_from_slice(&self.resulting_hash);

        res
    }
}