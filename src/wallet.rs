use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::slip::Slip;

//
// SECRETKEY and PUBLICKEY ? referencing here? or just fetching from the cryptoclass?
//
use secp256k1::{Secp256k1, Message, Signature, SecretKey, PublicKey};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Wallet {
    body:                        WalletBody,

    inputs_lc_hmap:              HashMap<[u8; 32], u8>,
    inputs_hmap_counter:         u32,
    inputs_hmap_counter_limit:   u32,

    outputs_lc_hmap:             HashMap<[u8; 32], u8>,
    outputs_hmap_counter:        u32,
    outputs_hmap_counter_limit:  u32,

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WalletBody {
    balance:     u64,
    privatekey:  SecretKey,
    publickey:   PublicKey,
    inputs:      Vec<Slip>,
    outputs:     Vec<Slip>,
    default_fee: u64,
    version:     u32,
    pending:     Vec<String>,
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
            balance:     0.0,
            privatekey:  none,
            publickey:   none,
            inputs:      vec![],
            outputs:     vec![],
            default_fee: 2,
            version:     2.15,
            pending:     vec![],
        };
    }
}




