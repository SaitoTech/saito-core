use std::{thread, time};
use std::cell::RefCell;
use std::sync::Arc;

use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use crate::wallet::Wallet;
use crate::shashmap::Shashmap;
use crate::network::NetworkMessage;
use crate::types::BlockMessage;
use crate::storage::Storage;

// use saito_primitives::block::Block;

use actix::*;

#[derive(Clone)]
pub struct Consensus {
    blockchain: Blockchain,
    mempool:    Mempool,
    pub wallet: Arc<Wallet>,
    shashmap:   Shashmap,
    pub lottery_addr: Recipient<BlockMessage>,
}

//
// Consensus implements the Actor trait 
//
impl Actor for Consensus {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }
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
                self.blockchain.add_block(blk, &mut self.shashmap);
            },
            NetworkMessage::IncomingTransaction(tx) => {
                self.mempool.add_transaction(tx);
                self.try_bundle();
            },
        }
    }
}

impl Consensus {
    pub fn new(wallet: Arc<Wallet>, lottery_addr: Recipient<BlockMessage>) -> Consensus {
        return Consensus {
            blockchain: Blockchain::new(),
            mempool:    Mempool::new(),
            wallet:     wallet,
            shashmap:   Shashmap::new(),
            lottery_addr
        }
    }

    pub fn heartbeat(&mut self, ctx: &mut Context<Self>) {
        ctx.run_later(time::Duration::from_millis(50), |act, ctx| {
            println!("IN HEARBEAT");
            act.try_bundle();
            act.heartbeat(ctx);
        });
    }

    pub fn try_bundle(&mut self) {
        println!("TRYING TO BUNDLE BLOCK");
        let last_block_header = self.blockchain.return_latest_block_header();

        // possibly pass by reference?
        if self.mempool.can_bundle_block(last_block_header.clone()) {
            let mut blk = self.mempool.bundle_block(&self.wallet, last_block_header);
            
            let reclaimed_funds = self.blockchain.calculate_reclaimed_funds(blk.header());
            blk.set_reclaimed(reclaimed_funds);
            
            println!("BLOCK : {:?}", blk);
            // need to add some control flow if a block isn't produced successfully
            self.blockchain.add_block(blk, &mut self.shashmap);

            // add slips to wallet after successfully producing block
            //blk.body.txs
            //    .iter() 
            //    .flat_map(|tx| tx.return_to_slips())
            //    .filter(|slip| slip.return_add() == self.wallet.return_publickey())
            //    .for_each(move |slip| self.wallet.add_slip(slip));

            let block_header = self.blockchain.return_latest_block_header();
            let block_message = BlockMessage::new(Storage::read_block_from_disk(block_header.unwrap().bsh));

            self.lottery_addr.do_send(block_message).unwrap();
        }

    }
}
