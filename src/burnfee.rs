use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BurnFee {
    pub start: f32,
    pub current: f32,
}

impl BurnFee {
    /// Returns the BurnFee used to calculate the work needed to produce a block
    ///
    /// * `start` - y-value at x = 0
    /// * `current` - y-value at x = 0 for next bloc
    pub fn new(start: f32, current: f32) -> BurnFee {
        return BurnFee {
	    start, 
	    current 
	};
    }


    /// returns the amount of work needed to produce a block given the timestamp of
    /// the previous block, the current timestamp, and the y-axis of the burn fee
    /// curve. This is used both in the creation of blocks (mempool) as well as 
    /// during block validation.
    ///
    /// * `prevts` - timestamp of previous block
    /// * `ts`     - candidate timestamp
    /// * `start`  - burn fee value (y-axis) for curve determination ("start")
    ///
    pub fn return_work_needed(&mut self, prevts: u64, ts: u64, start: u64, heartbeat: u64) -> u64 {

	let mut elapsed_time = ts - prevts;
        if elapsed_time == 0 { elapsed_time = 1; }
        if elapsed_time > (2000 * heartbeat) { return 0; }

        let elapsed_time_float     = elapsed_time as f64;
        let start_float            = start as f64;
        let work_needed_float: f64 = start_float / elapsed_time_float; 

	return work_needed_float.round() as u64;

    }
}

