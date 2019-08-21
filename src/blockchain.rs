use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use saito_primitives::block::{Block, BlockHeader};
use saito_primitives::burnfee::BurnFee;
use crate::storage::Storage;
use crate::shashmap::Shashmap;
use saito_primitives::helper::{create_timestamp};


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
    blocks:      Vec<BlockHeader>,              // blocks
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
    last_bf:			f32,

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
	    last_bf:		   0.0,

	    lowest_acceptable_ts:  0,
	    lowest_acceptable_bsh: [0; 32],
	    lowest_acceptable_bid: 0,

        };
    }


    pub fn add_block(&mut self, blk: Block, shashmap: &mut Shashmap) {

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
        let block_header_entry = BlockHeader::new(blk.body.bf.current, blk.return_bsh(), blk.body.prevbsh, blk.body.id, blk.body.ts);

	//
	// TODO - binary search on insert point
	//
	let pos: usize = self.index.blocks.len();
        self.bsh_bid_hmap.insert(blk.return_bsh(), blk.body.id);
        self.index.blocks.insert(pos, block_header_entry);



	////////////////////////////
	// identify longest chain //
	////////////////////////////

	let mut i_am_the_longest_chain: u8  = 0;
        let mut shared_ancestor_pos: usize  = 0;
        let mut shared_ancestor_pos_found: bool = false;


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
        	    let mut lchain_pos:       usize   = self.lc_pos;
        	    let mut nchain_pos:       usize   = pos;
        	    let mut lchain_len:       u32     = 0;
        	    let mut nchain_len:       u32     = 0;
        	    let mut lchain_bf:        f32     = self.index.blocks[lchain_pos].bf;
        	    let mut nchain_bf:        f32     = self.index.blocks[nchain_pos].bf;
        	    let mut lchain_ts:        u64     = self.index.blocks[lchain_pos].ts;
        	    let mut nchain_ts:        u64     = self.index.blocks[nchain_pos].ts;
        	    let mut lchain_prevbsh:   [u8;32] = self.index.blocks[lchain_pos].prevbsh;
        	    let mut nchain_prevbsh:   [u8;32] = self.index.blocks[nchain_pos].prevbsh;
	            let mut search_pos:       usize   = 0;
        	    let mut search_bf:        f32     = 0.0;
        	    let mut search_bsh:       [u8;32] = [0;32];
        	    let mut search_prevbsh:   [u8;32] = [0;32];
		    let mut search_completed: bool    = false;

	            if nchain_ts >= lchain_ts {
	                search_pos = nchain_pos - 1;
        	    } else {
          	        search_pos = lchain_pos - 1;
         	    }

		    while !search_completed {

			if search_pos == 0 { 
                            search_completed = true; 
                        }

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
            		    if search_pos > 0 { search_pos = search_pos - 1; }            
            
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
                                    return
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
println!("LC POS SET!");

        }

	//
	// old and new chains
	//
	let mut shared_ancestor_bsh:  [u8;32];
	let mut new_hash_to_hunt_for: [u8;32];
	let mut old_hash_to_hunt_for: Option<[u8;32]>;
	let mut new_block_hashes:     Vec<[u8;32]>;
	let mut new_block_idxs:       Vec<usize>;
	let mut new_block_ids:        Vec<u32>;
	let mut old_block_hashes:     Vec<[u8;32]>;
	let mut old_block_idxs:       Vec<usize>;
	let mut old_block_ids:        Vec<u32>;


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
        if i_am_the_longest_chain == 1 && self.index.blocks.len() == 1 {
            self.add_block_success(blk, pos, 1, 1);
            return;
        } 
        
	if i_am_the_longest_chain == 1 && self.index.blocks.len() > 1 {
           
            let old_hash: Option<[u8; 32]>;
            if last_lc_pos == pos { 
                old_hash = None; 
            } else {
                old_hash = Some(self.index.blocks[last_lc_pos].bsh);
            }

	    new_hash_to_hunt_for = blk.return_bsh();
	    old_hash_to_hunt_for = old_hash;
	    new_block_hashes     = vec![];
	    new_block_idxs       = vec![];
	    new_block_ids        = vec![];
    	    old_block_idxs       = vec![];
    	    old_block_ids        = vec![];
	    old_block_hashes     = vec![];

	    //
	    // our block builds on the longest chain
	    //
	    if blk.body.prevbsh == old_hash_to_hunt_for.unwrap() || None == old_hash_to_hunt_for {
	        println!("ADDING INFO TO NEW_BLOCK_INDEXES");
                new_block_hashes.push(self.index.blocks[pos].bsh);
	        new_block_ids.push(self.index.blocks[pos].bid);
	        new_block_idxs.push(pos);
	    }

	    //
	    // this is a chain reorganization
	    //
	    else {

	        for j in ((shared_ancestor_pos+1)..(self.index.blocks.len())).rev() {
	            if self.index.blocks[j].bsh == old_hash_to_hunt_for.unwrap() {
          		old_hash_to_hunt_for = Some(self.index.blocks[j].prevbsh);
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
	} else {

	    //
	    // initialize
	    //
            println!("we are not the longest chain (?)");
	    new_hash_to_hunt_for = [0;32];
	    old_hash_to_hunt_for = Some([0;32]);
	    new_block_hashes     = vec![];
	    new_block_idxs       = vec![];
	    new_block_ids        = vec![];
    	    old_block_idxs       = vec![];
    	    old_block_ids        = vec![];
	    old_block_hashes     = vec![];


	}



	// add block to blockchain
	self.validate(
	    blk,
            shashmap,
	    pos,
	    shared_ancestor_pos,
	    i_am_the_longest_chain,
	    new_block_hashes,
	    new_block_idxs,
	    new_block_ids,
	    old_block_hashes,
	    old_block_idxs,
	    old_block_ids
	);

    }


    //////////////////////////////////////////
    // validate block and unwind / wind txs //
    //////////////////////////////////////////
    pub fn validate(
	&mut self, 
	mut blk              :Block,
        shashmap             :&mut Shashmap,
	pos		     :usize,
	shared_ancestor_pos  :usize,
	i_am_the_longest_chain:u8,
	new_block_hashes     :Vec<[u8;32]>,
	new_block_idxs       :Vec<usize>,
	new_block_ids        :Vec<u32>,
	old_block_hashes     :Vec<[u8;32]>,
	old_block_idxs       :Vec<usize>,
	old_block_ids        :Vec<u32>
    ) {

	//
	// validate the block
	//
	if !self.validate_block(&blk) {
		//
		// TODO
		//
		// -- force is not added as argment to this function, fix?
		//
	    self.add_block_failure(blk, pos, i_am_the_longest_chain, 0);
	    return;
	}

	//
	// block has validated, so we insert to shashmaps
	//
        for tx in blk.body.txs.iter() { shashmap.insert_new_transaction(&tx); }


	let mut force: u8 = 0;


	//
	// unwind and wind
	//
	if old_block_hashes.len() > 0 {
	    let obhlen = old_block_hashes.len()-1;
	    self.unwind_chain(
	        blk,
                shashmap,
	        pos,
	        i_am_the_longest_chain,
	        shared_ancestor_pos,
	        new_block_hashes,
	        new_block_idxs,
	        new_block_ids,
	        old_block_hashes,
	        old_block_idxs,
	        old_block_ids,
  		force,
  		0,
  		obhlen,
	    ); 
	} else {
	    self.wind_chain(
	        blk,
                shashmap,
	        pos,
	        i_am_the_longest_chain,
	        shared_ancestor_pos,
	        new_block_hashes,
	        new_block_idxs,
	        new_block_ids,
	        old_block_hashes,
	        old_block_idxs,
	        old_block_ids,
  		force,
  		0,
  		0,
	    ); 
	}
    }



    pub fn unwind_chain(
	 &mut self,
	 mut blk	      :Block,
         shashmap             :&mut Shashmap,
	 pos		      :usize,
 	 mut i_am_the_longest_chain:u8,
	 shared_ancestor_pos  :usize,
	 new_block_hashes     :Vec<[u8;32]>,
	 new_block_idxs       :Vec<usize>,
	 new_block_ids        :Vec<u32>,
	 old_block_hashes     :Vec<[u8;32]>,
	 old_block_idxs       :Vec<usize>,
	 old_block_ids        :Vec<u32>,
	 mut force            :u8,
         mut resetting_flag   :u8,
         mut current_unwind_index :usize,
    ) {


        if old_block_hashes.len() > 0 {

	    //
	    // load old block or list of TX SLIPS
	    //
	    // await this.returnBlockByHash(old_block_hashes[current_unwind_index], 2);
	    // if block_does_not_exist {

		//
		// request missing block
		//

		//
		// exit, or rollback with last longest china
		//

	    //}

	    //
	    // block or data is legit, so run on_chain_reorganization
	    // this should update the LC index as well
	    // self.on_chain_reorganization(OLD_BLOCK_DATA);


	    // if we are the node that produced this block, we catch any transactions
	    // that were added to it. we want to add these transactions back into our
	    // mempool once the chain has been rewritten if their inputs are still
	    // valid.
	    //
	    //if (this.app.wallet.returnPublicKey() == blk.block.creator) {
      		//
      		// a block that we created is getting undone, so we push all of the
      		// transactions into a special queue that exists in our mempool for
      		// us to check once we have finished re-writing the chain.
      		//
      		//if (blk.transactions != null) {
      		//  for (let i = 0; i < blk.transactions.length; i++) {
      		//    console.log("RECOVERING TRANSACTIONS FROM PREVIOUS BLOCKS");
      		//    console.log(blk.transactions[i]);
      		//    this.app.mempool.recoverTransaction(blk.transactions[i]);
      		//  }
      		//}
    	    //}


	    //
	    // unspend in shashmap
	    //
            //for tx in blk.body.txs.iter() {
	    //    shashmap.spend_transaction(&tx, blk.body.id);
	    //}
	    //
	    //
	    // we either move on to our next block, or we hit
	    // the end of the chain of blocks to unspend and
	    // move on to wind the proposed new chain
	    //
	    if current_unwind_index == 0 {
	        self.wind_chain(
	            blk,
                    shashmap,
	            pos,
	            i_am_the_longest_chain,
	            shared_ancestor_pos,
	            new_block_hashes,
	            new_block_idxs,
	            new_block_ids,
	            old_block_hashes,
	            old_block_idxs,
	            old_block_ids,
  		    force,
  		    resetting_flag,
		    0
	        );
	    } else {
		self.unwind_chain(
	            blk,
                    shashmap,
	            pos,
	            i_am_the_longest_chain,
	            shared_ancestor_pos,
	            new_block_hashes,
	            new_block_idxs,
	            new_block_ids,
	            old_block_hashes,
	            old_block_idxs,
	            old_block_ids,
  		    force,
  		    resetting_flag,
		    current_unwind_index-1
	        );
	    }

	} else {

	    //
	    // no more blocks to unwind
	    //
	    self.unwind_chain(
	        blk,
                shashmap,
	        pos,
	        i_am_the_longest_chain,
	        shared_ancestor_pos,
	        new_block_hashes,
	        new_block_idxs,
	        new_block_ids,
	        old_block_hashes,
	        old_block_idxs,
	        old_block_ids,
  	        force,
  	        resetting_flag,
	        0
	    );

	}

    } // end of unwind_chain



    pub fn wind_chain(
	&mut self,
	mut blk		     :Block,
        shashmap             :&mut Shashmap,
        pos		     :usize,
        i_am_the_longest_chain:u8,
        shared_ancestor_pos  :usize,
        new_block_hashes     :Vec<[u8;32]>,
        new_block_idxs       :Vec<usize>,
        new_block_ids        :Vec<u32>,
        old_block_hashes     :Vec<[u8;32]>,
        old_block_idxs       :Vec<usize>,
        old_block_ids        :Vec<u32>,
	mut force            :u8,
       	mut resetting_flag   :u8,
        mut current_wind_index :usize,
    ) {

        println!("CURRENT WIND INDEX: {}", current_wind_index);
	let this_block_hash = new_block_hashes[current_wind_index];

  	//
  	// we have not saved the latest block to disk yet, so
  	// there's no need to go through the delay of opening
  	// files from disk.
  	//
  	if this_block_hash == blk.return_bsh() {

            if self.validate_block(&blk) {

                //
      	 	// we do not handle onChainReorganization for everything
      		// here as we do for older blocks. the reason for this is
      		// that the block is not yet saved to disk.
      		//
      		// onChainReorganization is run on addBlockToBlockchainSuccess
      		//
	        // spend in shashmap
	        //
                //for tx in blk.body.txs.iter() {
	        //    shashmap.spend_transaction(&tx, blk.body.id);
	        //}

	        //
      	        self.add_block_success(blk, pos, i_am_the_longest_chain, force);
		//
                return;

            } else {

                if current_wind_index == 0 {

            	    // this is the first block we have tried to add
            	    // and so we can just roll out the older chain
            	    // again as it is known good.
            	    //
            	    // note that old and new hashes are swapped
            	    // and the old chain is set as null because
            	    // we won't move back to it. we also set the
            	    // resetting_flag to 1 so we know to fork
            	    // into add_block_failure
            	    //
	    	    if old_block_hashes.len() > 0 {
                        self.wind_chain(
                    	    blk,
                    	    shashmap,
                    	    pos,
                    	    i_am_the_longest_chain,
                    	    shared_ancestor_pos,
                    	    new_block_hashes,
                    	    new_block_idxs,
                    	    new_block_ids,
                    	    old_block_hashes,
                    	    old_block_idxs,
                    	    old_block_ids,
                    	    force,
                    	    1,
                    	    0
                        );
		        return;
	    	    } else {
		        self.add_block_failure(blk, pos, i_am_the_longest_chain, force);
		        return;
	    	    }
	    	} else {

            	    //
            	    // we need to unwind some of our previously
            	    // added blocks from the new chain. so we
            	    // swap our hashes to wind/unwind.
            	    //
            	    let mut chain_to_unwind_hashes :Vec<[u8;32]> = vec![];
            	    let mut chain_to_unwind_idxs   :Vec<usize>   = vec![];
            	    let mut chain_to_unwind_ids    :Vec<u32>     = vec![];

	    	    // 
	    	    // TODO 
	    	    // 
	    	    // vector splicing would be easier
	    	    // 
       	    	    for h in (current_wind_index)..new_block_hashes.len() {
	    	        chain_to_unwind_hashes.push(new_block_hashes[h]);
	    	    }
       	    	    for h in (current_wind_index)..new_block_hashes.len() {
	    	        chain_to_unwind_idxs.push(new_block_idxs[h]);
	    	    }
       	    	    for h in (current_wind_index)..new_block_hashes.len() {
	    	        chain_to_unwind_ids.push(new_block_ids[h]);
	    	    }

            	    //
            	    // unwind NEW and wind OLD
            	    //
            	    // note that we are setting the resetting_flag to 1
            	    //
		    let ctulen = chain_to_unwind_hashes.len();
            	    self.unwind_chain(
	            	blk,
                    	shashmap,
	            	pos,
	            	i_am_the_longest_chain,
	            	shared_ancestor_pos,
	            	old_block_hashes,
	            	old_block_idxs,
	            	old_block_ids,
	            	chain_to_unwind_hashes,
	            	chain_to_unwind_idxs,
	            	chain_to_unwind_ids,
  	            	force,
  	            	1,
	            	ctulen,
            	    );
		    return;
	        }
	    }

        } else {
 
	    // rename -- not blk because it
    	    //var blk = await this.returnBlockByHash(new_block_hashes[current_wind_index], 2);
            //if (blk == null) {

            self.add_block_failure(blk, pos, i_am_the_longest_chain, force);
            return;

        }


        if self.validate_block(&blk) {

	    //
	    // on chain reorganization
	    //
            //self.on_chain_reorganization();

            //
            // spend in shashmap
            //
            //for tx in blk.body.txs.iter() {
            //    shashmap.spend_transaction(&tx, blk.body.id);
            //}

      	    if current_wind_index == new_block_idxs.len()-1 {
        	if resetting_flag == 0 {
       	   	    self.add_block_success(blk, pos, i_am_the_longest_chain, force);
        	} else {
       	   	    self.add_block_failure(blk, pos, i_am_the_longest_chain, force);
        	}
      	    } else {

            	self.unwind_chain(
	            blk,
                    shashmap,
	            pos,
	            i_am_the_longest_chain,
	            shared_ancestor_pos,
	            new_block_hashes,
	            new_block_idxs,
	            new_block_ids,
	            old_block_hashes,
	            old_block_idxs,
	            old_block_ids,
  	            force,
  	            1,
	            current_wind_index+1,
            	);
		return;
	    }

       	    self.add_block_success(blk, pos, i_am_the_longest_chain, force);

	} else {

	    if current_wind_index == 0 {

                self.wind_chain(
	            blk,
                    shashmap,
	            pos,
	            i_am_the_longest_chain,
	            shared_ancestor_pos,
	            old_block_hashes,
	            old_block_idxs,
	            old_block_ids,
	            vec![],
	            vec![],
	            vec![],
  	            force,
  	            1,
	            0,
            	);

	    } else {

                //
                // we need to unwind some of our previously
                // added blocks from the new chain. so we
                // swap our hashes to wind/unwind.
                //
                let chain_to_unwind_hashes :Vec<[u8;32]> = vec![];
                let chain_to_unwind_idxs   :Vec<usize>   = vec![];
                let chain_to_unwind_ids    :Vec<u32>   = vec![];

                //
                // TODO
                //
                // vector splicing would be easier
                //
                for h in (current_wind_index)..new_block_hashes.len() {
                    chain_to_unwind_hashes.push(new_block_hashes[h]);
                }
                for h in (current_wind_index)..new_block_hashes.len() {
                    chain_to_unwind_idxs.push(new_block_idxs[h]);
                }
                for h in (current_wind_index)..new_block_hashes.len() {
                    chain_to_unwind_ids.push(new_block_ids[h]);
                }

                //
                // unwind NEW and wind OLD
                //
                // note that we are setting the resetting_flag to 1
                //
                self.unwind_chain(
                    blk,
                    shashmap,
                    pos,
                    i_am_the_longest_chain,
                    shared_ancestor_pos,
                    old_block_hashes,
                    old_block_idxs,
                    old_block_ids,
                    chain_to_unwind_hashes,
                    chain_to_unwind_idxs,
                    chain_to_unwind_ids,
                    force,
                    1,
                    chain_to_unwind_hashes.len(),
                );
	    }
	}
    }

    pub fn add_block_success(&mut self, blk: Block, pos: usize, i_am_the_longest_chain: u8, force: u8) {
	println!("SUCCESS ADDING BLOCK");
        Storage::write_block_to_disk(blk);
	println!("Adding block: {:?}", self.return_latest_block_header().bsh); 
	println!("lc: {:?}", i_am_the_longest_chain);
	println!("\n\n\n");
    }

    pub fn add_block_failure(&mut self, blk: Block, pos: usize, i_am_the_longest_chain: u8, force: u8) {
	println!("FAILURE ADDING BLOCK");
	println!("\n\n\n");
    }

    pub fn validate_block(&self, blk: &Block) -> bool {
        return true;
    }

    pub fn is_bsh_indexed(&mut self, bsh:[u8; 32] ) -> bool {
	if self.bsh_lc_hmap.contains_key(&bsh) {
	    return true;
	} else {
	    return false;
	}
    }

    pub fn return_latest_block_header(&mut self) -> BlockHeader {
        if !self.lc_pos_set { 
	    return BlockHeader::new(0.0, [0;32], [0;32], 0, 0);
	}
	return self.index.blocks[self.lc_pos].clone();
    }

    pub fn return_index_length(&self) -> usize {
        return self.index.blocks.len();
    }

    pub fn return_heartbeat(&self) -> u64 {
        return 100_000;
    }

}

//pub struct BlockHeader {
//    pub bf:  f32,
//    pub bsh: [u8;32],
//    pub prevbsh: [u8;32],
//    pub bid: u32,
//    pub ts:  u64,

#[cfg(test)]
mod test {
    use super::*;
    use saito_primitives::slip::{Slip};
    use saito_primitives::transaction::{Transaction};
    use saito_primitives::crypto::generate_keys;

    use std::{thread,time};
    
    #[test]
    fn test_add_block() {
        let (_, publickey) = generate_keys();

        let mut blockchain = Blockchain::new();
        let mut shashmap = Shashmap::new();

        let mut blk = Block::new(publickey, [0; 32]);
        blk.body.id = 1; 
        blk.is_valid = 1;

        let blk_header = blk.header();
        blockchain.add_block(blk, &mut shashmap);

        assert_eq!(blockchain.return_latest_block_header(), blk_header);
    }

    #[test]
    fn test_validate_chain() {
        // assert that the shashmap has all of the inputs provided in the blocks
        let (_, publickey) = generate_keys();

        let mut blockchain = Blockchain::new();
        let mut shashmap = Shashmap::new();

        let mut blk = Block::new(publickey, [0; 32]);

        let mut tx: Transaction = Transaction::new();
        let mut slip: Slip = Slip::new(publickey);
        
        slip.set_amt(200_000_000);

        tx.add_from_slip(slip.clone());
        
        let mut transactions = vec![tx];

        blk.set_transactions(&mut transactions);

        blk.body.id = 1;
        blk.is_valid = 1;

        blockchain.add_block(blk, &mut shashmap);
        
        assert_eq!(shashmap.return_value(slip.return_signature_source()), Some(&1));
    }

    #[test]
    fn test_wind_unwind_chain() {
        let (_, publickey) = generate_keys();

        let mut blockchain = Blockchain::new();
        let mut shashmap = Shashmap::new();

        let mut blk1 = Block::new(publickey, [0; 32]);
        blk1.is_valid = 1;
        blk1.body.id = 1;

        let mut blk2 = Block::new(publickey, blk1.return_bsh());
        blk2.is_valid = 1;
        blk2.body.id = 2;

        let mut blk3 = Block::new(publickey, blk2.return_bsh());
        blk3.is_valid = 1;
        blk3.body.id = 3;

        // introduce our forks 
        
        let mut blk4 = Block::new(publickey, blk3.return_bsh());
        blk4.is_valid = 1;
        blk4.body.id = 4;
        
        thread::sleep(time::Duration::from_millis(1000));

        // create fork for 6 and 7
        let mut blk6 = Block::new(publickey, blk3.return_bsh());
        blk6.is_valid = 1;
        blk6.body.id = 4;

        let mut blk5 = Block::new(publickey, blk4.return_bsh());
        blk5.is_valid = 1;
        blk5.body.id = 5;

        thread::sleep(time::Duration::from_millis(1000));

        let mut blk7 = Block::new(publickey, blk6.return_bsh());
        blk7.is_valid = 1;
        blk7.body.id = 5;

        let mut blk8 = Block::new(publickey, blk7.return_bsh());
        blk8.is_valid = 1;
        blk8.body.id = 6;

        let blocks = vec![blk1, blk2, blk3, blk4, blk6, blk5, blk7, blk8];

        for blk in blocks {
            blockchain.add_block(blk, &mut shashmap);
        }

        assert_eq!(blockchain.return_index_length(), 9);
    }
}

