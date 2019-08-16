use serde::{Serialize, Deserialize};


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
    pub bf:  f32,
    pub bsh: [u8;32],
    pub prevbsh: [u8;32],
    pub bid: u32,
    pub ts:  u64,
}

impl BlockHeader {
    pub fn new(bf: f32, bsh: [u8;32], prevbsh: [u8;32], bid: u32, ts: u64) -> BlockHeader {
        return BlockHeader { bf, bsh, prevbsh, bid, ts };
    }
}

