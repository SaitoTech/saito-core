use serde::{Serialize, Deserialize};
use crate::helper::{create_timestamp};

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


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BurnFeeCalculator {
    fee: u64,
    heartbeat: u32,
    last_block_timestamp: u64,
    last_block_delta: u64
}

impl BurnFeeCalculator {
    pub fn new(self)-> BurnFeeCalculator {
        return BurnFeeCalculator {
            fee: 1_000_000_000,
            heartbeat: 10,
            last_block_timestamp: create_timestamp(),
            last_block_delta: 0
        }
    }

    pub fn calculate(&self, mut elapsed_time: u64) -> u64 {
        let elapsed_time_float = elapsed_time as f64 / 1000.0;
        let heart_beat_float = self.heartbeat as f64 * 2.0;

        if elapsed_time_float > heart_beat_float { return 0; }

        if elapsed_time == 0 { elapsed_time = 1; }

        let elapsed_time_float = elapsed_time as f64;
        return (self.fee as f64 / (elapsed_time_float / 100_000_000_000 as f64)).round() as u64;
    }

    pub fn return_current_burnfee(&self) -> u64 {
        return self.calculate(create_timestamp() - self.last_block_timestamp);
    }
}
