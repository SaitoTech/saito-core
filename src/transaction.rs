use serde::{Serialize, Deserialize};
use crate::routingpath::RoutingPath;
use crate::slip::{Slip};
use crate::helper::{create_timestamp};
use crate::crypto::{Signature};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum TransactionBroadcastType {
  Normal,
  GoldenTicket,
  Fee,
  Rebroadcast,
  VIP,
  GoldenChunk,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TransactionBody {
    id:   u32,
    ts:   u64,
    pub   to: Vec<Slip>,
    pub   from: Vec<Slip>,
    sig:  Signature,
    ver:  f32,
    typ:  TransactionBroadcastType,
    path: Vec<RoutingPath>,
    msg:  Vec<u8>,
    ps:   u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Transaction {

    body: TransactionBody,
    is_valid: u8,

//    msg_hash: [u8; 32],
//    size: u64,
//    fees_total: u64,
//    fees_usable_for_block_producer: u64,
//    fees_cumulative: s64,
//    decrypted_msg: Vec<u8>,

}
   
impl Transaction {
    pub fn new() -> Transaction {
        return Transaction {
            body: TransactionBody {
                id:   0,
		ts:   create_timestamp(),
		to:   vec![],
		from: vec![],
		sig:  Signature::from_compact(&[0; 64]).unwrap(),
		ver:  0.1,
		typ:  TransactionBroadcastType::Normal,
		path: vec![],
		msg:  vec![],
		ps:   0
            },
            is_valid: 0
        };
    }

    pub fn add_to_slip(&mut self, slip: Slip) {
        self.body.to.push(slip);
    }

    pub fn add_from_slip(&mut self, slip: Slip) {
        self.body.from.push(slip)
    }
}

impl Clone for TransactionBody {
    fn clone(&self) -> TransactionBody {
        TransactionBody {
            id:   self.id,
	    ts:   self.ts,
	    to:   self.to.clone(),
	    from: self.from.clone(),
	    sig:  self.sig,
	    ver:  self.ver,
	    typ:  self.typ,
	    path: self.path.clone(),
	    msg:  self.msg.clone(),
	    ps:   self.ps
        }
    }
}




