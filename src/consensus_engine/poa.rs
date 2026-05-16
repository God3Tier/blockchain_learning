use ed25519_dalek::{VerifyingKey, Verifier, Signature, Signer, SigningKey};
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

impl Poa {
    pub fn contains_key(&self, key: &VerifyingKey) -> bool {
        self.keys.contains(&key)
    }
}

impl Consensus for Poa {
    type Digest = SlotDigest; 

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        if parent_digest.slot > header.consensus_digest.slot {
            return false
        }

        if self.keys[header.consensus_digest.slot as usize % self.keys.len()] != header.consensus_digest.public_key {
            return false
        }

        let signature_bytes: Option<[u8; 64]> = match header.consensus_digest.sig.as_slice().try_into() {
            Ok(bytes) => Some(bytes),
            Err(_) => None
        };

        if signature_bytes.is_none(){
            return false
        }

        let signature = Signature::from(signature_bytes.unwrap());
        
        header.consensus_digest.public_key.verify(&header.signature(), &signature).is_ok()
    } 

    fn seal(&self, parent_digest: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        let bytes = partial_header.signature();
        let slot = (parent_digest.slot + 1) % self.keys.len() as u64;
        
        if self.my_signing_key.is_none() {
            return None
        }

        let sig: Vec<u8> = self.my_signing_key.as_ref().unwrap().sign(&bytes).to_string().as_bytes().into();
        
        let slot_digest = SlotDigest {
            slot, 
            public_key: self.keys[slot as usize],
            sig
        }; 
        
        return Some(Header::new(partial_header.parent.clone(), partial_header.merkle_root, partial_header.state_root, slot_digest, partial_header.block_number))
    }
}

