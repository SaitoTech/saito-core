use serde::{Serialize, Deserialize};
use crate::helper::create_timestamp;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BurnFee {
    start: f32,
    current: f32,
}

impl BurnFee {
    pub fn new(start: f32, current: f32) -> BurnFee {
        return BurnFee { start, current };
    }
}

// burnfee references the work needed to produce a block
// references
//
// work_available
// work_needed
// 
// fees_total
// fees_usable
// fees_usable_to_block_creator
//
// fees_paid_to_lottery
// fees_paid_to_block_creator

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BurnFeeCalculator {
    fee: u64, 
    heartbeat: u32,
}

impl BurnFeeCalculator {
    pub fn new() -> BurnFeeCalculator {
        return BurnFeeCalculator {
            fee: 1_000_000, 
            heartbeat: 10,
        }
    }

    pub fn calculate(&self, mut elapsed_time: u64) -> u64 {
        let elapsed_time_float = elapsed_time as f64 / 1000.0;
        let double_heart_beat_float = self.heartbeat as f64 * 2.0;

        if elapsed_time_float > double_heart_beat_float { return 0; }

        println!("ELAPSED TIME IN SECONDS {}", elapsed_time_float);

        if elapsed_time == 0 { elapsed_time = 1; }

        let elapsed_time_float = elapsed_time as f64;
        let fee_float: f64 = self.fee as f64 * 1000 as f64;
        let calculation =  fee_float / elapsed_time_float;

        let result = calculation.round() as u64;

        return result;
    }

    // return work needed
    //
    // validate work

    // need to adjust the current burnfee based on the previous blocks burnfee value
}
