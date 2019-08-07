use serde::{Serialize, Deserialize};
use crate::slip::{Slip};
use crate::helper::{create_timestamp};
use crate::crypto::{Signature};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum TransactionType {
  Normal,
  GoldenTicket,
  Fee,
  Rebroadcast,
  VIP,
  GoldenChunk,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Transaction {
    id: u32,
    tx_type: TransactionType,
    timestamp: u128,
    sig: Signature,

    pub to: Vec<Slip>,
    pub from: Vec<Slip>,

    #[serde(with = "serde_bytes")]
    pub msg: Vec<u8>,
}

impl Transaction {
    pub fn new(tx_type: TransactionType) -> Transaction {
        return Transaction {
            id: 0,
            timestamp: create_timestamp(),
            tx_type,
            sig: Signature::from_compact(&[0; 64]).unwrap(),
            to: vec![],
            from: vec![],
            msg: vec![] 
        };
    }

    pub fn add_to_slip(&mut self, slip: Slip) {
        self.to.push(slip);
    }

    pub fn add_from_slip(&mut self, slip: Slip) {
        self.from.push(slip)
    }
}

impl Clone for Transaction {
    fn clone(&self) -> Transaction {
        Transaction {
            id: self.id,
            tx_type: self.tx_type,
            timestamp: self.timestamp,
            sig: self.sig,
            to: self.to.clone(),
            from: self.from.clone(),
            msg: self.msg.clone()
        }
    }
}



