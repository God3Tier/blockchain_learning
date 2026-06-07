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
        signature: Vec<u8>
    },
    Default
}

impl UnlockingScript {
    pub fn verify(&self, lock: &LockingScript) -> bool {
        match self {
            UnlockingScript::SimpleHash{pubkey, signature} => {
                // TODO: actually implement the method
                let valid_signature =Signature::from_slice(signature); 
                if valid_signature.is_err() {
                    return false;
                }

                if let LockingScript::SimpleHash{pubkey_hash} = lock {
                    let res = pubkey.verify(&pubkey_hash.as_bytes(),  &valid_signature.unwrap()); 
                    return res.is_ok()
                    
                }
                false
            }, 
            UnlockingScript::Default=> false, 
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            UnlockingScript::SimpleHash{pubkey, signature} => {
                signature.iter().chain(pubkey.as_bytes().iter()).cloned().collect::<Vec<u8>>()
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
    pub fn create_hash(&self) -> HashStruct {
        // TODO: create the hash based on the policy requested by user
        match self {
            LockingScript::SimpleHash{pubkey_hash} => {
                // TODO: actually implement the method
                let mut msg = vec!(); 
                msg.extend_from_slice(&pubkey_hash.as_bytes());
                HashStruct::generate_hash(String::from_utf8(msg).unwrap())
            }, 
            LockingScript::Default=> HashStruct::default(), 
        }
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
        
    }
}