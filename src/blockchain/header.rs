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
}
