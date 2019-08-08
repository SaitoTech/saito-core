use std::collections::HashMap;


pub struct Shashmap {
    // might want i32 as to include the negative numbers for unspent tx to 
    hashmap: HashMap<Vec<u8>, u32>,
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


