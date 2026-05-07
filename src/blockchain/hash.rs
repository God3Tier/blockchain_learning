use sha2::{Sha256, Digest};

#[derive(Default)]
pub struct HashStruct(pub [u8; 32]); 

impl HashStruct {
    pub fn generate_hash(input: String) -> Self {
        HashStruct(Sha256::digest(input).into())
    }

    pub fn from_hash(input: [u8; 32]) -> Self {
        HashStruct(input)
    }

    pub fn rehash_from_2(h1: &HashStruct, h2: &HashStruct) -> HashStruct {
        let string = String::from_utf8([h1.0, h2.0].concat()).unwrap(); 
        Self::generate_hash(string)
    }
}