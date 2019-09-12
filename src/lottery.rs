use saito_primitives::{
    block::Block,
    crypto::{hash, generate_random_data, PublicKey},
    transaction::{Transaction, TransactionBroadcastType},
    slip::Slip,
    golden_ticket::GoldenTicket,
};

use std::cell::RefMut;
use std::sync::{Arc, RwLock};

use crate::wallet::Wallet;
use crate::network::NetworkMessage;
use crate::types::BlockMessage;

use actix::*;

pub trait LotteryGame {
    fn play(&mut self, prevblk: &Block, wallet: &Arc<Wallet>, consensus_addr: &Recipient<NetworkMessage>);
    fn generate_random_solution(&self) -> [u8; 32];
    fn is_valid_solution(&self, random_solution: [u8; 32], prevblk: &Block) -> bool;
    fn find_winner(&self, solution: &[u8; 32], prevblk: &Block) -> PublicKey;
    fn create_gt_solution(&self, random_solution: [u8; 32], block_hash: [u8; 32], publickey: PublicKey) -> GoldenTicket;
}

#[derive(Clone)]
pub struct Miner {
    pub active: bool,
    pub difficulty: f32,
}

impl LotteryGame for Miner {
    fn play(&mut self, prevblk: &Block, wallet: &Arc<Wallet>, consensus_addr: &Recipient<NetworkMessage>) {
        self.active = true;
        while self.active {
            let solution = self.generate_random_solution();

            if self.is_valid_solution(solution, prevblk) {
                println!("WE FOUND A VALID SOLUTION");
                self.active = false;

                let gt_solution = self.create_gt_solution(
                    solution,
                    prevblk.return_bsh(),
                    wallet.return_publickey()
                );

                // Find winning node
                let winning_tx_address = self.find_winner(&solution, &prevblk);
                println!("WINNING_TX_ADDRESS: {:?}", winning_tx_address);

                // we need to calculate the fees that are gonna go in the slips here
                let paid_burn_fee = prevblk.return_paid_burnfee();
                println!("PAID BURN FEE: {:?}", paid_burn_fee);

                // This is just inputs - outputs for all transactions in the block
                let total_fees_for_creator = prevblk.return_available_fees(&prevblk.return_creator());
                println!("TOTAL FEES FOR CREATOR: {:?}", total_fees_for_creator);

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
                golden_tx.set_msg(bincode::serialize(&gt_solution).unwrap());

                // sign TX
                golden_tx.set_sig(wallet.create_signature(golden_tx.return_signature_source().as_slice()));

                consensus_addr.do_send(NetworkMessage::IncomingTransaction(golden_tx)).unwrap();
            }

            //let sleep_duration = time::Duration::from_millis(50);
            //thread::sleep(sleep_duration);

        }
    }

    fn generate_random_solution(&self) -> [u8; 32] {
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
                // the rich get richer 
                return prevblk.return_creator();
            }
        }
    }

    fn create_gt_solution(&self, random: [u8; 32], target: [u8; 32], publickey: PublicKey) -> GoldenTicket {
        return GoldenTicket::new(1, target, random, publickey) ;
    }
}

impl Miner {
    pub fn new() -> Miner {
        return Miner { active: true, difficulty: 2.0 } 
    }
}

#[derive(Clone)]
pub struct Lottery<G: LotteryGame> {
    pub game: G, 
    pub target: Option<Block>,
    pub wallet: Arc<Wallet>,
    pub consensus_addr: Recipient<NetworkMessage>,
}

impl<G> Actor for Lottery<G> where G: LotteryGame + 'static {
    type Context = Context<Self>;
}

impl<G> Handler<BlockMessage> for Lottery<G> where G: LotteryGame + 'static {
    type Result = ();
    fn handle(&mut self, msg: BlockMessage, _: &mut Context<Self>) {
        self.game.play(&msg.payload, &self.wallet, &self.consensus_addr.clone());
    }
}

impl<G> Lottery<G> where G: LotteryGame {
    pub fn new(game: G, wallet: Arc<Wallet>, consensus_addr: Recipient<NetworkMessage>) -> Lottery<G> { 
        return Lottery { game, target: None, wallet, consensus_addr }
    }

    pub fn init(&mut self) {
        loop {
            match &self.target {
                Some(prevblk) => self.game.play(&prevblk, &self.wallet, &self.consensus_addr.clone()),
                None => {},
            } 
        } 
    }
}



#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn find_golden_ticket() {
        let wallet = Wallet::new();

        let mut lottery = Lottery::new(Miner::new());
        let treasury: u64 = 286_810_000_000_000_00; 
        let coinbase = (treasury as f64 / 21500 as f64).round() as u64;
        
        let mut block = Block::new(wallet.return_publickey(), [0; 32]);
        block.set_coinbase(coinbase);
        lottery.target = Some(block);

        lottery.init(&wallet);

        assert_eq!(lottery.game.active, false);
    }

}
