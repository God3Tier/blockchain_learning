use crate::blockchain_features::hash::HashStruct;
use crate::consensus_engine::Digest;
#[derive(Clone)]
pub struct Header<D: Digest> {
    pub parent: HashStruct,
    pub merkle_root: HashStruct,
    pub state_root: HashStruct,
    pub block_number: u64,
    time_stamp: std::time::SystemTime,
    pub consensus_digest: D, 
    pub nonce: u64
}

impl<D: Digest> Header<D> {
    
    pub fn from_digest<NewDigest: Digest>(&self, digest: NewDigest) -> Header<NewDigest> {
        Header {
            parent: self.parent.clone(), 
            merkle_root: self.merkle_root.clone(),
            state_root: self.state_root.clone(),
            block_number: self.block_number,
            time_stamp: std::time::SystemTime::now(),
            consensus_digest: digest,
            nonce: self.nonce
        }
    } 
    
    pub fn new(parent: HashStruct, merkle_root: HashStruct, state_root: HashStruct, consensus_digest: D, block_number: u64) -> Self {
        Header {
            parent,
            merkle_root,
            state_root,
            block_number,
            time_stamp: std::time::SystemTime::now(),
            consensus_digest,
            nonce: 1
        }
    }

    // This is a function to be called on a parent
    pub fn child(&self, entrinsic_root: HashStruct, state_root: HashStruct, consensus_digest: D) -> Self {
        Header::new(self.hash(), entrinsic_root, state_root, consensus_digest, self.block_number + 1)
    }

    pub fn verify_child(&self, child: &Header<D>) -> bool {
        child.parent == self.hash()
    }

    pub fn hash(&self) -> HashStruct {
        // Include all header fields that define this block's identity
        let mut buf = vec![];
            
         // Chain linkage
        buf.extend_from_slice(&self.parent.as_bytes());
        buf.extend_from_slice(&self.merkle_root.as_bytes());
        buf.extend_from_slice(&self.state_root.as_bytes());
            
        // Order and consensus
        buf.extend_from_slice(&self.block_number.to_le_bytes());
        buf.extend_from_slice(&self.nonce.to_le_bytes());
            
        // TODO: Include consensus_digest
        buf.extend_from_slice(&self.consensus_digest.as_bytes());
        // Create base hash
        let base_string = String::from_utf8(buf).unwrap();  // Or use hex encoding
        HashStruct::generate_hash(base_string)
    }

    pub fn signature(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.parent.as_bytes());
        bytes.extend_from_slice(&self.merkle_root.as_bytes());
        bytes.extend_from_slice(&self.block_number.to_le_bytes());
        bytes.extend_from_slice(&self.state_root.as_bytes());
        bytes
    }
}
