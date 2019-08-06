use sha2::Sha256;
use digest::Digest;

use merkle::{MerkleTree, Hashable};

use ring::digest::{SHA256, Context};

use secp256k1::{Secp256k1, Message, Signature};
use secp256k1::{SecretKey, PublicKey};

use rand::{Rng, thread_rng};
use base58::{ToBase58};

use crate::transaction::Transaction;

pub fn generate_keys() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    return secp.generate_keypair(&mut thread_rng());
}

// need to implement Hashable trait for Transaction
pub fn create_merkle_root(transactions: &Vec<Transaction>) -> Vec<u8> {
    let merkle = MerkleTree::from_vec(&SHA256, transactions.clone());
    let merkle_root: Vec<u8> = merkle.root_hash().clone();
    return merkle_root;
}

pub fn sha256_hash(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(data);
    return hasher.result().to_vec();
}


