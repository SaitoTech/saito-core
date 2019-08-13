use crate::transaction::Transaction;
use crate::slip::Slip;
use crate::crypto::PublicKey;

use std::{thread, time};
use std::sync::mpsc:: Sender;

// dummy class to emulate transactions coming over the network
pub struct Network {}

impl Network {
    pub fn new() -> Network {
        return Network {};
    }

    pub fn init(&self, sender: Sender<Transaction>, publickey: PublicKey) {
        loop {
            let mut transaction: Transaction = Transaction::new();
            let mut slip: Slip = Slip::new(publickey);

            slip.set_amt(200_000_000);
            transaction.add_from_slip(slip);

            let three_seconds = time::Duration::from_millis(3000);
            thread::sleep(three_seconds);

            sender.send(transaction).unwrap();
        }
    }
}
