use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use crate::wallet::Wallet;
use crate::shashmap::Shashmap;

use crate::network::NetworkMessage;

use actix::*;

#[derive(Clone)]
pub struct Consensus {
    blockchain: Blockchain,
    mempool:    Mempool,
    pub wallet: Wallet,
    shashmap:   Shashmap,
}

// Consensus implements the Actor trait 
// which is given an actix Context containing its Addr 
impl Actor for Consensus {
    type Context = Context<Self>;
}

// Consensus implements a Handler trait which allows it to receive NetworkMessages

// It uses a match control flow which runs either add_block or add_transaction depending on the
// NetworkMessage payload
impl Handler<NetworkMessage> for Consensus {
    type Result = ();

    fn handle(&mut self, msg: NetworkMessage, _: &mut Context<Self>) {
        match msg {
            NetworkMessage::IncomingBlock(blk) => {
                self.blockchain.add_block(blk);
            },
            NetworkMessage::IncomingTransaction(tx) => {
                self.mempool.add_transaction(tx);
            },
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

