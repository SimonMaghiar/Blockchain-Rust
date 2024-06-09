pub mod block;
pub mod transaction;

pub use block::Block;
pub use transaction::Transaction;

use secp256k1::PublicKey;
use crate::wallets::{Wallet, JOHN_PRIVATE_KEY, JENIFER_PRIVATE_KEY, MINER_PRIVATE_KEY, MINT_PRIVATE_KEY, PRE_RELEASE_PRIVATE_KEY};

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    transactions: Vec<Transaction>,
    difficulty: usize,
    block_time: u32,
    reward: f64,
}

impl Blockchain {
    pub fn new() -> Self {
        // Initial Coin Release
        let pre_release_wallet = Wallet::new(PRE_RELEASE_PRIVATE_KEY);
        let mint_wallet = Wallet::new(MINT_PRIVATE_KEY);
        let john_wallet = Wallet::new(JOHN_PRIVATE_KEY);
        let initial_coin_release_mint = Transaction::new(pre_release_wallet.public_key, mint_wallet.public_key, 10000000000000.0);
        let initial_coin_release = Transaction::new(mint_wallet.public_key, john_wallet.public_key, 100.0);
        Blockchain {
            chain: vec![Block::new(vec![initial_coin_release, initial_coin_release_mint])],
            transactions: vec![],
            difficulty: 2, 
            block_time: 5000,
            reward: 10.0,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if transaction.is_valid(&self) {
            self.transactions.push(transaction);
        }
    }

    pub fn get_balance(&self, address: PublicKey) -> f64 {
        let mut balance: f64 = 0.0;
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.to == address {
                    balance += transaction.amount;
                }
                if transaction.from == address {
                    balance -= transaction.amount;
                }
            }
        }
        balance
    }

    pub fn mine_transactions(&mut self, miner_reward_address: PublicKey) {
        let mint_wallet = Wallet::new(MINT_PRIVATE_KEY);
        let mut reward_transaction = Transaction::new(mint_wallet.public_key, miner_reward_address, self.reward);
        reward_transaction.sign(&mint_wallet);

        let mut final_transactions = vec![reward_transaction];
        final_transactions.append(&mut self.transactions);
        if final_transactions.len() > 1 {
            self.add_block(Block::new(final_transactions));
        } 
        self.transactions.clear();
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.previous_hash = self.chain.last().unwrap().hash.clone();
        block.block_number = self.chain.last().unwrap().block_number + 1;
        block.mine(self.difficulty);
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            if current_block.hash != current_block.get_hash() || current_block.previous_hash != previous_block.hash || !current_block.has_valid_transactions(&self) {
                return false;
            }
        }
        true
    }
}
