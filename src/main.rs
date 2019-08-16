use saito_core::network::Network;
use saito_core::consensus::Consensus;
use saito_core::runtime::Runtime;

use std::thread;

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

let x :[u8;32] = [0;32];
let y :[u8;32] = [1;32];
let z :[u8;32] = [0;32];

if x == y {
  println!("X is equal to Y");
}
if x == z {
  println!("X is equal to Z");
}


    //    
    // Instantiate
    //
    let mut consensus = Consensus::new();
    let consensus_addr = consensus.clone().start();
    let network = Network { consensus_addr };
    let runtime = Runtime::new();
    let publickey = consensus.wallet.return_publickey();


    //
    // Initialize
    //
    thread::spawn(move || {
        consensus.init();
    });

    thread::spawn(move || {
        network.init(publickey);
    });


    //
    // run the part of the code that handles the exchange of 
    // messages between the different components in the Saito
    // system
    //
    system.run().unwrap();

}



