use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::block::{Block};
use crate::burnfee::BurnFee;

//
// The Blockchain
//
// the contents of this data object represent the state of the
// blockchain itself, including the blocks that are on the 
// longest-chain as well as the material that is sitting off
// the longest-chain but capable of being switched over.
//
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Blockchain {
    index:          BlockchainIndex,
    bsh_lc_hmap:    HashMap<[u8; 32], u8>,
    bsh_bid_hmap:   HashMap<[u8; 32], u32>,
    lc_pos_set:     bool,
    lc_pos:         usize,				// pos of lc

}

impl Blockchain {

    pub fn new() -> Blockchain {
        return Blockchain {
	    index:         BlockchainIndex::new(),
	    bsh_lc_hmap:   HashMap::new(),
	    bsh_bid_hmap:  HashMap::new(),
	    lc_pos_set:    false,
	    lc_pos:        0,
        };
    }


    pub fn add_block(&mut self, blk: Block) {

	let pos = self.index.blocks.len();

        //
	// add block data to index
	//
        self.index.bsh.insert(pos, blk.return_bsh());
        self.index.prevbsh.insert(pos, blk.body.prevbsh);
        self.index.bid.insert(pos, blk.body.id);
        self.index.mintid.insert(pos, 0);
        self.index.maxtid.insert(pos, 0);
        self.index.ts.insert(pos, blk.body.ts);
        self.index.lc.insert(pos, 1);
        self.index.bf.insert(pos, BurnFee::new(0.0,0.0).current);
        self.index.blocks.insert(pos, blk);

	self.lc_pos = pos;
	self.lc_pos_set = true;

	println!("Adding block: {:?}", self.return_latest_bsh()); 

    }

    pub fn return_latest_ts(&mut self) -> u64 {
        if (!self.lc_pos_set) { return 0; }
	return self.index.ts[self.lc_pos];
    }

    pub fn return_latest_bsh(&self) -> [u8; 32] {
        if (!self.lc_pos_set) { return [0; 32]; }
	return self.index.bsh[self.lc_pos];
    }

    pub fn return_latest_bf_current(&self) -> u64 {
        return 100_000_000_000;
    }

    pub fn return_heartbeat(&self) -> u64 {
        return 100_000;
    }

}




//
// The Blockchain Indices 
//
// the contents of this data object are kept in sync so that
// every element in every vector references the same implicit
// block, regardless of whether it is on the longest chain or
// not.
//
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlockchainIndex {
    blocks:      Vec<Block>,                  // blocks
    bsh:         Vec<[u8; 32]>,               // hashes
    prevbsh:     Vec<[u8; 32]>,               // hash of previous block
    bid:         Vec<u32>,                    // block id
    mintid:      Vec<u32>,
    maxtid:      Vec<u32>,
    ts:          Vec<u64>,                    // timestamps
    lc:          Vec<u8>,                     // is longest chain (0 = no, 1 = yes)
    bf:          Vec<f32>                     // burnfee per block
}

impl BlockchainIndex {
    pub fn new() -> BlockchainIndex {
        return BlockchainIndex {
            blocks:      vec![],                 // blocks
            bsh:         vec![],                 // hashes
            prevbsh:     vec![],                 // hash of previous block
            bid:         vec![],                 // block id
            mintid:      vec![],                 // min tid
            maxtid:      vec![],                 // max tid
            ts:          vec![],                 // timestamps
            lc:          vec![],                 // is longest chain (0 = no, 1 = yes)
            bf:          vec![]                  // burnfee per block
        };
    }
}






