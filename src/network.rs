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

impl Actor for Network {
    type Context = Context<Network>;
} 

impl Network {
    pub fn init(&self, publickey: PublicKey) {
        loop {
            let mut transaction: Transaction = Transaction::new();
            let mut slip: Slip = Slip::new(publickey);

            slip.set_amt(200_000_000);
            transaction.add_from_slip(slip);

            let three_seconds = time::Duration::from_millis(3000);
            thread::sleep(three_seconds);

            self.consensus_addr.send(transaction);
        }
    }

    pub fn send_transaction_to_consensus(&self, publickey: PublicKey) {
        let mut transaction: Transaction = Transaction::new();
        let mut slip: Slip = Slip::new(publickey);

        slip.set_amt(200_000_000);
        transaction.add_from_slip(slip);

        let three_seconds = time::Duration::from_millis(3000);
        thread::sleep(three_seconds);
        
        self.consensus_addr.send(transaction);
    }

    pub fn send_block_to_consensus(&self, publickey: PublicKey) {
        let blk = Block::new(publickey); 
        self.consens_addr.send(blk);
    }
}
