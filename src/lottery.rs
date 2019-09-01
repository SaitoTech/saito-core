use saito_primitives::{
    block::Block;
    crypto::hash;
};

pub trait LotteryGame {
    fn generate_random_solution(&self, prevblk: &Block) -> u32;
    fn is_valid_solution(&self, random_solution: &Vec<u8>, prevblk: &Block) -> u32;
}

pub struct Miner {
    pub active: bool
    pub difficulty: f32;
}

impl LotteryGame for Miner {
    fn generate_random_solution(&self, prevblk: &Block) -> u32 {
        let mut rng = thread_rng();
        let random_number = rng.gen::<u32>();
        let random_number_bytes: [u8; 4] = unsafe { transmute(random_number.to_be()) };
        
        let mut hashed_solution: [u8];

        hash(random_number_bytes.as_vec(), &hashed_solution);
    }

    fn is_valid_solution(&self, random_solution: &Vec<u8>, prevblk: &Block) -> bool {
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
}

pub struct Lottery<G: LotteryGame> {
    pub game: G 
}

impl<G> Lottery<G> {
    pub fn new(game: G) -> Lottery {
        return Lottery { game }
    }
}

