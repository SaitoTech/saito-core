use hashbrown::HashMap;


pub struct Shashmap {
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

    pub fn return_value(&mut self, _x: String) -> u32 {
        let rv = self.hashmap.get(&_x);
	return Some(rv);
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


