use sha2::{Digest, Sha256};

#[derive(Debug, Default, Clone, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_hash_is_deterministic() {
        let a = HashStruct::generate_hash("hello".to_string());
        let b = HashStruct::generate_hash("hello".to_string());
        assert_eq!(a, b);
    }

    #[test]
    fn from_hash_and_as_bytes_roundtrip() {
        let arr = [1u8; 32];
        let h = HashStruct::from_hash(arr);
        assert_eq!(h.as_bytes(), arr);
        assert_eq!(h, HashStruct::from_hash(arr));
    }

    #[test]
    #[should_panic]
    fn display_panics_for_non_utf8_bytes() {
        // The Display impl attempts to convert raw hash bytes to UTF-8 and will unwrap(), causing panic
        let h = HashStruct::generate_hash("test".to_string());
        let _ = format!("{}", h);
    }

    #[test]
    #[should_panic]
    fn rehash_from_2_panics_on_raw_bytes_conversion() {
        let h1 = HashStruct::generate_hash("a".to_string());
        let h2 = HashStruct::generate_hash("b".to_string());
        let _ = HashStruct::rehash_from_2(&h1, &h2);
    }
}