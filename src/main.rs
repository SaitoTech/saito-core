use saito_core::network::Network;
use saito_core::consensus::Consensus;
use saito_core::runtime::Runtime;
use saito_core::wallet::Wallet;
use saito_core::lottery::{Lottery, Miner};

use std::sync::{Arc, RwLock};

use actix::*;

fn main() {

    //
    // Actix framework allows communications between different
    // parts of the system. There are three major sections that
    // run independently and communicate asynchronously:
    //
    //   - network
    //   - consensus
    //   - runtime
    //
    let system = System::new("SAITO");

    //    
    // Instantiate
    //
    Consensus::create(|ctx| {
        let consensus_addr = ctx.address().recipient();
        
        // need to add config in here
        let wallet = Arc::new(RwLock::new(Wallet::new()));
        
        let lottery = Lottery::new(Miner::new(), wallet.clone(), consensus_addr.clone());
        let lottery_addr = lottery.start().recipient();
        
        let _runtime = Runtime::new();
        let _network = Network { consensus_addr: consensus_addr.clone() };

        return Consensus::new(wallet.clone(), lottery_addr);
    });


    //
    // run the part of the code that handles the exchange of 
    // messages between the different components in the Saito
    // system
    //
    system.run().unwrap();

}



