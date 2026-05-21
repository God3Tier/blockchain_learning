use super::{
    Consensus,
    Digest, 
    poa::{
        SlotDigest, 
        Poa
    },
    hybrid::{
        HybridDigest, 
        Hybrid}
};

use crate::blockchain_features::header::Header;

pub struct Forked { 
    fork_height: u64, 
    before: Poa, 
    after: Hybrid
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ForkedDigest {
    Before(SlotDigest), 
    After(HybridDigest)
}

impl Digest for ForkedDigest {
    fn genesis() -> Self {
        ForkedDigest::Before(SlotDigest::genesis())
    }
}

impl Consensus for Forked {       
    type Digest = ForkedDigest;
    
    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        if header.block_number > self.fork_height {
            match &header.consensus_digest {
                ForkedDigest::Before(slot) => {
                    let parent_digest = match parent_digest {
                        ForkedDigest::Before(slot) => slot, 
                        ForkedDigest::After(hybrid) => &hybrid.babe_digest
                    };
                    let header = header.from_digest(slot.clone());
                    self.before.validate(parent_digest, &header)
                }, 
                ForkedDigest::After(hybrid) => {
                    let parent_digest = match parent_digest {
                        ForkedDigest::Before(slot) => slot,
                        ForkedDigest::After(hybrid) => &hybrid.babe_digest
                    };
                    
                    let header = header.from_digest(hybrid.babe_digest.clone());
                    self.before.validate(parent_digest, &header)
                }
            }
        } else {
            match &header.consensus_digest {
                ForkedDigest::Before(slot) => {
                    let slot = HybridDigest {
                        babe_digest: slot.clone(), 
                        grandpa: None
                    }; 
                    let parent_digest = match parent_digest {
                        ForkedDigest::Before(slot) => {
                            &HybridDigest {
                                babe_digest: slot.clone(), 
                                grandpa: None
                            }
                        }, 
                        ForkedDigest::After(hybrid) => hybrid
                    };
                    let header = header.from_digest(slot);
                    self.after.validate(parent_digest, &header)
                }, 
                ForkedDigest::After(hybrid) => {
                    let parent_digest = match parent_digest {
                        ForkedDigest::Before(slot) => {
                            &HybridDigest {
                                babe_digest: slot.clone(), 
                                grandpa: None
                            }
                        },
                        ForkedDigest::After(hybrid) => hybrid
                    };
                    
                    let header = header.from_digest(hybrid.clone());
                    self.after.validate(parent_digest, &header)
                }
            }
        }        
    }
    
    fn seal(&self, parent_digest: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        if partial_header.block_number < self.fork_height {
            let parent_digest = match parent_digest {
                ForkedDigest::Before(slot) => {
                    slot
                }, 
                ForkedDigest::After(hybrid) => { 
                    &hybrid.babe_digest
                }
            };
            let sealed = self.before.seal(parent_digest, partial_header)?;
            Some(sealed.from_digest(ForkedDigest::Before(sealed.consensus_digest.clone())))
        } else {
            let parent_digest = match parent_digest {
                ForkedDigest::Before(slot) => {
                    &HybridDigest {
                        babe_digest: slot.clone(), 
                        grandpa: None
                    }
                }, 
                ForkedDigest::After(hybrid) => { 
                    hybrid
                }
            };
            let sealed = self.after.seal(parent_digest, partial_header)?;
            Some(sealed.from_digest(ForkedDigest::After(sealed.consensus_digest.clone())))
        }
    }
    
}