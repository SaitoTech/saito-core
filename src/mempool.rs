// use std::{thread, time};
use std::sync::RwLock;
use crate::wallet::Wallet;
use crate::config::GENESIS_PERIOD;

use saito_primitives::block::{Block, BlockHeader};
use saito_primitives::burnfee::BurnFee;
use saito_primitives::transaction::{Transaction, TransactionBroadcastType};
use saito_primitives::golden_ticket::GoldenTicket;
use saito_primitives::helper::create_timestamp;

#[derive(Debug, Clone)]
pub struct Mempool {
    blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    burnfee: BurnFee,
    work_available: u64,
}

impl Mempool {
    pub fn new() -> Mempool {
        return Mempool {
            blocks: vec![],
            transactions: vec![],
            burnfee: BurnFee::new(0.0, 0),
            work_available: 0,
        };
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn add_transaction(&mut self, tx: Transaction) { 
        self.work_available = tx.return_work_available("11413212312313321");
        self.transactions.push(tx.clone());
    }

    pub fn clear_transactions(&mut self) {
        self.transactions = vec![];
        self.work_available = 0;
    }

    //
    // TODO
    //
    // use blockchain data in RETURN_WORK_NEEDED call
    pub fn can_bundle_block (&mut self, block_header: Option<BlockHeader>) -> bool {
        match block_header {
            Some(block_header) => {
                let work_needed = BurnFee::return_work_needed(
                    block_header.ts,
                    create_timestamp(),
                    block_header.bf.start,
                );
                println!(
                    "TS: {} -- WORK ---- {:?} -- {:?} --- TX COUNT {:?}",
                    create_timestamp(), 
                    work_needed,
                    self.work_available,
                    self.transactions.len()
                );
                if work_needed <= self.work_available && self.transactions.len() > 0 {
                    return true;
                } else { return false; }
            }
            None => return true
        } 

    }

    pub fn bundle_block (&mut self, wallet: &RwLock<Wallet>, previous_block_header: Option<BlockHeader>) -> Block {
        let mut block: Block;
        let publickey = wallet.read().unwrap().return_publickey();

        let new_burnfee: BurnFee;

        self.clear_transactions();

        // set the majority of values if we have a previous block header
        match previous_block_header.clone() {
            Some(previous_block_header) => {
                block = Block::new(publickey,previous_block_header.bsh);

                let treasury = previous_block_header.treasury + previous_block_header.reclaimed;
                let coinbase = (treasury as f64 / GENESIS_PERIOD as f64).round() as u64;

                block.set_id(previous_block_header.bid + 1);
                block.set_mintid(previous_block_header.mintid);
                block.set_maxtid(previous_block_header.maxtid);
                block.set_coinbase(coinbase);
                block.set_treasury(treasury - coinbase);
                block.set_prevhash(previous_block_header.bsh);
                //block.set_difficulty(previous_block_header.difficulty);
                //block.set_paysplit(previous_block_header.paysplit);

                new_burnfee = BurnFee::adjust_work_needed(
                    previous_block_header,
                    block.body.ts,
                );
            },
            None => {
                block = Block::new(publickey, [0; 32]);
                new_burnfee = BurnFee::new(10.0, 0);
            }
        }

        // swap transactions with mempool
        block.set_transactions(&mut self.transactions);

        // set burnfee
        block.set_burnfee(new_burnfee);

        // calculate difficulty and paysplit
        match previous_block_header {
            Some(previous_block_header) => {
                // if there's no goldenticket, default to past values 
                let mut new_difficulty = previous_block_header.difficulty;
                let mut new_paysplit = previous_block_header.paysplit;

                for tx in block.body.txs.iter() {
                    if tx.return_tx_type() == TransactionBroadcastType::GoldenTicket {
                        let msg = &tx.body.msg;
                        let gt: GoldenTicket  = bincode::deserialize(&msg[..]).unwrap();
                        new_difficulty = gt.calculate_difficulty(previous_block_header.difficulty);
                        new_paysplit = gt.calculate_paysplit(previous_block_header.paysplit);
                    }
                }

                block.set_difficulty(new_difficulty);
                block.set_paysplit(new_paysplit);

            },
            None => {},
        }

        return block;
    }
}

