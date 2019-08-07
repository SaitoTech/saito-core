use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BurnFee {
    start: f32,
    current: f32,
}

