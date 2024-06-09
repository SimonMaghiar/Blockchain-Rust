mod wallets;
mod blockchain;

use wallets::{Wallet, JOHN_PRIVATE_KEY, JENIFER_PRIVATE_KEY, MINER_PRIVATE_KEY};
use blockchain::{Blockchain, Transaction};

fn main() {
    let john_wallet = Wallet::new(JOHN_PRIVATE_KEY);
    let jenifer_wallet = Wallet::new(JENIFER_PRIVATE_KEY);
    let miner_wallet = Wallet::new(MINER_PRIVATE_KEY);

    let mut blockchain = Blockchain::new();
    let mut transaction = Transaction::new(john_wallet.public_key, jenifer_wallet.public_key, 20.0);
    transaction.sign(&john_wallet);
    blockchain.add_transaction(transaction);
    blockchain.mine_transactions(miner_wallet.public_key);

    println!("{:#?}", blockchain);
    println!("John's balance: {}", blockchain.get_balance(john_wallet.public_key));
    println!("Is blockchain valid: {}", blockchain.is_valid());
}
