use ed25519_dalek::{VerifyingKey, Verifier, Signature, Signer, SigningKey};
use rand_core::OsRng;

use crate::{
    consensus_engine::{Consensus}, 
    blockchain_features::header::Header
};

pub struct Poa {
     keys: Vec<VerifyingKey>, 
     my_signing_key: Option<SigningKey>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SlotDigest {
    slot: u64, 
    public_key: VerifyingKey,
    pub sig: Vec<u8>
}

impl super::Digest for SlotDigest {
    fn genesis() -> Self {
        let mut csprng = OsRng;
        let throwaway = SigningKey::generate(&mut csprng);
        SlotDigest {
            slot: 0,
            public_key: throwaway.verifying_key(), 
            sig: vec!()
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.slot.to_le_bytes());
        res.extend_from_slice(self.public_key.as_bytes());
        res.extend_from_slice(&self.sig);
        res
    }
}


impl Poa {
    pub fn contains_key(&self, key: &VerifyingKey) -> bool {
        self.keys.contains(&key)
    }
}

impl Consensus for Poa {
    type Digest = SlotDigest; 

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        let expected_slot = (parent_digest.slot + 1) % self.keys.len() as u64;
        if header.consensus_digest.slot != expected_slot {
            return false
        }

        let expected_key = self.keys[expected_slot as usize];
        if header.consensus_digest.public_key != expected_key {
            return false
        }

        let signature_bytes: Option<[u8; 64]> = header.consensus_digest.sig.as_slice().try_into().ok();    
        if signature_bytes.is_none(){
            return false
        }

        let signature = Signature::from(signature_bytes.unwrap());
        
        header.consensus_digest.public_key.verify(&header.signature(), &signature).is_ok()
    } 

    fn seal(&self, parent_digest: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        let bytes = partial_header.signature();
        let slot = (parent_digest.slot + 1) % self.keys.len() as u64;
        
        let expected_key = self.keys[slot as usize];
        if expected_key != self.my_signing_key.as_ref().unwrap().verifying_key() {
            return None
        }

        let sig: Vec<u8> = self.my_signing_key.as_ref().unwrap().sign(&bytes).to_bytes().into();
        
        let slot_digest = SlotDigest {
            slot, 
            public_key: self.keys[slot as usize],
            sig
        }; 
        
        Some(Header::new(partial_header.parent.clone(), partial_header.merkle_root, partial_header.state_root, slot_digest, partial_header.block_number))
    }
}

