use crate::blockchain_features::hash::HashStruct;

#[derive(Clone)]
pub struct Header<Digest> {
    pub parent: HashStruct,
    pub merkle_root: HashStruct,
    pub state_root: HashStruct,
    pub block_number: u64,
    time_stamp: std::time::SystemTime,
    pub consensus_digest: Digest, 
    pub nonce: u64
}

impl<Digest> Header<Digest> {
    
    pub fn from_digest<NewDigest>(&self, digest: NewDigest) -> Header<NewDigest> {
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
    
    pub fn new(parent: HashStruct, merkle_root: HashStruct, state_root: HashStruct, consensus_digest: Digest, block_number: u64) -> Self {
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
    pub fn child(&self, entrinsic_root: HashStruct, state_root: HashStruct, consensus_digest: Digest) -> Self {
        Header::new(self.hash(), entrinsic_root, state_root, consensus_digest, self.block_number + 1)
    }

    pub fn verify_child(&self, child: &Header<Digest>) -> bool {
        child.parent == self.hash()
    }

    pub fn hash(&self) -> HashStruct {
        let time_hash = HashStruct::generate_hash(format!("{:?}", self.time_stamp)); 
        HashStruct::rehash_from_2(&HashStruct::rehash_from_2(&self.parent, &self.merkle_root), &HashStruct::rehash_from_2(&self.state_root, &time_hash))
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
