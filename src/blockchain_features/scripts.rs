use secp256k1::{SecretKey, PublicKey};
use crate::blockchain_features::hash::HashStruct;

pub struct KeyPair {
    pub private_key: SecretKey, 
    pub public_key: PublicKey,
}

impl KeyPair {
    pub fn pubkey_hash(&self) -> HashStruct {
        HashStruct::generate_hash(String::from_utf8(self.public_key.serialize().into_iter().collect::<Vec<u8>>()).unwrap())
    }
}

#[derive(Clone)]
pub enum UnlockingScript {
    SimpleHash {
        pubkey: PublicKey,
        signature: String
    },
    Default
}

impl UnlockingScript {
    pub fn verify(&self, hash: &LockingScript) -> bool {
        match self {
            UnlockingScript::SimpleHash{pubkey, signature} => {
                // TODO: actually implement the methods
                true
            }, 
            UnlockingScript::Default=> false, 
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            UnlockingScript::SimpleHash{pubkey, signature} => {
                signature.as_bytes().into_iter().chain(pubkey.serialize().iter()).map(|&a| a).collect::<Vec<u8>>()
            }
            UnlockingScript::Default => {
                vec!(0)
            }
        }
        
    }
}

// TODO: Add more verification method as see fit
#[derive(Clone, Default)]
pub enum LockingScript {
    SimpleHash{ pubkey_hash: HashStruct },
    #[default]
    Default
}

impl LockingScript {
    pub fn create_hash(&self) -> String {
        // TODO: create the hash based on the policy requested by user
        "temporary".to_string()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            LockingScript::SimpleHash{pubkey_hash} => pubkey_hash.0.into_iter().collect::<Vec<u8>>(), 
            LockingScript::Default => vec!(0) 
        }
        
    }
}