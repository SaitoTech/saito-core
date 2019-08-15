use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use saito_primitives::block::{Block};
use saito_primitives::burnfee::BurnFee;

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

    last_bsh:			[u8; 32],
    last_bid:			u32,
    last_ts:			u64,

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

	    last_bsh:		   [0; 32],
	    last_bid:		   0,
	    last_ts:		   0,

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
	let pos :usize = self.index.blocks.len();
        self.index.blocks.insert(pos, blk);
        self.bsh_lc_hmap.insert(blk.return_bsh(), 1);
        self.bsh_bid_hmap.insert(blk.return_bsh(), blk.body.id);



	//
	// identify longest chain
	//
	let mut i_am_the_longest_chain : u8 = 0;
        let mut shared_ancestor_pos : i32 = -1;



	if self.index.blocks.len() == 1 {
	
    	    //
    	    // starting point
    	    //
    	    if self.last_bid > 0 {
      		if blk.body.prevbsh == self.last_bsh {
       		    i_am_the_longest_chain = 1;
      		}
    	    } else {
      		i_am_the_longest_chain = 1;
    	    }

	} else {

	    if blk.body.id >= self.index.blocks[self.lc_pos].body.id {
  
    		if blk.body.prevbsh == self.index.blocks[self.lc_pos].return_bsh() {
        	    i_am_the_longest_chain = 1;
      		} else {

		    //
        	    // find the last shared ancestor
        	    //
        	    let lchain_pos       :usize   = self.lc_pos;
        	    let nchain_pos       :usize   = pos;
        	    let lchain_len       :u32     = 0;
        	    let nchain_len       :u32     = 0;
        	    let lchain_bf        :f32     = self.index.blocks[lchain_pos].body.bf.current;
        	    let nchain_bf        :f32     = self.index.blocks[nchain_pos].body.bf.current;
        	    let lchain_ts        :u64     = self.index.blocks[lchain_pos].body.ts;
        	    let nchain_ts        :u64     = self.index.blocks[nchain_pos].body.ts;
        	    let lchain_prevbsh   :[u8;32] = self.index.blocks[lchain_pos].body.prevbsh;
        	    let nchain_prevbsh   :[u8;32] = self.index.blocks[nchain_pos].body.prevbsh;
	            let search_pos       :usize   = 0;
        	    let search_bf        :f32  	  = 0.0;
        	    let search_ts        :u64     = 0;
        	    let search_bsh       :[u8;32] = [0;32];
        	    let search_prevbsh   :[u8;32] = [0;32];
		    let search_completed :bool    = false;

	            if nchain_ts >= lchain_ts {
	                search_pos = nchain_pos - 1;
        	    } else {
          	        search_pos = lchain_pos - 1;
         	    }

		    while search_pos >= 0 && search_completed == false {

	                search_ts         = self.index.blocks[search_pos].body.ts;
          	        search_bf         = self.index.blocks[search_pos].body.bf.current;
          	        search_bsh        = self.index.blocks[search_pos].return_bsh();
          	        search_prevhash   = self.index.prevhash[search_pos].body.prevbsh;

			//
			// we find the common ancestor
			//
		        if search_bsh == lchain_prevbsh && search_bsh == nchain_prevbsh {
			    shared_ancestor_pos = search_pos;
			    search_completed = true;

			//
			// or we keep looking
			//
			} else {

            		    if (search_bsh == lchain_prevbsh) {
            		        lchain_len = lchain_len + 1; 
            		        lchain_prevbsh = self.index.blocks[search_pos].body.prevbsh;
            		        lchain_bf = lchain_bf + self.index.blocks[search_pos].body.bf.current;
            		    }
            
            		    if (search_bsh == nchain_prevbsh) {
            		        nchain_prevbsh = this.index.blocks[search_pos].body.prevbsh;
            		        nchain_len = nchain_len + 1; 
            		        nchain_bf = nchain_bf + self.index.blocks[search_pos].body.bf.current;
            		    }
            
            		    shared_ancestor_pos = search_pos;
            		    search_pos = search_pos - 1;
            
            		    //
            		    // new chain completely disconnected
            		    // 
            		    if (shared_ancestor_pos == 1) {
            		        if (nchain_prevbsh == "") {
				    //
				    // add the block, and escape from this
				    //
            		            //await this.addBlockToBlockchainSuccess(newblock, pos, 0);
				    println!("blockchain - block disconnected from chain");
            		            return;
            		        } 
            		    } 

            		    if (shared_ancestor_pos == 0) {
            		        if (nchain_prevbsh != lchain_prevbsh) {
				    //
				    // add the block, and escape from this
				    //
            		            //await this.addBlockToBlockchainSuccess(newblock, pos, 0);
				    println!("blockchain - block disconnected from chain");
            		            return;
            		        } 
            		    } 

   		        }
		    }
	    	}
	    }
	}




	self.lc_pos = pos;
	self.lc_pos_set = true;

	println!("Adding block: {:?}", self.return_latest_bsh()); 
	println!("lc: {:?}", i_am_the_longest_chain);
        println!("ancestor: {:?}", shared_ancestor_pos);

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






