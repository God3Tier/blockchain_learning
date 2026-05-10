#[derive(Clone)]
pub enum UnlockingScript {
    SimpleHash {
        pubkey: String, 
        signature: String
    }
}

impl UnlockingScript {
    pub fn verify(&self, hash: &LockingScript) -> bool {
        match self {
            UnlockingScript::SimpleHash{pubkey, signature} => {
                // TODO: actually implement the methods
                true
            }
        }
    }
}

// TODO: Add more verification method as see fit
#[derive(Clone)]
pub enum LockingScript {
    SimpleHash{ pubkey_hash: String }
}

impl LockingScript {
    pub fn create_hash(&self) -> String {
        // TODO: create the hash based on the policy requested by user
        "temporary".to_string()
    }
}