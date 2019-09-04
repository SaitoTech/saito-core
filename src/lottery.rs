use saito_primitives::{
    block::Block,
    crypto::{hash, generate_random_data, PublicKey},
    transaction::{Transaction, TransactionBroadcastType},
    slip::Slip,
    golden_ticket::GoldenTicket,
};

use crate::wallet::Wallet;

use actix::*;

pub trait LotteryGame {
    fn play(&mut self, prevblk: &Block, wallet: &Wallet);
    fn generate_random_solution(&self, prevblk: &Block) -> [u8; 32];
    fn is_valid_solution(&self, random_solution: [u8; 32], prevblk: &Block) -> bool;
    fn find_winner(&self, solution: &[u8; 32], prevblk: &Block) -> PublicKey;
    fn create_gt_solution(&self, random_solution: [u8; 32], block_hash: [u8; 32], publickey: PublicKey) -> GoldenTicket;
}

pub struct Miner {
    pub active: bool,
    pub difficulty: f32,
}

impl LotteryGame for Miner {
    fn play(&mut self, prevblk: &Block, wallet: &Wallet) {
        while self.active {
            let solution = self.generate_random_solution(prevblk);

            if self.is_valid_solution(solution, prevblk) {
                self.active = false;

                let gt_solution = self.create_gt_solution(
                    solution,
                    prevblk.return_bsh(),
                    wallet.return_publickey()
                );

                // Find winning node
                let winning_tx_address = self.find_winner(&solution, &prevblk);

                // we need to calculate the fees that are gonna go in the slips here
                let paid_burn_fee = prevblk.return_paid_burnfee();

                // This is just inputs - outputs for all transactions in the block
                let total_fees_for_creator = prevblk.return_available_fees(&prevblk.return_creator());

                // get the fees available from our publickey
                let total_fees_in_block = prevblk.return_available_fees(&wallet.return_publickey());

                // calculate the amount the creator can take for themselves
                let creator_surplus = total_fees_for_creator - paid_burn_fee;

                // find the amount that will be divied out to miners and nodes
                let total_fees_for_miners_and_nodes = 
                    (total_fees_in_block - creator_surplus) + prevblk.return_coinbase();

                // Calculate shares
                let miner_share = (total_fees_for_miners_and_nodes as f32 * prevblk.return_paysplit()).round() as u64;
                let node_share  = total_fees_for_miners_and_nodes - miner_share;

                // create our golden ticket tx (au_tx)
                let mut golden_tx: Transaction = match wallet.create_transaction(
                    wallet.return_publickey(),
                    TransactionBroadcastType::GoldenTicket,
                    100_000,
                    0
                ) {
                    Some(tx) => tx,
                    None => Transaction::new(),
                };
                
                let mut miner_slip = Slip::new(wallet.return_publickey());
                miner_slip.set_amt(miner_share);

                let mut node_slip = Slip::new(winning_tx_address);
                node_slip.set_amt(node_share);
                
                golden_tx.add_to_slip(miner_slip);
                golden_tx.add_to_slip(node_slip);

                // sign TX
                golden_tx.set_sig(wallet.create_signature(golden_tx.return_signature_source().as_slice()));
            }

        }
    }

    fn generate_random_solution(&self, prevblk: &Block) -> [u8; 32] {
        let mut hashed_solution: [u8; 32] = [0; 32];
        hash(generate_random_data(), &mut hashed_solution);
        return hashed_solution;
    }

    fn is_valid_solution(&self, random_solution: [u8; 32], prevblk: &Block) -> bool {
         let difficulty = self.difficulty.round() as usize;

         let random_solution_slice = &random_solution[0..difficulty];
         let previous_hash_slice = &prevblk.return_bsh()[0..difficulty];

         // println!("RANDOM SOLUTION {}", random_solution_slice);
         // println!("PREVIOUS HASH SLICE {}", random_solution_slice);

         if random_solution_slice == previous_hash_slice {
             return true
         } else {
             return false;
         }
    }

    fn find_winner(&self, solution: &[u8; 32], prevblk: &Block) -> PublicKey {
        match prevblk.body.txs.first() {
            Some(tx) => {
                return tx.return_to_slips()
                    .first()
                    .unwrap()
                    .return_add();
            },
            None => {
                return PublicKey::from_slice(&[0; 32]).unwrap();
            }
        }
    }

    fn create_gt_solution(&self, random: [u8; 32], block_hash: [u8; 32], publickey: PublicKey) -> GoldenTicket {
        let mut vote: u8 = 0;
         
        return GoldenTicket {
            target: block_hash,
            vote: 1,
            random,
            publickey,
        }
    }
}

impl Miner {
    pub fn new() -> Miner {
        return Miner { active: true, difficulty: 2.0 } 
    }
}

pub struct Lottery<G: LotteryGame> {
    pub game: G, 
    pub target: Option<Block>,
}

//impl<G> Actor for Lottery<G> where G: LotteryGame {
//    type Context = Context<Self>;
//}
//
//impl<G> Handler<Block> for Lottery<G> where G: LotteryGame {
//    type Result = ();
//    fn handle(&mut self, msg: Block, _: &mut Context<Self>) {
//        self.target = Some(msg);
//    }
//}

impl<G> Lottery<G> where G: LotteryGame {
    pub fn new(game: G) -> Lottery<G> { 
        return Lottery { game, target: None }
    }

    pub fn init(&self, wallet: &Wallet) {
        match self.target {
            Some(prevblk) => self.game.play(&prevblk, wallet),
            None => {},
        } 
    }
}

