use serde::{Serialize, Deserialize};
use crate::hop::{Hop};
use crate::slip::{Slip};
use crate::helper::{create_timestamp};
use crate::crypto::{Signature, PublicKey};

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
    pub typ:  TransactionBroadcastType,
    path: Vec<Hop>,
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

    pub fn set_msg(&mut self, msg: Vec<u8>) {
        self.body.msg = msg;
    }

    pub fn return_to_slips(&self) -> Vec<Slip> {
        return self.body.to.clone();
    }

    pub fn return_from_slips(&self) -> Vec<Slip> {
        return self.body.from.clone();
    }

    pub fn return_fees_usable(&self, publickey: &PublicKey) -> u64 {
        //  
        // we want to cache this value and reuse it in the future; 
        //
        let input_fees: u64 = self.body.from
            .iter()
            .filter(|slip| &slip.return_add() == publickey)
            .map(|slip| slip.return_amt())
            .sum();

        let output_fees: u64 = self.body.to
            .iter()
            .filter(|slip| &slip.return_add() == publickey)
            .map(|slip| slip.return_amt())
            .sum();

        // need stronger checks here for validation
        // this should not be possible to do unless your 
        // receiving the block reward 

        if input_fees < output_fees {
            return 0;
        } 

        return input_fees - output_fees;
    }

    pub fn set_tx_type(&mut self, tx_type: TransactionBroadcastType) {
        self.body.typ = tx_type;
    }

    //
    // TODO - calculate based on path information not 1
    //
    pub fn return_work_available(&self, _publickey: &str) -> u64 {
        return 1;
    }

    pub fn return_signature_source(&self) -> Vec<u8> {
        return bincode::serialize(&self.body).unwrap();
    }

    pub fn set_sig(&mut self, sig: Signature) {
        self.body.sig = sig 
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




