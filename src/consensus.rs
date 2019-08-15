use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use crate::wallet::Wallet;
use crate::shashmap::Shashmap;
use crate::network::NetworkMessage;

use saito_primitives::block::Block;

use actix::*;

#[derive(Clone)]
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
        
    pub fn init(&mut self ) {
        loop {

            if self.mempool.can_bundle_block(&self.wallet, &self.blockchain) {

                let blk = self.mempool.bundle_block(&self.wallet);
		match blk {
		    Some(blk) => {
                        self.blockchain.add_block(blk);
		    },
		    None => {
		    },
		}	

	    }
        } 

    }
}



//
// Consensus implements the Actor trait 
//
impl Actor for Consensus {
    type Context = Context<Self>;
}

//
// Actix Handlers
//
// NETWORK MESSAGES
//
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


