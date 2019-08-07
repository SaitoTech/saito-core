use std::mem::transmute;
use serde::{Serialize, Deserialize};
use crate::crypto::{hash, PublicKey};
use crate::helper::{return_timestamp};
use crate::transaction::Transaction;
use crate::burnfee::BurnFee;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Block {
    body:     BlockBody,
    is_valid: u8,
    mintid:   u32,
    maxtid:   u32,
    bsh:      [u8; 32],
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlockBody {
    pub id:          u32,
    pub ts:          u64,
    prevhash:        [u8; 32],
    merkle:          [u8; 32],
    pub creator:     PublicKey,
    pub txs:         Vec<Transaction>,
    bf:		     BurnFee,
    difficulty:      f32,
    paysplit:        f32,
    vote:            i8,
    treasury:        u64,
    coinbase:        u64,
    reclaimed:       u64
}



impl BlockBody {
    pub fn new(previous_hash: [u8; 32] , creator : PublicKey ) -> BlockBody {
        return BlockBody {
    	    id:          0,
    	    ts:          return_timestamp(),
    	    prevhash:    previous_hash,
    	    merkle:      [0; 32],
    	    creator:     creator,
    	    txs:         vec![],
	    bf:          BurnFee::new(0.0, 0.0),
    	    difficulty:  0.0,
    	    paysplit:    0.0,
    	    vote:        0,
    	    treasury:    0,
    	    coinbase:    0,
    	    reclaimed:   0
        };
    }
}

impl Block {
    pub fn new(prevhash: [u8; 32], creator: PublicKey) -> Block {
        return Block {
	    body:      BlockBody::new(prevhash, creator),
	    is_valid:  0,
	    mintid:    0,
	    maxtid:    0,
	    bsh:       [0; 32],
        };
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.body.txs.push(tx);
    }

    pub fn return_block_hash(&self) -> [u8; 32] {

        let mut data: Vec<u8> = vec![];

        let id_bytes: [u8; 4] = unsafe { transmute(self.body.id.to_be()) };
        let ts_bytes: [u8; 8] = unsafe { transmute(self.body.ts.to_be()) };
        let cr_bytes: Vec<u8> = self.body.creator.serialize().iter().cloned().collect();

        data.extend(&id_bytes);
        data.extend(&ts_bytes);
        data.extend(&cr_bytes);

        let mut output: [u8; 32] = [0; 32];

        hash(data, &mut output);

        return output;
    }

}
