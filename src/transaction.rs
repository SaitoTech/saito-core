use serde::{Serialize, Deserialize};
use crate::slip::{Slip};
use crate::helper::{create_timestamp};
use crate::crypto::{Signature};


#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum TransactionBroadcastType {
  Normal,
  GoldenTicket,
  Fee,
  Rebroadcast,
  VIP,
  GoldenChunk,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TransactionBody {
    id:   u32,
    ts:   u128,
    pub   to: Vec<Slip>,
    pub   from: Vec<Slip>,
    sig:  Signature,
    ver:  s16,
    typ:  TransactionBroadcastType,
    path: Vec<RoutingPath>,
    msg:  Vec<u8>,
    ps:   u8,
}


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Transaction {

    body: TransactionBody,
    is_valid: u8,

    #[serde(with = "serde_bytes")]

//    msg_hash: [u8; 32],
//    size: u64,
//    fees_total: u64,
//    fees_usable_for_block_producer: u64,
//    fees_cumulative: s64,
//    decrypted_msg: Vec<u8>,

}

   

impl Transaction {
    pub fn new(tx_type: TransactionBroadcastType) -> Transaction {
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



