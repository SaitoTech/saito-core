use std::{thread, time};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::burnfee::BurnFeeCalculator;
use crate::wallet::Wallet;
use crate::helper::create_timestamp;

use std::sync::mpsc::{Sender, Receiver};

#[derive(Debug)]
pub struct Creator {
    blocks: Vec<Block>,
    transactions: Vec<Transaction>,
    burn_fee_calc: BurnFeeCalculator,
    work: u64,
}

impl Creator {

    pub fn new() -> Creator {
        return Creator {
            blocks: vec![],
            transactions: vec![],
            burn_fee_calc: BurnFeeCalculator::new(),
            work: 0,
        };
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn add_transaction(&mut self, tx: Transaction) { 
        for slip in tx.return_from_slips() {
            self.work += slip.return_amt();
        }
        self.transactions.push(tx.clone());
    }

    pub fn clear_tx_mempool(&mut self) {
        self.transactions = vec![];
        self.work = 0;
    }

    pub fn bundle(
        &mut self, 
        wallet: &Wallet,
        rx: &Receiver<Transaction>,
        block_sender: &Sender<Block>
    ) {
        loop {
            // check how much work we have based on tx fees 
            if let Ok(tx) = rx.try_recv() {
                self.add_transaction(tx);
            }

	    let ts = create_timestamp();
            let work_needed = self.burn_fee_calc.return_work_needed(0, ts, 10_000_000_000);
println!("WORK NEEDED: {:?}", work_needed);

            if work_needed <= self.work {
                let mut block = Block::new(wallet.return_publickey());
                block.set_transactions(&mut self.transactions);
                println!("SENDING BLOCK");
                block_sender.send(block).unwrap();
                self.clear_tx_mempool();
                return;
            } else {
                let one_second = time::Duration::from_millis(1000);
                thread::sleep(one_second);
                //println!("{:.8} -- {:.8}", self.work, current_bf);
                //println!("\n\n");
            }
        }
    }
}

