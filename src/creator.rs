use std::{thread, time};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::burnfee::BurnFeeCalculator;
use crate::wallet::Wallet;

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

    pub fn return_transaction_length(&self) -> u32 {
        return self.transactions.len() as u32;
    }

    pub fn clear_tx_mempool(&mut self) {
        self.transactions = vec![];
        self.work = 0;
        self.burn_fee_calc.set_last_block_timestamp();
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
            let current_bf = self.burn_fee_calc.return_current_burnfee();

            if current_bf <= self.work {
                let mut block = Block::new(wallet.return_publickey());
                //for tx in self.transactions.iter_mut() {
                //    block.add_transaction(tx);
                //}
                block.set_transactions(&mut self.transactions);
                println!("SENDING BLOCK");
                block_sender.send(block).unwrap();
                
                self.clear_tx_mempool();
                return;
            } else {
                let one_second = time::Duration::from_millis(1000);
                thread::sleep(one_second);
                println!("{:.8} -- {:.8}", self.work, current_bf);
                println!("\n\n");
            }
        }
    }
}
