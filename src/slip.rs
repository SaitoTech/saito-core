use serde::{Serialize, Deserialize};
use crate::crypto::PublicKey;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Slip {
    body: SlipBody,
    lc: u8,
    is_valid: u8,
    spent_status: SlipSpentStatus,
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
pub enum SlipBroadcastType {
  Normal,
  GoldenTicket,
  Fee,
  Rebroadcast,
  VIP,
  GoldenChunk,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum SlipSpentStatus {
  Unspent,
  Spent,
  Pending,
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
            spent_status: SlipSpentStatus::Spent,
        }
    }

    pub fn set_amt(&mut self, amt: u64) {
        self.body.amt = amt;
    }

    pub fn return_amt(&self) -> u64 {
        return self.body.amt;
    }

    // implication is slip type is immutable
    pub fn return_signature_source(&self) -> Vec<u8> {
       return bincode::serialize(&self.body).unwrap();
    }
}




