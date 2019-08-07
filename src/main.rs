use saito_core::block::Block;
use saito_core::transaction::{Transaction};
use saito_core::slip::Slip;
use saito_core::blockchain::Blockchain;
use saito_core::crypto::generate_keys;

fn main() {

    let mut blockchain: Blockchain = Blockchain::new();
    let mut tx: Transaction = Transaction::new();

    let (_secret_key, public_key) = generate_keys();
    let mut block: Block = Block::new([0; 32], public_key);


    println!("{:?}", tx);

    let slip: Slip = Slip::new(public_key);

    println!("{:?}", slip);

    tx.add_to_slip(slip);
    block.add_transaction(tx);

    blockchain.add_block(block);

}



