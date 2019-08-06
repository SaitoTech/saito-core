use std::mem::transmute;
use serde::{Serialize, Deserialize};

use secp256k1::{Signature};
use merkle::{Hashable};
use ring::digest::Context;

use crate::helper::{time_since_unix_epoch};
use crate::slip::{Slip};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum TransactionType {
  Base,
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
            timestamp: time_since_unix_epoch(),
            tx_type,
            sig: Signature::from_compact(&[0; 64]).unwrap(),
            to: Vec::new(),
            from: Vec::new(),
            msg: Vec::new()
        };
    }

    pub fn add_to_slip(&mut self, slip: Slip) {
        self.to.push(slip);
    }

    pub fn add_from_slip(&mut self, slip: Slip) {
        self.from.push(slip)
    }

    pub fn return_signature_source(&self) -> Vec<u8> {
        let mut sig_source_bytes: Vec<u8> = Vec::new();
        let timestamp_bytes: [u8; 16] = unsafe { transmute(self.timestamp.to_be()) };

        sig_source_bytes.extend(&timestamp_bytes);

        for slip in self.from.iter() {
            sig_source_bytes.extend(slip.return_index());
        }

        for slip in self.to.iter() {
            sig_source_bytes.extend(slip.return_index());
        }
        return sig_source_bytes;
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

impl Hashable for Transaction {
    fn update_context(&self, context: &mut Context) {
        context.update(&self.return_signature_source());
    }
}