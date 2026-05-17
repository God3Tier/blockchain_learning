use std::marker::PhantomData;

use super::{Consensus, Digest, poa::SlotDigest, hybrid::HybridDigest};

use crate::blockchain_features::header::Header;

pub struct Forked <Before, After> { 
    fork_height: u64, 
    phasa: PhantomData<(Before, After)>
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

impl <B, A> Consensus for Forked<B, A> 
    where 
        B: Consensus,
        A: Consensus,
        B: Into<ForkedDigest>,
        A: Into<ForkedDigest>
{       
    type Digest = ForkedDigest;
    
    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        false
    }
    
    fn seal(&self, parent_digest: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        None
    }
    
}