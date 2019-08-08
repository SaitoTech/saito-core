use saito_core::block::Block;
use saito_core::transaction::{Transaction};
use saito_core::slip::Slip;
use saito_core::crypto::generate_keys;
use saito_core::blockchain::Blockchain;
use saito_core::creator::Creator;
use saito_core::wallet::Wallet;
use saito_core::shashmap::Shashmap;

struct Saito {
    blockchain: Blockchain,
    creator: Creator,
    wallet: Wallet,
    shashmap: Shashmap,
}

impl Saito {
    pub fn new() -> Saito {
        return Saito {
            blockchain: Blockchain::new(),
            creator: Creator::new(),
            wallet: Wallet::new(),
            shashmap: Shashmap::new(),
        }
    }
        

    pub fn init(&self) {
        self.creator.bundle();
    }
}

fn main() {

    let mut saito = Saito::new();
    saito.init();

    let mut block: Block = Block::new(saito.wallet.return_publickey());
    let mut tx: Transaction = Transaction::new();
    let slip: Slip = Slip::new(saito.wallet.return_publickey());

    tx.add_to_slip(slip);
    block.add_transaction(tx);

    saito.creator.add_block(block);
}



