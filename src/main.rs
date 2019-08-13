use saito_core::block::Block;
use saito_core::transaction::{Transaction};
use saito_core::slip::Slip;

use saito_core::blockchain::Blockchain;
use saito_core::creator::Creator;
use saito_core::wallet::Wallet;
use saito_core::shashmap::Shashmap;

use std::cell::RefCell;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::{thread, time};



struct Client {

    blockchain: Blockchain,
    creator:    Creator,
    wallet:     Wallet,
    shashmap:   Shashmap,



}

impl Client {

    pub fn new() -> Client{
        return Client {
            blockchain: Blockchain::new(),
            creator:    Creator::new(),
            wallet:     Wallet::new(),
            shashmap:   Shashmap::new(),
        }
    }
        
    pub fn init(
        &mut self,
        tx_receiver: Receiver<Transaction>,
        block_sender: Sender<Block>,
        block_receiver: Receiver<Block>
    ) {

        loop {
          self.creator.bundle(&self.wallet, &tx_receiver, &block_sender);
          let block = block_receiver.recv().unwrap();
          self.add_block(block);
        } 
    }

    pub fn add_block(&mut self, block: Block) {
        self.blockchain.add_block(block);
    }

    pub fn validate_tx(&self, tx: Transaction, tx_sender: Sender<Transaction>) {
        if true {
            tx_sender.send(tx).unwrap();
        }  
    }
}



fn main() {

    let (tx_sender, tx_receiver): (Sender<Transaction>, Receiver<Transaction>) = channel();
    let (block_sender, block_receiver): (Sender<Block>, Receiver<Block>) = channel();

    let client = RefCell::new(Client::new());
    let publickey = client.borrow().wallet.return_publickey();

    thread::spawn(move || {
        client.borrow_mut().init(
            tx_receiver,
            block_sender.clone(),
            block_receiver,
        );
    });

    loop {
        let mut transaction: Transaction = Transaction::new();
        let mut slip: Slip = Slip::new(publickey);
    
        slip.set_amt(200_000_000);
        transaction.add_from_slip(slip);

        let three_seconds = time::Duration::from_millis(3000);
        thread::sleep(three_seconds);
        
        tx_sender.send(transaction).unwrap();
    }
}



