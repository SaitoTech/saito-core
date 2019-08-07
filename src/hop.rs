use serde::{Serialize, Deserialize};
use crate::crypto::{PublicKey, Signature};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Hop {
    from: PublicKey,
    to: PublicKey,
    sig: Signature,
}

impl Hop {
    pub fn new(to: PublicKey, from: PublicKey, sig: Signature) -> Hop {
        return Hop { to, from, sig }
    }
}
