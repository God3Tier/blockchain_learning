use crate::{
    blockchain_features::{block::Block, hash::HashStruct},
    utxo_set::UtxoSet,
    consensus_engine::forked::ForkedDigest
};
use std::collections::HashMap;

pub struct State {
    pub utxo_set: UtxoSet,
    pub block_map: HashMap<HashStruct, Block<ForkedDigest>>,
}
