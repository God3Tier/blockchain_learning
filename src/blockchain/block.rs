use crate::{
    blockchain::hash::HashStruct,
    blockchain::transaction::Transaction,
    blockchain::{block_body::BlockBody, header::Header},
};

pub enum ActionType {}

pub struct Action {}

pub enum BlockState {
    Rejected,
    Processing,
    Completed,
}

pub struct Block {
    header: Header,
    body: BlockBody,
}

impl Block {
    fn genesis() -> Self {
        let parent = HashStruct::default();
        let first_transaction = Transaction::default();

        // TODO: Make sure the state_root is the appropriate HashStruct rather than it's current placeholderqw
        Block {
            header: Header::new(parent, first_transaction.get_hash(), HashStruct::default()),
            body: BlockBody::new(vec![first_transaction]),
        }
    }
}
