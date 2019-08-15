use std::path::Path;
use std::fs::{File};
use std::io::prelude::*;
use std::str;

use saito_primitives::block::Block;
use saito_primitives::helper::create_timestamp;

pub struct Storage {
    pub dest: String,
    pub blocks_dir: String,
}

impl Storage {
    pub fn new() -> Storage {
        return Storage {
            dest: String::from("data"),
            blocks_dir: String::from("./data/blocks"),
        }
    }

    pub fn write_block_to_disk(&self, blk: Block) {
        let mut filename = self.blocks_dir.clone();
 
        filename.push_str(&create_timestamp().to_string());
        filename.push_str(&"-");
        filename.push_str(str::from_utf8(&blk.return_bsh()).unwrap());
        filename.push_str(&".sai");

        let encode: Vec<u8> = bincode::serialize(blk.return_body()).unwrap();
        let mut f = File::create(filename).unwrap();
        f.write_all(&encode[..]).unwrap();
    }

    pub fn read_block_from_disk(&self, path: &Path) -> Block {
        let mut encoded = Vec::<u8>::new();
        let mut r = File::open(path).expect("Could not find block at this location");

        r.read_to_end(&mut encoded).unwrap();
 
        return bincode::deserialize(&encoded[..]).unwrap();
    }
}
