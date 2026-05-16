use crate::{
    blockchain_features::hash::HashStruct,
    blockchain_features::transaction::Transaction,
    blockchain_features::{block_body::BlockBody, header::Header},
};

pub struct Block<Digest> {
    header: Header<Digest>,
    body: BlockBody,
}

impl<Digest> Block<Digest> {
    pub fn genesis() -> Self {
        let parent = HashStruct::default();
        let first_transaction = Transaction::default();
        // TODO: Make sure the state_root is the appropriate HashStruct rather than it's current placeholderqw
        Block {
            header: Header::new(parent, first_transaction.get_hash(), HashStruct::default()),
            body: BlockBody::new(vec![first_transaction]),
        }
    }

    // TODO: Actually put the state parameter
    pub fn child(&self, transactions: Vec<Transaction>, state: u32, consenses: Digest) -> Self {
        let block_body = BlockBody::new(transactions);
        let entrinsic_root = block_body.get_hash();
        let state_root = HashStruct::generate_hash(state.to_string());
        
        Block { 
            header: self.header.child(entrinsic_root, state_root, consenses),
            body: block_body
        }
    }

    pub fn verify_sub_chain(&self, blocks: &[Block<Digest>]) -> bool {
        if blocks.is_empty() {
            return true
        }

        let mut iterator = self;

        for chain in blocks {
            if chain.header.parent != iterator.header.hash() {
                return false; 
            }

            // TODO: You need to also verify the block 
            
            iterator = chain; 
        }

        
        true
    }
}
