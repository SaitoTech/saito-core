use std::{thread, time};
use std::net::SocketAddr;

use saito_primitives::slip::Slip;
use saito_primitives::block::Block;
use saito_primitives::crypto::PublicKey;
use saito_primitives::transaction::Transaction;

use serde::{Serialize, Deserialize};

use actix::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PeerId(PublicKey);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: PeerId,
    pub addr: Option<SocketAddr>,
}

impl PeerInfo {
    pub fn new(id: PeerId, addr: SocketAddr) -> PeerInfo {
        return PeerInfo { id, addr: Some(addr) };
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PeerType {
    Lite,
    FullNode,
}


#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PeerStatus {
    Connecting,
    Connected,
    Disconnected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    pub peer_info: PeerInfo,
    pub peer_type: PeerType, 
    pub peer_status: PeerStatus, 
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Handshake {
    /// Protocol version.
    pub version: u32,
    /// Sender's peer id.
    pub peer_id: PeerId,
    /// Sender's listening addr.
    pub listen_port: Option<u16>,
}

pub struct Network {
//    pub peers: Vec<Peer> ,
    pub consensus_addr: Recipient<NetworkMessage>
}

impl Actor for Network {
    type Context = Context<Network>;
} 

#[derive(Message, Debug)]
pub enum NetworkMessage {
    IncomingBlock(Block),
    IncomingTransaction(Transaction),
}

#[derive(Debug)]
pub enum NetworkRequest {
    Block,
    Transaction
}

impl Network {
    pub fn init(&self, publickey: PublicKey) {
        println!("init network");
        let request = NetworkRequest::Transaction;
        loop {
            let three_seconds = time::Duration::from_millis(3000);
            thread::sleep(three_seconds);

            match request {
                NetworkRequest::Block => self.send_block_to_consensus(publickey),
                NetworkRequest::Transaction => self.send_transaction_to_consensus(publickey),
            }
        }
    }

    pub fn send_transaction_to_consensus(&self, publickey: PublicKey) {
        println!("send tx");
        let mut tx: Transaction = Transaction::new();
        let mut slip: Slip = Slip::new(publickey);

        slip.set_amt(200_000_000);
        tx.add_from_slip(slip);

        self.consensus_addr.do_send(NetworkMessage::IncomingTransaction(tx)).unwrap();
    }

    pub fn send_block_to_consensus(&self, publickey: PublicKey) {
        println!("send block");
        let blk = Block::new(publickey, [1;32]);
        self.consensus_addr.do_send(NetworkMessage::IncomingBlock(blk)).unwrap();
    }

}


