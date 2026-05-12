use crate::{
    blockchain::hash::HashStruct,
    blockchain::transaction::Transaction,
    blockchain::{block_body::BlockBody, header::Header},
};

pub struct Block {
    header: Header,
    body: BlockBody,
}

impl Block {
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
    pub fn child(&self, transactions: Vec<Transaction>, state: u32) -> Self {
        let block_body = BlockBody::new(transactions);
        let entrinsic_root = block_body.get_hash();
        let state_root = HashStruct::generate_hash(state.to_string());
        
        Block {
            header: self.header.child(entrinsic_root, state_root),
            body: block_body
        }
    }    
}
