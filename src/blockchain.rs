use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use saito_primitives::block::{Block};
use saito_primitives::burnfee::BurnFee;

//
// Block Header (for index)
//
// the contents of this data object represent the information 
// about the block itself that is stored in the blockchain
// index. it is used primarily when rolling / unrolling the 
// blockchain.
//
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlockHeader {
    bf	 :f32,
    bsh  :[u8;32],
    prevbsh  :[u8;32],
    bid  :u32,
    ts   :u64,
}

impl BlockHeader {
    pub fn new(bf :f32, bsh :[u8;32], prevbsh :[u8;32], bid :u32, ts :u64) -> BlockHeader {
        return BlockHeader {
	    bf:	                   bf,
	    bsh:		   bsh,
	    prevbsh:		   bsh,
	    bid:		   bid,
	    ts:		   	   ts,
        };
    }
}





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

	///////////////////
	// SANITY CHECKS //
	///////////////////

	//
	// check block is superficially valid
	//
	if blk.is_valid == 0 {
	    println!("block is not valid - terminating add_block in blockchain...");
	    return;
	}

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



	/////////////////////////////////
	// SETTING IMPORTANT VARIABLES //
	/////////////////////////////////

	//
	// create reference for previous lc
	//
	let last_lc_pos = self.lc_pos; 


  	//
  	// previous block not indexed, but acceptable
  	//
  	if blk.body.ts < self.lowest_acceptable_ts {
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
  	if self.lowest_acceptable_ts == 0 {

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

            if self.lowest_acceptable_ts == 0 {
                self.lowest_acceptable_ts = blk.body.ts;
            }

	} else {

    	    if self.lowest_acceptable_ts > blk.body.ts {

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
        	    if blk.body.id > (self.index.blocks[self.lc_pos].bid - self.genesis_period) {

			//
			// TODO
		        //
		        // send request for missing block
		        //

		    }
        	}
      	    }
    	}



	////////////////////
	// insert indexes //
	////////////////////
	//
	// TODO -- we insert the full block, including transactions but we should
	// probably avoid this. We could try inserting a ghost block that does 
	// not have the transaction data but that has the stuff we care about in
	// order to determine the public chain.
	//
	// bf / ts / prevbsh / bsh / bid
	//
	//
        let block_header_entry = BlockHeader::new(blk.body.bf.current, blk.return_bsh(), blk.body.prevbsh, blk.body.id, blk.body.ts);

	//
	// TODO - binary search on insert point
	//
	let pos :usize = self.index.blocks.len();
        self.bsh_bid_hmap.insert(blk.return_bsh(), blk.body.id);
        self.index.blocks.insert(pos, block_header_entry);



	////////////////////////////
	// identify longest chain //
	////////////////////////////

	let mut i_am_the_longest_chain : u8 = 0;
        let mut shared_ancestor_pos : usize = 0;
        let mut shared_ancestor_pos_found : bool = false;


	//
	// find the shared ancestor position
	// figure out if i am on the longest chain
        //
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

	    if blk.body.id >= self.index.blocks[self.lc_pos].bid {

    		if blk.body.prevbsh == self.index.blocks[self.lc_pos].bsh {
        	    i_am_the_longest_chain = 1;
      		} else {

		    //
        	    // find the last shared ancestor
        	    //
        	    let mut lchain_pos       :usize   = self.lc_pos;
        	    let mut nchain_pos       :usize   = pos;
        	    let mut lchain_len       :u32     = 0;
        	    let mut nchain_len       :u32     = 0;
        	    let mut lchain_bf        :f32     = self.index.blocks[lchain_pos].bf;
        	    let mut nchain_bf        :f32     = self.index.blocks[nchain_pos].bf;
        	    let mut lchain_ts        :u64     = self.index.blocks[lchain_pos].ts;
        	    let mut nchain_ts        :u64     = self.index.blocks[nchain_pos].ts;
        	    let mut lchain_prevbsh   :[u8;32] = self.index.blocks[lchain_pos].prevbsh;
        	    let mut nchain_prevbsh   :[u8;32] = self.index.blocks[nchain_pos].prevbsh;
	            let mut search_pos       :usize   = 0;
        	    let mut search_bf        :f32     = 0.0;
        	    let mut search_bsh       :[u8;32] = [0;32];
        	    let mut search_prevbsh   :[u8;32] = [0;32];
		    let mut search_completed :bool    = false;

	            if nchain_ts >= lchain_ts {
	                search_pos = nchain_pos - 1;
        	    } else {
          	        search_pos = lchain_pos - 1;
         	    }

		    while search_pos >= 0 && search_completed == false {

			if search_pos == 0 { search_completed = true; }

          	        search_bf         = self.index.blocks[search_pos].bf;
          	        search_bsh        = self.index.blocks[search_pos].bsh;
          	        search_prevbsh    = self.index.blocks[search_pos].prevbsh;

			//
			// we find the common ancestor
			//
		        if search_bsh == lchain_prevbsh && search_bsh == nchain_prevbsh {
			    shared_ancestor_pos = search_pos;
			    shared_ancestor_pos_found = true;
			    search_completed = true;
			//
			// or we keep looking
			//
			} else {

            		    if search_bsh == lchain_prevbsh {
            		        lchain_len = lchain_len + 1; 
            		        lchain_prevbsh = self.index.blocks[search_pos].prevbsh;
            		        lchain_bf = lchain_bf + self.index.blocks[search_pos].bf;
            		    }
            
            		    if search_bsh == nchain_prevbsh {
            		        nchain_prevbsh = self.index.blocks[search_pos].prevbsh;
            		        nchain_len = nchain_len + 1; 
            		        nchain_bf = nchain_bf + self.index.blocks[search_pos].bf;
            		    }
            
            		    shared_ancestor_pos = search_pos;
            		    search_pos = search_pos - 1;
            
            		    //
            		    // new chain completely disconnected
            		    // 
            		    if shared_ancestor_pos == 1 {
            		        if [0;32] == nchain_prevbsh {
				    //
				    // add the block, and escape from this
				    //
            		            //await this.addBlockToBlockchainSuccess(newblock, pos, 0);
				    println!("blockchain - block disconnected from chain");
            		            return;
            		        } 
            		    } 

            		    if shared_ancestor_pos == 0 {
            		        if lchain_prevbsh == nchain_prevbsh {
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

        	    //
        	    // at this point, we have a shared ancestor position for
        	    // our two possible chains, and we need to decide which
        	    // we are treating as the longest chain.
        	    //
        	    if nchain_len > lchain_len && nchain_bf >= lchain_bf && shared_ancestor_pos_found == true {

		        //
   		        // to prevent our system from being gamed, we
   		        // require the attacking chain to have equivalent
   		        // or greater aggregate burn fee. This ensures that
   		        // an attacker cannot lower difficulty, pump out a
   		        // ton of blocks, and then hike the difficulty only
   		        // at the last moment to claim the longest chain.
   		        //
   		        println!("UPDATING LONGEST CHAIN: no voter preference needed");
   		        i_am_the_longest_chain = 1;

   		    } else {

   		        //
   		        // to prevent our system from being gamed, we
   		        // require the attacking chain to have equivalent
   		        // or greater aggregate burn fee. This ensures that
   		        // an attacker cannot lower difficulty, pump out a
   		        // ton of blocks, and then hike the difficulty only
   		        // at the last moment to claim the longest chain.
   		        //
   		        // this is like the option above, except that we
   		        // have a choice of which block to support.
   		        //
   		        if nchain_len == lchain_len && nchain_bf >= lchain_bf && shared_ancestor_pos_found == true {

			    //
			    // TODO - allow voter preference 
			    //
   		            //if (this.app.voter.prefersBlock(newblock, this.returnLatestBlock())) {
   		        	println!("UPDATING LONGEST CHAIN: no voter preference needed");
   		                i_am_the_longest_chain = 1;
   		            //}
   		        }
        	    }
	    	}
	    } else {

      	        println!("blockchain -- add block -- edge case with unordered blocks");

	        //
	        // this catches an edge case that happens if we ask for blocks starting from
	        // id = 132, but the first block we RECEIVE is a later block in that chain,
	        // such as 135 or so.
	        //
      		//
      		// in this case our blockchain class will treat the first block as the starting
      		// point and we run into issues unless we explicitly reset the blockchain to
      		// treat block 132 as the proper first block.
      		//
      		// so we reset this to our first block and mark it as part of the longest chain
      		// the network will figure this out in time as further blocks build on it.
      		//
      		if blk.body.prevbsh == self.last_bsh && blk.body.prevbsh != [0;32] {

		    //
	            // reset later blocks
		    //
       	 	    for h in (pos+1)..self.index.blocks.len() {

println!("resetting blockchain block off LC at: {:?}", h);
println!("last to reset is: {:?}", self.index.blocks.len());

			//
			// reset LC hashmap
			//
        		self.bsh_lc_hmap.insert(self.index.blocks[h].bsh, i_am_the_longest_chain);

			//
			// TODO - onChainReorganization
			//
			// - storage ? - wallet ? - modules ?
			//
			//self.onChainReorganization(self.index.blocks[h].body.id, self.index.blocks[h].body.return_bsh(), 0);

        	    }

        	    i_am_the_longest_chain = 1;
		}
	    }
	}


	//
	// insert into LC hashmap
	//
        self.bsh_lc_hmap.insert(self.index.blocks[pos].bsh, i_am_the_longest_chain);


	//
	// update blockchain state variables depending
	//
	if i_am_the_longest_chain == 1 {

	    self.last_bsh  = self.index.blocks[pos].bsh;
	    self.last_ts   = self.index.blocks[pos].ts;
	    self.last_bid  = self.index.blocks[pos].bid;
  	    self.lc_pos = pos;
	    self.lc_pos_set = true;

        }


	//
	// old and new chains
	//
	let mut shared_ancestor_bsh  :[u8;32];
	let mut new_hash_to_hunt_for :[u8;32];
	let mut old_hash_to_hunt_for :[u8;32];
	let mut new_block_hashes     :Vec<[u8;32]>;
	let mut new_block_idxs       :Vec<usize>;
	let mut new_block_ids        :Vec<u32>;
	let mut old_block_hashes     :Vec<[u8;32]>;
	let mut old_block_idxs       :Vec<usize>;
	let mut old_block_ids        :Vec<u32>;


	//
	// POTENTIAL LONGEST CHAIN
	//
	// TODO
	//
	// unclear about the appropriateness of sending the blockchain
	// to addBlockToBLockchain success in this manner. We are simplifying
	// so need to check if this is still needed.
	//
  	// the first block goes directly to addBlockToBlockchainSuccess
  	// in order to avoid it getting inserted into the database with
  	// longest_chain of 0. This is only an issue with the first
  	// block.
  	//
	if i_am_the_longest_chain == 1 && self.index.blocks.len() > 1 {

	    shared_ancestor_bsh  = self.index.blocks[shared_ancestor_pos].bsh;
	    new_hash_to_hunt_for = blk.return_bsh();
	    old_hash_to_hunt_for = self.index.blocks[last_lc_pos].bsh;
	    new_block_hashes     = vec![];
	    new_block_idxs       = vec![];
	    new_block_ids        = vec![];
    	    old_block_idxs       = vec![];
    	    old_block_ids        = vec![];
	    old_block_hashes     = vec![];

	    //
	    // our block builds on the longest chain
	    //
	    if blk.body.prevbsh == old_hash_to_hunt_for {
	        new_block_hashes.push(self.index.blocks[pos].bsh);
	        new_block_ids.push(self.index.blocks[pos].bid);
	        new_block_idxs.push(pos);
	    }

	    //
	    // this is a chain reorganization
	    //
	    else {

	        for j in ((shared_ancestor_pos+1)..(self.index.blocks.len())).rev() {
	            if self.index.blocks[j].bsh == old_hash_to_hunt_for {
          		old_hash_to_hunt_for = self.index.blocks[j].prevbsh;
          		old_block_hashes.push(self.index.blocks[j].bsh);
          		old_block_ids.push(self.index.blocks[j].bid);
          		old_block_idxs.push(j);
        	    }
      		}

      		old_block_hashes.reverse();
      		old_block_idxs.reverse();

	        for j in ((shared_ancestor_pos+1)..(self.index.blocks.len())).rev() {
        	    if self.index.blocks[j].bsh == new_hash_to_hunt_for {
          		new_hash_to_hunt_for = self.index.blocks[j].prevbsh;
          		new_block_hashes.push(self.index.blocks[j].bsh);
          		new_block_ids.push(self.index.blocks[j].bid);
          		new_block_idxs.push(j);
        	    }
      		}

      		new_block_hashes.reverse();
      		new_block_idxs.reverse();

	    }
	}

	// add block to blockchain
	//self.validate_new_chain();

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
        if !self.lc_pos_set { return 0; }
	return self.index.blocks[self.lc_pos].ts;
    }

    pub fn return_latest_bsh(&self) -> [u8; 32] {
        if !self.lc_pos_set { return [0; 32]; }
	return self.index.blocks[self.lc_pos].bsh;
    }

    pub fn return_latest_bf_current(&self) -> f32 {
        if !self.lc_pos_set { return 0.0; }
	return self.index.blocks[self.lc_pos].bf;
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
    blocks:      Vec<BlockHeader>,                  // blocks
    //blocks:      Vec<Block>,                  // blocks
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






