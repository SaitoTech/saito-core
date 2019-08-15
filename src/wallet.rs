use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use saito_primitives::slip::Slip;
use saito_primitives::transaction::Transaction;
use saito_primitives::crypto::{SecretKey, PublicKey, generate_keys};


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




