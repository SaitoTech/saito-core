use serde::{Serialize, Deserialize};
use crate::crypto::PublicKey;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Slip {
    add: PublicKey,
    amt: u64,
    bid: u32,
    tid: u32,
    sid: u32,
    bsh: [u8; 32],
}

impl Slip {
    pub fn new(publickey: PublicKey) -> Slip {
        return Slip {
            add: publickey,
            amt: 0,
            bid: 0,
            tid: 0,
            sid: 0,
            bsh: [0; 32],
        }
    }
}
