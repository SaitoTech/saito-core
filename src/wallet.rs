use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use saito_primitives::slip::{Slip, SlipSpentStatus};
use saito_primitives::transaction::{Transaction, TransactionBroadcastType};
use saito_primitives::crypto::{SecretKey, PublicKey, Signature, generate_keys, hash, sign};


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

    fn return_privatekey(&self) -> SecretKey {
        return self.body.privatekey;
    }

    pub fn create_signature(&self, data: &[u8]) -> Signature {
        let mut hashed_data: [u8; 32] = [0; 32];
        hash(data.to_vec(), &mut hashed_data);
        return sign(&hashed_data, &self.return_privatekey());
    }

    pub fn create_transaction(
        &self,
        publickey: PublicKey,
        tx_type: TransactionBroadcastType,
        fee: u64,
        amt: u64,
    ) -> Option<Transaction> {
        let total = fee + amt;
        let from_slips = self.return_available_inputs(total);

        match from_slips {
            Some(slips) => {
                let from_amt: u64 = slips.iter().map(|slip| slip.return_amt()).sum();
                let to_recover_amt = from_amt - total;

                let mut tx = Transaction::new();
                tx.set_tx_type(tx_type);

                for from_slip in slips.iter() {
                    tx.add_from_slip(from_slip.clone());
                }
                
                let mut to_slip = Slip::new(publickey);
                to_slip.set_amt(to_recover_amt);

                tx.add_to_slip(to_slip);

                return Some(tx);
            },
            None => { return None; },     
        }
    }

    pub fn return_balance(&self) -> u64 {
        return self.body.slips
            .iter()
            .filter(|slip| slip.spent_status == SlipSpentStatus::Unspent)
            .map(|slip| slip.return_amt())
            .sum();
    }

    pub fn return_available_inputs(&self, amt: u64) -> Option<Vec<Slip>>{
        let mut slip_vec: Vec<Slip> = Vec::new();
        let mut slip_sum_amount: u64 = 0;

        for slip in self.body.slips.iter() {
            if slip.spent_status == SlipSpentStatus::Unspent {
                slip_sum_amount += slip.return_amt();
                slip_vec.push(slip.clone());
 
                if slip_sum_amount > amt {
                    return Some(slip_vec);
                }
            } 
        } 
        return None;
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




