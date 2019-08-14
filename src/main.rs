use saito_core::transaction::Transaction;

use saito_core::network::Network;
use saito_core::consensus::Consensus;
use saito_core::runtime::Runtime;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

use actix::*;

fn main() {
    let system = System::new("SAITO");

    let mut consensus = Consensus::new();
    let consensus_addr = consensus.clone().start();
    let network = Network { consensus_addr };

    let runtime = Runtime::new();

    let publickey = consensus.wallet.return_publickey();

    thread::spawn(move || {
        consensus.init();
    });

    thread::spawn(move || {
        network.init(publickey);
    });

    system.run();


    // Network
    // - has Consensus Addr
    // - has Runtime Addr
    // - NetworkManager 
    //
    // Consensus
    // - has Runtime Addr
    // - has Network Addr
    //
    // Runtime
    //  - Consensus Addr
    //  - has Network Addr
    //
    // and the runtime addr to consensus
}



