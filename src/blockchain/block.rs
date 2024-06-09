use crate::Transaction;
use crate::Blockchain;

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug)]
pub struct Block {
    pub timestamp: DateTime<Utc>,
    pub block_number: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: i64,
}

impl Block {
    pub fn new(data: Vec<Transaction>) -> Self {
        let mut block = Block {
            timestamp: Utc::now(),
            block_number: 0,
            transactions: data,
            previous_hash: String::new(),
            hash: String::new(),
            nonce: 0,
        };
        block.hash = Block::get_hash(&block);
        block
    }

    pub fn get_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.timestamp.to_string().as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.nonce.to_ne_bytes());
        let hash = hasher.finalize();
        let mut hash_str = String::new();
        for byte in hash {
            write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
        }
        hash_str
    }

    pub fn mine(&mut self, difficulty: usize) {
        let hash_prefix = "0".repeat(difficulty as usize);
        while !self.hash.starts_with(&hash_prefix) {
            self.nonce += 1;
            self.hash = self.get_hash();
        }
    }

    pub fn has_valid_transactions(&self, chain: &Blockchain) -> bool {
        for transaction in &self.transactions {
            if !transaction.is_valid(chain) {
                println!("Transaction that is not valid: {:?}", transaction);
                return false;
            }
        }
        true
    }
}
