use std::{thread, time};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::burnfee::BurnFee;
use crate::wallet::Wallet;
use crate::helper::create_timestamp;


#[derive(Debug, Clone)]
pub struct Mempool {
    blocks: Vec<Block>,
    transactions: Vec<Transaction>,
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


    pub fn bundle_block (&mut self, wallet: &Wallet) -> Block {
        loop {
            // check how much work we have based on tx fees 
            //if let Ok(tx) = rx.try_recv() {
            //    self.add_transaction(tx);
            //}

	    let ts = create_timestamp();
            let work_needed = self.burnfee.return_work_needed(0, ts, 10_000_000_000, 100_000);

            if work_needed <= self.work_available {
		println!("WORK ADEQUATE: producing block");
                let mut block = Block::new(wallet.return_publickey());
                block.set_transactions(&mut self.transactions);
                //block_sender.send(block).unwrap();
                self.clear_transactions();
                return block;
            } else {
                let one_second = time::Duration::from_millis(1000);
                thread::sleep(one_second);
            }
        }
    }

}

