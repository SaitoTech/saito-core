use std::mem::transmute;

use serde::{Serialize, Deserialize};
use sha2::Sha256;
use digest::Digest;

use secp256k1::{PublicKey};

use crate::crypto::{sha256_hash, create_merkle_root};
use crate::helper::{time_since_unix_epoch};

use crate::transaction::{Transaction};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Block {
    id: u32,

    #[serde(with = "serde_bytes")]
    previous_hash: Vec<u8>,

    #[serde(with = "serde_bytes")]
    merkle_root: Vec<u8>,

    pub timestamp: u128,
    creator: PublicKey,

    pub transactions: Vec<Transaction>,
    difficulty: f32,
    paysplit: f32,
    treasury: f32,
    coinbase: f32,
    reclaimed: f32
}

impl Block {
    pub fn new(previous_hash: Vec<u8>, publickey: PublicKey) -> Block {
        return Block {
            id: 1,
            timestamp: time_since_unix_epoch(),
            previous_hash,
            merkle_root: Vec::new(),
            creator: publickey,
            transactions: Vec::new(),
            difficulty: 0.0,
            paysplit: 0.5,
            treasury: 2868100000.0,
            coinbase: 0.0,
            reclaimed: 0.0
        };
    }

    pub fn return_block_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let mut data: Vec<u8> = vec![];

        let id_bytes: [u8; 4] = unsafe { transmute(self.id.to_be()) };
        let timestamp_bytes: [u8; 16] = unsafe { transmute(self.timestamp.to_be()) };
        let address_bytes: Vec<u8> = self.creator.serialize().iter().cloned().collect();

        data.extend(&id_bytes);
        data.extend(&timestamp_bytes);
        data.extend(&address_bytes);

        return sha256_hash(data);
    }

    pub fn return_transactions(&self) -> Vec<Transaction> {
        return self.transactions.clone();
    }

    pub fn set_merkle_root(&mut self) {
        self.merkle_root = create_merkle_root(&self.transactions.clone());
    }

    pub fn return_slip_len(&self) -> u32 {
        let mut slip_number: u32 = 0;
        for tx in self.transactions.iter() {
            slip_number += tx.to.len() as u32;
            slip_number += tx.from.len()as u32;
        }
        return slip_number
    }

    pub fn return_tx_len(&self) -> u32{
        return self.transactions.len() as u32;
    }
}