use saito_core::block::Block;
use saito_core::transaction::{Transaction, TransactionType};
use saito_core::slip::Slip;
use saito_core::crypto::generate_keys;

fn main() {
    let (secret_key, public_key) = generate_keys();

    let mut block: Block = Block::new([0; 32], public_key);
    let mut tx: Transaction = Transaction::new(TransactionType::Base);
    let slip: Slip = Slip::new(public_key);

    tx.add_to_slip(slip);
    block.transactions.push(tx);

    println!("{:?}", block);
}
