use saito_core::network::Network;
use saito_core::consensus::Consensus;
use saito_core::runtime::Runtime;
use saito_core::wallet::Wallet;
use saito_core::lottery::{Lottery, Miner};

use std::thread;
use std::cell::RefCell;
use std::sync::Arc;

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

    let addr = Lottery::create(|ctx| {
        let lottery_addr = ctx.address().recipient();

        // need to add config in here
        let wallet = Arc::new(Wallet::new());

        let mut consensus = Consensus::new(wallet.clone(), lottery_addr);
        let consensus_addr = consensus.start().recipient();
        
        let runtime = Runtime::new();
        let network = Network { consensus_addr: consensus_addr.clone() };

        return Lottery::new(Miner::new(), wallet.clone(), consensus_addr.clone());
    });


    //
    // run the part of the code that handles the exchange of 
    // messages between the different components in the Saito
    // system
    //
    system.run().unwrap();

}



