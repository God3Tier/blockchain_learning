pub struct HashStruct(pub [u8; 32]); 

impl HashStruct {
    fn generate_hash() -> HashStruct {
        HashStruct([0; 32])
    }
}