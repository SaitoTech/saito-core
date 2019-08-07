use std::mem::transmute;
use serde::{Serialize, Deserialize};
use crate::crypto::{hash, PublicKey};
use crate::helper::{create_timestamp};
use crate::transaction::{Transaction};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Block {
    id: u32,
    previous_hash: [u8; 32],
    merkle_root: [u8; 32],
    pub timestamp: u128,
    creator: PublicKey,
    pub transactions: Vec<Transaction>,
    difficulty: f32,
    paysplit: f32,
    treasury: u64,
    coinbase: u64,
    reclaimed: u64
}

impl Block {
    pub fn new(previous_hash: [u8; 32], publickey: PublicKey) -> Block {
        return Block {
            id: 1,
            timestamp: create_timestamp(),
            previous_hash,
            merkle_root: [0; 32],
            creator: publickey,
            transactions: vec![],
            difficulty: 0.0,
            paysplit: 0.5,
            treasury: 286_810_000_000_000_000,
            coinbase: 0,
            reclaimed: 0,
        };
    }

    pub fn return_block_hash(&self) -> [u8; 32] {
        let mut data: Vec<u8> = vec![];

        let id_bytes: [u8; 4] = unsafe { transmute(self.id.to_be()) };
        let timestamp_bytes: [u8; 16] = unsafe { transmute(self.timestamp.to_be()) };
        let address_bytes: Vec<u8> = self.creator.serialize().iter().cloned().collect();

        data.extend(&id_bytes);
        data.extend(&timestamp_bytes);
        data.extend(&address_bytes);

        let mut output: [u8; 32] = [0; 32];

        hash(data, &mut output);

        return output;
    }
}
