use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Signature, Verifier};
use crate::blockchain_features::hash::HashStruct;

pub struct KeyPair {
    private_key: SigningKey, 
    pub public_key: VerifyingKey,
}

impl KeyPair {
    pub fn pubkey_hash(&self) -> HashStruct {
        HashStruct::generate_hash(String::from_utf8(self.public_key.as_bytes().into_iter().map(|&a| a).collect::<Vec<u8>>()).unwrap())
    }

    pub fn sign(&self, msg: &[u8]) -> String {
        self.private_key.sign(msg).to_string()
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.public_key.verify(message, signature).is_ok()
    }
}

#[derive(Clone)]
pub enum UnlockingScript {
    SimpleHash {
        pubkey: VerifyingKey,
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
                signature.as_bytes().into_iter().chain(pubkey.as_bytes().iter()).map(|&a| a).collect::<Vec<u8>>()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locking_script_default_as_bytes() {
        let ls = LockingScript::Default;
        assert_eq!(ls.as_bytes(), vec!(0));
    }

    #[test]
    fn unlocking_script_default_behaviour() {
        let us = UnlockingScript::Default;
        let ls = LockingScript::Default;
        assert!(!us.verify(&ls));
        assert_eq!(us.as_bytes(), vec!(0));
    }
}