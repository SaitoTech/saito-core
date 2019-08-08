use std::{thread, time};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::burnfee::BurnFeeCalculator;

#[derive(Debug)]
pub struct Creator {
    blocks: Vec<Block>,
    transactions: Vec<Transaction>>,
    burn_fee_calc: BurnFeeCalculator,
    work: u64,
}

impl Creator {
    pub fn new() -> Creator {
        return Creator {
            blocks: vec![],
            transactions: vec![],
        };
    }

    pub fn add_transaction(&mut self, tx: Transaction) { 
        self.transactions.push(tx);
    }

    pub fn return_transaction_length(&self) -> u32 {
        return self.transactions.len() as u32;
    }

    pub fn clear_tx_mempool(&mut self) {
        self.transactions = vec![];
    }

    pub fn bundle(&self) {
        loop {
            // check how much work we have based on tx fees 
            if burn_fee_calc.return_current_burnfee() > 0 {
                return;
            } else {
                let one_second = time::Duration::from_millis(1000);
                thread::sleep(one_second);
                println!("FEE -- {:.8}", burnfee.return_current_burnfee());
            }
        }
    }
}
