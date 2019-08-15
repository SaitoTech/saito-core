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


    genesis_ts:	    u64,
    genesis_bid:    u32,
    genesis_period: u32,


    lowest_acceptable_ts:	u64,
    lowest_acceptable_bsh:	[u8; 32],
    lowest_acceptable_bid:	u32,

}

impl Blockchain {

    pub fn new() -> Blockchain {
        return Blockchain {

	    index:         	   BlockchainIndex::new(),
	    bsh_lc_hmap:   	   HashMap::new(),
	    bsh_bid_hmap:  	   HashMap::new(),
	    lc_pos_set:    	   false,
	    lc_pos:        	   0,

	    genesis_ts:	    	   0,
	    genesis_bid:    	   0,
	    genesis_period: 	   0,

	    lowest_acceptable_ts:  0,
	    lowest_acceptable_bsh: [0; 32],
	    lowest_acceptable_bid: 0,

        };
    }


    pub fn add_block(&mut self, blk: Block) {

	//
	// check block is valid
	//
	if blk.is_valid == 0 {
	    println!("block is not valid - terminating add_block in blockchain...");
	    return;
	}

	//
	// create reference for previous lc
	//
	let last_lc_pos = self.lc_pos; 


	//
	// ignore pre-genesis blocks
	//
	if blk.body.ts < self.genesis_ts || blk.body.id < self.genesis_bid {
	    //
	    // TODO
	    //
	    // we ignore this restriction if we are loading from disk / forcing load
	    //
	    println!("not adding block to blockchain -- block precedes genesis");
	    return;
	}


	//
	// ignore hash collisions
	//
	if self.is_bsh_indexed(blk.return_bsh()) {
	    println!("not adding block to blockchain -- bsh already indexed");
	    return;
	}


  	//
  	// previous block not indexed, but acceptable
  	//
  	if (blk.body.ts < self.lowest_acceptable_ts) {
      	    self.lowest_acceptable_ts = blk.body.ts;
  	}


  	//
  	// track first block
  	//
  	// if we are adding our first block, we set this as our lowest
	// acceptable ts to avoid requesting earlier blocks as infinitum
	// into the past.
	//
	// lowest acceptable bid must be updated so that we know the 
	// earliest block we need to worry about when handling full slip
	// validation.
  	//
  	if (self.lowest_acceptable_ts == 0) {

	    self.lowest_acceptable_bid = blk.body.id;
    	    self.lowest_acceptable_bsh = blk.return_bsh();

	    //
	    // we update the lowest acceptable TS from the last
	    // time the server was running if we aren't indexing
	    // from the start.
	    //
	    //if (this.app.options.blockchain != null) {
      	    //    this.lowest_acceptable_ts = this.last_ts;
            //}

            if (self.lowest_acceptable_ts == 0) {
                self.lowest_acceptable_ts = blk.body.ts;
            }

	} else {

    	    if (self.lowest_acceptable_ts > blk.body.ts) {

		//
		// TODO
		//
		// we do not update this if we are forcing blocks in
		//
		//if ( && !(force)) {
      		    self.lowest_acceptable_ts = blk.body.ts;
    	    	//}
    	    }

        }






  	//
  	// fetch missing blocks
	//
	if blk.body.ts > self.lowest_acceptable_ts {
      	    if !self.is_bsh_indexed(blk.body.prevbsh)  {
		if self.lc_pos_set == true {
        	    if blk.body.id > (self.index.blocks[self.lc_pos].body.id - self.genesis_period) {

			//
			// TODO
		        //
		        // send request for missing block
		        //

		    }
        	}
      	    }
    	}


	//
	// insert indexes
	//
	let pos = self.index.blocks.len();
        self.index.blocks.insert(pos, blk);
        self.bsh_lc_hmap.insert(blk.return_bsh(), 1);
        self.bsh_bid_hmap.insert(blk.return_bsh(), blk.body.id);



	//
	// identify longest chain
	//
	self.lc_pos = pos;
	self.lc_pos_set = true;


	let mut i_am_the_longest_chain : u8 = 0;
        let mut shared_ancestor_pos : i32 = -1;


	println!("Adding block: {:?}", self.return_latest_bsh()); 
	println!("lc: {:?}", i_am_the_longest_chain);
        println!("ancestor: {:?}", shared_ancestor_position);

    }

    pub fn is_bsh_indexed(&mut self, bsh:[u8; 32] ) -> bool {
	if self.bsh_lc_hmap.contains_key(&bsh) {
	    return true;
	} else {
	    return false;
	}
    }

    pub fn return_latest_ts(&mut self) -> u64 {
        if (!self.lc_pos_set) { return 0; }
	return self.index.blocks[self.lc_pos].body.ts;
    }

    pub fn return_latest_bsh(&self) -> [u8; 32] {
        if (!self.lc_pos_set) { return [0; 32]; }
	return self.index.blocks[self.lc_pos].return_bsh();
    }

    pub fn return_latest_bf_current(&self) -> f32 {
        if (!self.lc_pos_set) { return 0.0; }
	return self.index.blocks[self.lc_pos].body.bf.current;
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
    //bsh:         Vec<[u8; 32]>,               // hashes
    //prevbsh:     Vec<[u8; 32]>,               // hash of previous block
    //bid:         Vec<u32>,                    // block id
    //mintid:      Vec<u32>,
    //maxtid:      Vec<u32>,
    //ts:          Vec<u64>,                    // timestamps
    //lc:          Vec<u8>,                     // is longest chain (0 = no, 1 = yes)
    //bf:          Vec<f32>                     // burnfee per block
}

impl BlockchainIndex {
    pub fn new() -> BlockchainIndex {
        return BlockchainIndex {
            blocks:      vec![],                 // blocks
            //bsh:         vec![],                 // hashes
            //prevbsh:     vec![],                 // hash of previous block
            //bid:         vec![],                 // block id
            //mintid:      vec![],                 // min tid
            //maxtid:      vec![],                 // max tid
            //ts:          vec![],                 // timestamps
            //lc:          vec![],                 // is longest chain (0 = no, 1 = yes)
            //bf:          vec![]                  // burnfee per block
        };
    }
}






