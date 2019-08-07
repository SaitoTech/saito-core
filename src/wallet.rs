use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::slip::Slip;


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Wallet {
    body:     WalletBody,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WalletBody {
}




impl Wallet {
    pub fn new() -> Wallet {
        return Wallet {
	    body:      WalletBody::new(),
        };
    }
}

impl WalletBody {
    pub fn new() -> WalletBody {
        return WalletBody {
	    body:      WalletBody::new(),
        };
    }
}




