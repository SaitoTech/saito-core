use saito_core::network::Network;
use saito_core::consensus::Consensus;
use saito_core::runtime::Runtime;

use std::thread;

use actix::*;

fn main() {
    // Start Actix 
    let system = System::new("SAITO");

    let mut consensus = Consensus::new();
    
    // We get the Actix Addr for Consensus and save it as consensus_addr 
    // We pass it as an attribute of Network so that it has the ability to send NetworkMessages
    let consensus_addr = consensus.clone().start();
    let network = Network { consensus_addr };

    let runtime = Runtime::new();
    let publickey = consensus.wallet.return_publickey();

    // Consensus needs to run in an inpdendent thread as to not block the main thread 
    thread::spawn(move || {
        consensus.init();
    });

    // Currently runs in new thread as it is emulating networking traffic being sent to consensus 
    thread::spawn(move || {
        network.init(publickey);
    });

    system.run().unwrap();

    // Network
    // - has Consensus Addr
    // - has Runtime Addr
    //
    // Consensus
    // - has Runtime Addr
    // - has Network Addr
    //
    // Runtime
    //  - Consensus Addr
    //  - has Network Addr
    //
}



