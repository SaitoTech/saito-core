use crate::transaction::Transaction;
use crate::slip::Slip;
use crate::crypto::PublicKey;

use std::{thread, time};
use std::sync::mpsc:: Sender;

// dummy class to emulate transactions coming over the network
pub struct Network {
    transaction_to_mempool_sender: Sender<Transaction>,
    publickey: PublicKey,
}

impl Network {
    pub fn new(sender: Sender<Transaction>, publickey: PublicKey) -> Network {
        return Network {
            transaction_to_mempool_sender: sender,
            publickey: publickey,
        };
    }

    pub fn init(&self) {
        loop {
            let mut transaction: Transaction = Transaction::new();
            let mut slip: Slip = Slip::new(self.publickey);

            slip.set_amt(200_000_000);
            transaction.add_from_slip(slip);

            let three_seconds = time::Duration::from_millis(3000);
            thread::sleep(three_seconds);

            self.transaction_to_mempool_sender.send(transaction).unwrap();
        }
    }
}
