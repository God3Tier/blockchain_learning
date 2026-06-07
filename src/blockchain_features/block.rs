use crate::{
    blockchain_features::hash::HashStruct,
    blockchain_features::transaction::Transaction,
    blockchain_features::{block_body::BlockBody, header::Header},
    consensus_engine::Digest, 
    state::State,
};

pub struct Block<D: Digest> {
    pub header: Header<D>,
    pub body: BlockBody,
}

impl<D: Digest> Block<D> {
    pub fn genesis() -> Self {
        let parent = HashStruct::default();
        let first_transaction = Transaction::default();
        // TODO: Make sure the state_root is the appropriate HashStruct rather than it's current placeholderqw
        Block {
            header: Header::new(parent, first_transaction.get_hash(), HashStruct::default(), D::genesis(), 0),
            body: BlockBody::new(vec![first_transaction]),
        }
    }

    // TODO: Actually put the state parameter
    pub fn child(&self, transactions: Vec<Transaction>, state: &State, consenses: D) -> Self {
        let block_body = BlockBody::new(transactions);
        let entrinsic_root = block_body.get_hash();
        let state_root = state.utxo_set.hash();
        
        Block { 
            header: self.header.child(entrinsic_root, state_root, consenses),
            body: block_body
        }
    }

    pub fn verify_sub_chain(&self, blocks: &[Block<D>]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn genesis_constructs_block_for_unit_digest() {
        // Ignored until Transaction::get_hash is implemented
        let _g: Block<()> = Block::genesis();
    }

    #[test]
    #[ignore]
    fn verify_sub_chain_empty_returns_true() {
        // Ignored until Transaction::get_hash is implemented
        let g: Block<()> = Block::genesis();
        assert!(g.verify_sub_chain(&[]));
    }
}
