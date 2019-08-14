use crate::transaction::Transaction;
use crate::slip::Slip;
use crate::block::Block;
use crate::crypto::PublicKey;
use crate::consensus::Consensus;

use std::{thread, time};

use actix::*;

// dummy class to emulate transactions coming over the network
pub struct Network {
    pub consensus_addr: Addr<Consensus>
}

// Implementing Actor trait for Network 
impl Actor for Network {
    type Context = Context<Network>;
} 

// Creating a Actix Message struct type that we use to send messages
// to Consensus
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
        let blk = Block::new(publickey); 
        self.consensus_addr.do_send(NetworkMessage::IncomingBlock(blk));
    }
}
