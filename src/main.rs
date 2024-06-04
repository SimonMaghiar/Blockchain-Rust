use sha2::{Digest, Sha256};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use std::{fmt::Write, str::FromStr};
use chrono::prelude::*;

#[derive(Debug)]
struct Block {
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: i64
}

impl Block {

    fn new(data: String) -> Self {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            data,
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
        hasher.update(self.data.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.nonce.to_ne_bytes());
        let hash = hasher.finalize();
        let mut hash_str = String::new();
        for byte in hash {
            write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
        }
        return hash_str;
    }

    fn mine (&mut self, difficulty: usize) {
        let hash_prefix = "0".repeat(difficulty as usize);
        while !self.hash.starts_with(&hash_prefix) {
            self.nonce+=1;
            self.hash = self.get_hash();
        }
    }
}
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    blockTime: u32
}


impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new("Genesis Block".to_string());
        return Blockchain {
            chain: vec![genesis_block],
            difficulty: 2, 
            blockTime: 5000
        }
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

            if current_block.hash != current_block.get_hash() || current_block.previous_hash != previous_block.hash {
                return false;
            } 
        }
        return true;
    }
}

fn main() {
    let mut block1 = Block::new("First block".to_string());
    let mut block2 = Block::new("Second block".to_string());
    let mut block3 = Block::new("Third block".to_string());

    let mut blockchain = Blockchain::new();
    blockchain.add_block(block1);
    blockchain.add_block(block2);
    blockchain.add_block(block3);
    blockchain.chain[1].data = "This is hacked!".to_string();
    println!("{:#?}", blockchain);
    println!("Is the blockchain valid: {}", blockchain.is_valid());
}
