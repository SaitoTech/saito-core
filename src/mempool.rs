use std::{thread, time};
use crate::wallet::Wallet;
use crate::blockchain::Blockchain;
use crate::blockchain::BlockHeader;
use saito_primitives::block::Block;
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
            burnfee: BurnFee::new(0.0, 0.0),
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
    pub fn can_bundle_block (&mut self, wallet: &Wallet, block_header: BlockHeader) -> bool {
	let ts = create_timestamp();
        let work_needed = self.burnfee.return_work_needed(block_header.ts, ts, block_header.bf, 100_000_000);
        if work_needed <= self.work_available {
	    return true;
        }
	return false;
    }

    pub fn bundle_block (&mut self, wallet: &Wallet, block_header: BlockHeader) -> Option<Block> {
        let mut block = Block::new(wallet.return_publickey(), block_header.prevbsh);
        block.set_transactions(&mut self.transactions);
        block.is_valid = 1;
	self.clear_transactions();
	return Some(block);
    }
}

