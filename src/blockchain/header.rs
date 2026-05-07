use crate::blockchain::hash::HashStruct;

pub struct Header {
    pub parent: HashStruct, 
    pub merkle_root: HashStruct, 
    pub state_root: HashStruct,
    time_stamp: std::time::SystemTime, 
    
}