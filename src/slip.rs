use serde::{Serialize, Deserialize};
use secp256k1::PublicKey;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Slip {
    address: PublicKey,
    amount: f32,
    block_id: u32,
    transaction_id: u32,
    id: u32,

    #[serde(with = "serde_bytes")]
    block_hash: Vec<u8>,
}

impl Slip {
    pub fn new(publickey: PublicKey) -> Slip {
        return Slip {
            address: publickey,
            amount: 0.0,
            block_id: 0,
            transaction_id: 0,
            id: 0,
            block_hash: Vec::new(),
        }
    }

    pub fn return_index(&self) -> Vec<u8> {
        return bincode::serialize(self).unwrap();
    }
}