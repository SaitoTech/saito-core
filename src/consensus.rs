use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use crate::wallet::Wallet;
use crate::shashmap::Shashmap;

use crate::transaction::Transaction;

use std::sync::mpsc::Receiver;

pub struct Consensus {
    blockchain: Blockchain,
    mempool:    Mempool,
    pub wallet: Wallet,
    shashmap:   Shashmap,
}

impl Consensus {
    pub fn new() -> Consensus {
        return Consensus {
            blockchain: Blockchain::new(),
            mempool:    Mempool::new(),
            wallet:     Wallet::new(),
            shashmap:   Shashmap::new(),
        }
    }
        
    pub fn init(&mut self, tx_receiver: Receiver<Transaction>) {
        loop {
          let block = self.mempool.bundle_block(&self.wallet, &tx_receiver);
          self.blockchain.add_block(block);
        } 
    }
}


// need to include actix actor boilerplate for moving messages at a high level
