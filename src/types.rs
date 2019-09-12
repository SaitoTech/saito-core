use saito_primitives::block::Block;
use saito_primitives::transaction::Transaction;

use actix::*;

#[derive(Message)]
pub struct BlockMessage {
    pub payload: Block
}

impl BlockMessage {
    pub fn new(payload: Block) -> Self {
        return BlockMessage { payload } 
    }
}

#[derive(Message)]
pub struct TransactionMessage {
    pub payload: Transaction
}

impl TransactionMessage {
    pub fn new(payload: Transaction) -> Self {
        return TransactionMessage { payload } 
    }
}
