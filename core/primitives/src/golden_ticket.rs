use serde::{Serialize, Deserialize};
use crate::{
    block::Block,
    crypto::PublicKey
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldenTicket {
    target: [u8; 32],
    vote: u8,
    random: [u8; 32],
    publickey: PublicKey
}

impl GoldenTicket {
    pub fn new(vote: u8, target: [u8; 32], random: [u8; 32], publickey: PublicKey) -> Self {
        return GoldenTicket{vote, target, random, publickey};
    }

    fn calculate_difficulty (&self, prevblk: &Block) -> f32 {
        return match self.vote {
            1 => prevblk.return_difficulty() + 0.01,
            _ => prevblk.return_difficulty() - 0.01
        }
    }

    fn calculate_paysplit (&self, prevblk: &Block) -> f32 {
        return match self.vote {
            1 => prevblk.return_paysplit() + 0.01,
            _ => prevblk.return_paysplit() - 0.01
        }
    }
}
