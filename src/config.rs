use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};

use saito_primitives::slip:: Slip;
use saito_primitives::crypto::{ReadablePublicKey, ReadablePrivateKey};

pub const TREASURY: u64 = 286_810_000_000_000_00;
pub const GENESIS_PERIOD: u64 = 21500;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub config_filename: String,
    pub chain_config: ChainConfig,
    pub wallet_config: WalletConfig,
    pub network_config: NetworkConfig,
}


#[derive(Serialize, Deserialize)]
pub struct ChainConfig {}

#[derive(Serialize, Deserialize)]
pub struct WalletConfig {
    // assume both of these need to be strings in base58 to be ledgable for people 
    publickey: ReadablePublicKey,
    privatekey: ReadablePrivateKey,
    amount: u64,
    inputs: Vec<Slip>,
    outputs: Vec<Slip>
}

#[derive(Serialize, Deserialize)]
pub struct NetworkConfig {}

impl Config {
    pub fn read_from_file(path: &PathBuf) -> Config {
        let mut file = File::open(path).expect("Could not open config file.");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Could not read from config file.");
        Config::from(content.as_str())       
    }

    pub fn write_to_file(&self, path: &PathBuf) {
        let mut file = File::create(path).expect("Failed to create / write a config file.");
        let str = serde_json::to_string_pretty(self).expect("Error serializing the config.");
        if let Err(err) = file.write_all(str.as_bytes()) {
            panic!("Failed to write a config file {}", err);
        }
    }
}

impl From<&str> for Config {
    fn from(content: &str) -> Self {
        serde_json::from_str(content).expect("Failed to deserialize config")
    }
}
