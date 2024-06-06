use sha2::{Digest, Sha256};
use secp256k1::{Message, PublicKey, Secp256k1};
use secp256k1::ecdsa::Signature;
use wallets::{Wallet, JOHN_PRIVATE_KEY, JENIFER_PRIVATE_KEY, MINER_PRIVATE_KEY, MINT_PRIVATE_KEY, PRE_RELEASE_PRIVATE_KEY};
use std::{fmt::Write};
use chrono::prelude::*;
mod wallets;


#[derive(Debug)]
struct Block {
    timestamp: i64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: i64
}

impl Block {
    fn new(data: Vec<Transaction>) -> Self {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            transactions: data,
            previous_hash: String::new(),
            hash: String::new(),
            nonce: 0
        };
        block.hash = Block::get_hash(&block);
        return block;
    }
    fn get_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.timestamp.to_string().as_bytes());
        // hasher.update(self.data.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.nonce.to_ne_bytes());
        let hash = hasher.finalize();
        let mut hash_str = String::new();
        for byte in hash {
            write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
        }
        return hash_str;
    }
    fn mine(&mut self, difficulty: usize) {
        let hash_prefix = "0".repeat(difficulty as usize);
        while !self.hash.starts_with(&hash_prefix) {
            self.nonce+=1;
            self.hash = self.get_hash();
        }
    }
    fn has_valid_transactions(&self, chain: &Blockchain) -> bool {
        for transaction in &self.transactions {
            if !transaction.is_valid(chain) {
                println!("Transaction that is not valid: {:?}", transaction);
                return false;
            }
        }
        return true;
    }
}
#[derive(Debug)]

struct Transaction {
    from: PublicKey, 
    to: PublicKey, 
    amount: f64,
    signature: Option<Signature>
}

impl Transaction {
    fn new (from: PublicKey, to: PublicKey, amount: f64) -> Self {
        let transaction = Transaction {
            from, 
            to, 
            amount,
            signature: None
        };
        return transaction;
    }
    fn sign(&mut self, wallet: &Wallet) {
        if wallet.public_key == self.from {
            let secp = Secp256k1::new();
            let message = self.create_message();
            let message_hash = Sha256::digest(&message); // Create a SHA256 hash of the message
            let message = Message::from_digest_slice(&message_hash).expect("32 bytes");
            self.signature = Some(secp.sign_ecdsa(&message, &wallet.private_key));
        }
    }
    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.amount.to_le_bytes());
        bytes.extend(&self.from.serialize_uncompressed());
        bytes.extend(&self.to.serialize_uncompressed());
        bytes
    }
    fn is_valid(&self, chain: &Blockchain) -> bool {
        if self.amount < 0.0 || chain.get_balance(self.from) < self.amount || self.signature == None || !self.verify() {
            return false;
        }
        return true;
    }
    fn verify(&self) -> bool {
        if let Some(signature) = self.signature {
            let secp = Secp256k1::new();
            let message = self.create_message();
            let message_hash = Sha256::digest(&message); // Create a SHA256 hash of the message
            let message = Message::from_digest_slice(&message_hash).expect("32 bytes");
            return secp.verify_ecdsa(&message, &signature, &self.from).is_ok();
        }
        return false; // No signature or error occurred
    }
}


#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    transactions: Vec<Transaction>,
    difficulty: usize,
    block_time: u32,
    reward: f64
}


impl Blockchain {
    fn new() -> Self {
        // Initial Coin Release
        let pre_release_wallet = Wallet::new(PRE_RELEASE_PRIVATE_KEY);
        let mint_wallet = Wallet::new(MINT_PRIVATE_KEY);
        let john_wallet = Wallet::new(JOHN_PRIVATE_KEY);
        let initial_coin_release_mint = Transaction::new(pre_release_wallet.public_key, mint_wallet.public_key, 10000000000000.0);
        let initial_coin_release = Transaction::new(mint_wallet.public_key, john_wallet.public_key, 100.0);
        return Blockchain {
            chain: vec![Block::new(vec![initial_coin_release, initial_coin_release_mint])],
            transactions: vec![],
            difficulty: 2, 
            block_time: 5000,
            reward: 10.0
        }
    }
    fn add_transaction (&mut self, trasaction: Transaction) {
        if trasaction.is_valid(&self) {
            self.transactions.push(trasaction);
        }
    }

    fn get_balance (&self, address: PublicKey) -> f64 {
        let mut balance: f64 = 0.0;
        self.chain.iter().for_each(|block| {
            block.transactions.iter().for_each(|transaction| {
                if transaction.to == address {
                    balance += transaction.amount;
                }
                if transaction.from == address {
                    balance -= transaction.amount;
                }
            });
        });
        return balance;
    }

    fn mine_transactions (&mut self, miner_reward_address: PublicKey) {
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

    fn add_block(&mut self, mut block: Block) {
        block.previous_hash = self.chain.last().unwrap().hash.clone();
        block.mine(self.difficulty);
        self.chain.push(block);
    }

    fn is_valid (&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            if current_block.hash != current_block.get_hash() || current_block.previous_hash != previous_block.hash || !current_block.has_valid_transactions(&self) {
                return false;
            } 
        }
        return true;
    }
}

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
