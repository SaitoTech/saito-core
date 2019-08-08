use saito_core::block::Block;
use saito_core::transaction::{Transaction};
use saito_core::slip::Slip;
use saito_core::blockchain::Blockchain;
use saito_core::wallet::Wallet;
use saito_core::crypto::generate_keys;
use saito_core::shashmap::Shashmap;

struct Saito {
    blockchain: Blockchain,
    creator: Creator,
    wallet: Wallet,
}

impl Saito {
    pub fn new() {
        blockchain: Blockchain::new(),
        creator: Creator::new(),
        wallet: Wallet::new(),
    }

    pub fn init(&self) {
        self.creator.bundle();
    }
}

fn main() {

    let (_secret_key, public_key) = generate_keys();

    let mut block: Block = Block::new(public_key);

    let v1 = String::from("Testing");
    let v2 = String::from("Testing2");

    let mut shashmap: Shashmap = Shashmap::new();

    println!("{:?}", &v1);
    println!("{:?}", &v2);

    shashmap.insert(v1, 1313);
    shashmap.insert(v2, 2434);

    let xx = shashmap.return_value(String::from("Testing"));
    println!("{:?}", xx);

    let wallet: Wallet = Wallet::new();

    println!("{:?}", wallet);

    let mut blockchain: Blockchain = Blockchain::new();
    let mut tx: Transaction = Transaction::new();

    println!("{:?}", tx);

    let slip: Slip = Slip::new(public_key);

    println!("{:?}", slip);

    tx.add_to_slip(slip);
    block.add_transaction(tx);

    blockchain.add_block(block);

}



