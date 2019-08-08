use serde::{Serialize, Deserialize};
use crate::slip::Slip;
use std::collections::HashMap;
use crate::crypto::generate_keys;
use crate::transaction::Transaction;


//
// SECRETKEY and PUBLICKEY ? referencing here? or just fetching from the cryptoclass?
//
use secp256k1::{SecretKey, PublicKey};



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Wallet {
    body:                        WalletBody,
    slips_hmap:                  HashMap<[u8; 32], u8>,
    slips_limit:		 u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WalletBody {
    balance:     u64,
    privatekey:  SecretKey,
    publickey:   PublicKey,
    slips:       Vec<Slip>,
    default_fee: u64,
    version:     f32,
    pending:     Vec<Transaction>,
}




impl Wallet {
    pub fn new() -> Wallet {
        return Wallet {
	    body:                        WalletBody::new(),
	    slips_hmap:                  HashMap::new(),
	    slips_limit:                 10000,
        };
    }

    pub fn return_publickey(&self) -> PublicKey {
        return self.body.publickey;
    }

}

impl WalletBody {
    pub fn new() -> WalletBody {

        let (_privatekey, _publickey) = generate_keys();

        return WalletBody {
            balance:     0,
            privatekey:  _privatekey,
            publickey:   _publickey,
            slips:       vec![],
            default_fee: 200_000_000,
            version:     2.15,
            pending:     vec![],
        };
    }
}




