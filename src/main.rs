use saito_core::transaction::Transaction;

use saito_core::network::Network;
use saito_core::consensus::Consensus;
use saito_core::runtime::Runtime;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

fn main() {
    actix::System::run(|| {
        let mut consensus = Consensus::new();
        let network = Network::new();
        let runtime = Runtime::new();

        // exists in the meanwhile before rolling out more actix handlers  
        let (tx_mempool_sender, tx_mempool_receiver): (Sender<Transaction>, Receiver<Transaction>) = channel();

        let publickey = consensus.wallet.return_publickey();

        thread::spawn(move || {
            consensus.init(tx_mempool_receiver);
        });

        thread::spawn(move || {
            network.init(tx_mempool_sender, publickey);
        });
    });
}



