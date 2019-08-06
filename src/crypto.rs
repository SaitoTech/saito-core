use sha2::Sha256;
use digest::Digest;

pub use merkle::{MerkleTree, Hashable};

pub use ring::digest::{SHA256, Context};
pub use secp256k1::{Secp256k1, Message, Signature, SecretKey, PublicKey};

pub use rand::{Rng, thread_rng};
pub use base58::{ToBase58};


<<<<<<< HEAD
=======
use crate::transaction::Transaction;

>>>>>>> b368740392faf39695a77500d4544a7bbadfe829
pub fn generate_keys() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    return secp.generate_keypair(&mut thread_rng());
}

pub fn hash(data: Vec<u8>, output: &mut [u8]) {
    let mut hasher = Sha256::new();
    hasher.input(data);
    return output.copy_from_slice(hasher.result().as_slice());
}
<<<<<<< HEAD
=======


>>>>>>> b368740392faf39695a77500d4544a7bbadfe829
