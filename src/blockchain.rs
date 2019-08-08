use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::block::{Block};


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
}


impl Blockchain {
    pub fn new() -> Blockchain {
        return Blockchain {
	    index:         BlockchainIndex::new(),
	    bsh_lc_hmap:   HashMap::new(),
	    bsh_bid_hmap:  HashMap::new(),
        };
    }


    pub fn add_block(&mut self, blk: Block) {
      println!("{:?}", blk);
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
    blocks:      Vec<Block>,                // blocks
    hash:        Vec<[u8; 32]>,               // hashes
    prevhash:    Vec<[u8; 32]>,               // hash of previous block
    bid:         Vec<[u32; 1]>,               // block id
    mintid:      Vec<[u32; 1]>,               // min tid
    maxtid:      Vec<[u32; 1]>,               // max tid
    ts:          Vec<[u64; 1]>,               // timestamps
    lc:          Vec<[u8; 1]>,                // is longest chain (0 = no, 1 = yes)
    bf:          Vec<[f32; 1]>                // burnfee per block
}

impl BlockchainIndex {
    pub fn new() -> BlockchainIndex {
        return BlockchainIndex {
            blocks:      vec![],                 // blocks
            hash:        vec![],                 // hashes
            prevhash:    vec![],                 // hash of previous block
            bid:         vec![],                 // block id
            mintid:      vec![],                 // min tid
            maxtid:      vec![],                 // max tid
            ts:          vec![],                 // timestamps
            lc:          vec![],                 // is longest chain (0 = no, 1 = yes)
            bf:          vec![]                  // burnfee per block
        };
    }
}











