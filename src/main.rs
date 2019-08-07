use saito_core::block::Block;
use saito_core::transaction::{Transaction};
use saito_core::slip::Slip;
use saito_core::crypto::generate_keys;

fn main() {
    let (_secret_key, public_key) = generate_keys();

    let mut block: Block = Block::new([0; 32], public_key);
    let mut tx: Transaction = Transaction::new();

    println!("{:?}", tx);

    let slip: Slip = Slip::new(public_key);

    println!("{:?}", slip);

    tx.add_to_slip(slip);
    block.transactions.push(tx);

    println!("{:?}", block);
}
