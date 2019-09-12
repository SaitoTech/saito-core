// use std::{thread, time};
use crate::wallet::Wallet;
use crate::config::GENESIS_PERIOD;

use saito_primitives::block::{Block, BlockHeader};
use saito_primitives::burnfee::BurnFee;
use saito_primitives::transaction::Transaction;
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
                    block_header.bf,
                    100_000_000
                );
                println!(
                    "WORK ---- {:?} -- {:?} --- TX COUNT {:?}",
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

    pub fn bundle_block (&mut self, wallet: &Wallet, previous_block_header: Option<BlockHeader>) -> Block {
        let mut block: Block;
        let current_work: u64;

        match previous_block_header {
            Some(previous_block_header) => {
                block = Block::new(wallet.return_publickey(), previous_block_header.bsh);
                
                let treasury = previous_block_header.treasury + previous_block_header.reclaimed;
                let coinbase = (treasury as f64 / GENESIS_PERIOD as f64).round() as u64;

                block.set_id(previous_block_header.bid + 1);
                block.set_coinbase(coinbase);
                block.set_treasury(treasury - coinbase);
                block.set_prevhash(previous_block_header.bsh);
                block.set_difficulty(previous_block_header.difficulty);
                block.set_paysplit(previous_block_header.paysplit);

                current_work = BurnFee::return_work_needed(
                    previous_block_header.ts,
                    create_timestamp(),
                    previous_block_header.bf,
                    100_000_000
                );
            },
            None => {
                block = Block::new(wallet.return_publickey(), [0; 32]);
                current_work = 0;
            }
        }

        block.set_transactions(&mut self.transactions);
        block.set_burnfee(BurnFee::new(0.0, current_work));

	self.clear_transactions();
	return block;
    }
}

