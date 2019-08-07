use serde::{Serialize, Deserialize};
use crate::crypto::{PublicKey, Signature};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RoutingPath {
    from: PublicKey,
    to: PublicKey,
    sig: Signature,
}

impl RoutingPath {
    pub fn new(to: PublicKey, from: PublicKey, sig: Signature) -> RoutingPath {
        return RoutingPath { to, from, sig }
    }
}
