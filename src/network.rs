use std::{thread, time};

use saito_primitives::slip::Slip;
use saito_primitives::block::Block;
use saito_primitives::crypto::PublicKey;
use saito_primitives::transaction::Transaction;

use crate::consensus::Consensus;

use actix::*;

//
// Network
//
pub struct Network {
    pub consensus_addr: Addr<Consensus>
}

//
// Implementing Actor trait for Network 
//
impl Actor for Network {
    type Context = Context<Network>;
} 

//
// Actix Message for sending data to Consensus
// 
#[derive(Message)]
pub enum NetworkMessage {
    IncomingBlock(Block),
    IncomingTransaction(Transaction),
}

#[derive(Debug)]
pub enum NetworkRequest {
    Block,
    Transaction
}

impl Network {
    pub fn init(&self, publickey: PublicKey) {
        let request = NetworkRequest::Block;
        loop {
            let three_seconds = time::Duration::from_millis(3000);
            thread::sleep(three_seconds);

            match request {
                NetworkRequest::Block => self.send_block_to_consensus(publickey),
                NetworkRequest::Transaction => self.send_transaction_to_consensus(publickey),
            }
        }
    }

    pub fn send_transaction_to_consensus(&self, publickey: PublicKey) {
        let mut tx: Transaction = Transaction::new();
        let mut slip: Slip = Slip::new(publickey);
        slip.set_amt(200_000_000);
        tx.add_from_slip(slip);
        self.consensus_addr.do_send(NetworkMessage::IncomingTransaction(tx));
    }

    pub fn send_block_to_consensus(&self, publickey: PublicKey) {
        let blk = Block::new(publickey, [1;32]);
        self.consensus_addr.do_send(NetworkMessage::IncomingBlock(blk));
    }

}


