use std::collections::HashMap;
use saito_primitives::transaction::Transaction;


#[derive(Clone)]
pub struct Shashmap {
    hashmap: HashMap<Vec<u8>, i64>,
}

impl Shashmap {

    pub fn new() -> Shashmap {
        return Shashmap {
	    hashmap: HashMap::new() ,
        }
    }

    pub fn insert(&mut self, _x: Vec<u8>, _y: u32) {
        self.hashmap.insert(_x, _y as i64);
    }

    pub fn insert_new_transaction(&mut self, tx: &Transaction) {
	for to in tx.return_to_slips().iter() {
	    self.hashmap.insert(to.return_signature_source(), -1);
	}
	for from in tx.return_from_slips().iter() {
	    self.hashmap.insert(from.return_signature_source(), -1);
	}
    }

    pub fn spend_transaction(&mut self, tx: &Transaction, _bid: u32) {
	for to in tx.return_from_slips().iter() {
	    self.hashmap.insert(to.return_signature_source(), _bid as i64);
	}
    }

    pub fn unspend_transaction(&mut self, tx: &Transaction, _bid: u32) {
	for to in tx.return_to_slips().iter() {
	    self.hashmap.insert(to.return_signature_source(), -1);
	}
    }

    pub fn return_value(&self, slip_index: Vec<u8>) -> Option<&i64> {
        return self.hashmap.get(&slip_index);
    }

/***
    pub fn remove(&mut self, _x: String) {
        self.hashmap.remove(&_x);
    }
    pub fn validate_exists(&mut self, _x: String) -> u32 {
	return 1;
    }
    pub fn validate_unspent(&mut self, _x: String, _y: u32) -> u32 {
	return 1;
    }
***/

}


