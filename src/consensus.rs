use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use crate::wallet::Wallet;
use crate::shashmap::Shashmap;

use crate::transaction::Transaction;
use crate::block::Block;

use actix::*;

#[derive(Clone)]
pub struct Consensus {
    blockchain: Blockchain,
    mempool:    Mempool,
    pub wallet: Wallet,
    shashmap:   Shashmap,
}

impl Actor for Consensus {
    type Context = Context<Self>;
}

impl Handler<Transaction> for Consensus {
    type Result = ();
    
    fn handle(&mut self, tx: Transaction, _: &mut Context<Self>) {
        self.mempool.add_transaction(tx);
    }
}

impl Handler<Block> for Consensus {
    type Result = ();

    fn handle(&mut self, blk: Block, _: &mut Context<Self>) {
        self.blockchain.add_block(blk);
    }
}

impl Default for Consensus {
    fn default() -> Consensus {
        return Consensus {
            blockchain: Blockchain::new(),
            mempool:    Mempool::new(),
            wallet:     Wallet::new(),
            shashmap:   Shashmap::new(),
        }
    }
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
        
    pub fn init(&mut self ) {
        loop {
          let block = self.mempool.bundle_block(&self.wallet);
          self.blockchain.add_block(block);
        } 
    }
}


// need to include actix actor boilerplate for moving messages at a high level
