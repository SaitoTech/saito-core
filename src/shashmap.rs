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
        self.hashmap.insert(_x, _y);
    }

    pub fn insert_new_transaction(&mut self, &_tx: Transaction) {
	for j in 0.._tx.body.to.len() {
	    self.hashmap.insert(_tx.body.to[j].return_signature_source(), -1);
	}
	for j in 0.._tx.body.from.len() {
	    self.hashmap.insert(_tx.body.from[j].return_signature_source(), -1);
	}
    }

    pub fn spend_transaction(&mut self, &_tx: Transaction, _bid: u32) {
	for j in 0.._tx.body.from.len() {
	    self.hashmap.insert(_tx.body.from[j].return_signature_source(), _bid as i64);
	}
    }

    pub fn unspend_transaction(&mut self, &_tx: Transaction, _bid: u32) {
	for j in 0.._tx.body.to.len() {
	    self.hashmap.insert(_tx.body.from[j].return_signature_source(), -1);
	}
    }

    pub fn return_value(&self, slip_index: Vec<u8>) -> Option<&u32> {
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


