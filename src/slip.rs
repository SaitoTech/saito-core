use serde::{Serialize, Deserialize};
use crate::crypto::PublicKey;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Slip {
    body: SlipBody,
    lc: u8,
    is_valid: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SlipBody {
    add: PublicKey,
    typ: SlipBroadcastType,
    amt: u64,
    bid: u32,
    tid: u32,
    sid: u32,
    bsh: [u8; 32],
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum SlipBroadcastType{
  Normal,
  GoldenTicket,
  Fee,
  Rebroadcast,
  VIP,
  GoldenChunk,
}

impl Slip {
    pub fn new(publickey: PublicKey) -> Slip {
        return Slip {
            body: SlipBody {
                add: publickey,
                typ: SlipBroadcastType::Normal,
                amt: 0,
                bid: 0,
                tid: 0,
                sid: 0,
                bsh: [0; 32],
            },
            lc: 0,
            is_valid: 0,
        }
    }
}




