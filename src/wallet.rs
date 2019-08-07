use serde::{Serialize, Deserialize};
use crate::slip::Slip;
use std::collections::HashMap;
use crate::crypto::generate_keys;


//
// SECRETKEY and PUBLICKEY ? referencing here? or just fetching from the cryptoclass?
//
use secp256k1::{SecretKey, PublicKey};



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
    version:     f32,
    pending:     Vec<String>,
}




impl Wallet {
    pub fn new() -> Wallet {

        return Wallet {
	    body:                        WalletBody::new(),
	    inputs_lc_hmap:              HashMap::new(),
	    outputs_lc_hmap:             HashMap::new(),
	    inputs_hmap_counter:         0,
	    outputs_hmap_counter:        0,
	    inputs_hmap_counter_limit:   10000,
	    outputs_hmap_counter_limit:  10000,
        };
    }
}

impl WalletBody {
    pub fn new() -> WalletBody {

        let (_secret_key, _public_key) = generate_keys();

        return WalletBody {
            balance:     0,
            privatekey:  _secret_key,
            publickey:   _public_key,
            inputs:      vec![],
            outputs:     vec![],
            default_fee: 2,
            version:     2.15,
            pending:     vec![],
        };
    }
}




