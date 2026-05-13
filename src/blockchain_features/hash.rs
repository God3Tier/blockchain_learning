use sha2::{Digest, Sha256};

#[derive(Default, Clone, Eq)]
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

    pub fn as_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl std::fmt::Display for HashStruct{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.into_iter().collect::<Vec<u8>>()).unwrap())
    }
}

impl PartialEq for HashStruct {
    fn eq(&self, other: &Self) -> bool {

        if self.0.len() != other.0.len() {
            return false;
        }
        
        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
} 