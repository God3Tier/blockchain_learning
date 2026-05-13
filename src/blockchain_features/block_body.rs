use crate::blockchain_features::{hash::HashStruct, transaction::Transaction};

pub struct BlockBody {
    transactions: Vec<Transaction>,
}

impl BlockBody {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        BlockBody { transactions }
    }

    pub fn get_hash(&self) -> HashStruct {
        let mut store = Vec::new();

        for transaction in &self.transactions {
            store.push(transaction.get_hash());
        }

        if store.len() % 2 != 0 {
            store.push(self.transactions[self.transactions.len() - 1].get_hash());
        }

        while store.len() != 1 {
            let temp: Vec<HashStruct> = std::mem::take(&mut store);

            for i in 0..temp.len() / 2 {
                store.push(HashStruct::rehash_from_2(
                    &temp[i * 2],
                    &temp[i * 2 + 1],
                ))
            }
        }

        std::mem::take(&mut store[0])
    }
}
