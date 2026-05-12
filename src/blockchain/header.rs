use crate::blockchain::hash::HashStruct;

pub struct Header {
    pub parent: HashStruct,
    pub merkle_root: HashStruct,
    pub state_root: HashStruct,
    time_stamp: std::time::SystemTime,
}

impl Header {
    pub fn new(parent: HashStruct, merkle_root: HashStruct, state_root: HashStruct) -> Self {
        Header {
            parent,
            merkle_root,
            state_root,
            time_stamp: std::time::SystemTime::now(),
        }
    }

    // This is a function to be called on a parent
    pub fn child(&self, entrinsic_root: HashStruct, state_root: HashStruct) -> Self {
        Header::new(self.parent.clone(), entrinsic_root, state_root)
    }

    pub fn verify_child(&self, child: &Header) -> bool {
        child.parent == self.hash()
    }

    pub fn hash(&self) -> HashStruct {
        let time_hash = HashStruct::generate_hash(format!("{:?}", self.time_stamp)); 
        HashStruct::rehash_from_2(&HashStruct::rehash_from_2(&self.parent, &self.merkle_root), &HashStruct::rehash_from_2(&self.state_root, &time_hash))
    }
}
