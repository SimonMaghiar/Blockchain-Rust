use sha2::{Digest, Sha256};
use secp256k1::{Message, PublicKey, Secp256k1};
use secp256k1::ecdsa::Signature;
use wallets::WALLET;
use std::{fmt::Write};
use chrono::prelude::*;
mod keypair;
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
        let mut transaction = Transaction {
            from, 
            to, 
            amount,
            signature: None
        };
        return transaction;
    }
    fn sign(&mut self, wallet: &WALLET) {
        if wallet.public_key == self.from {
            let secp = Secp256k1::new();
            let message = Message::from_digest_slice(&[0xab; 32]).expect("32 bytes");
            self.signature = Some(secp.sign_ecdsa(&message, &wallet.private_key));
        }
    }
    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.amount.to_le_bytes());
        println!("{:?}", bytes);
        return bytes;
    }

    // fn verify(&self, public_key: &PublicKey) -> bool {
    //     let context = Secp256k1::new();
    //     let message = self.create_message();
    //     let signature = Signature::from_str(&self.signature).unwrap();
    //     context.verify(&Message::from_slice(&message).unwrap(), &signature, public_key).is_ok()
    // }
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
        return Blockchain {
            chain: vec![Block::new(vec![])],
            transactions: vec![],
            difficulty: 2, 
            block_time: 5000,
            reward: 10.0
        }
    }
    fn add_transaction (&mut self, trasaction: Transaction) {
        self.transactions.push(trasaction);
    }
    fn mine_transactions (&mut self, miner_reward_address: PublicKey) {
        let mint_public_key = PublicKey::from_slice(&[0x02,0xc6, 0x6e, 0x7d, 0x89, 0x66, 0xb5, 0xc5, 0x55,0xaf, 0x58, 0x05, 0x98, 0x9d, 0xa9, 0xfb, 0xf8,0xdb, 0x95, 0xe1, 0x56, 0x31, 0xce, 0x35, 0x8c,0x3a, 0x17, 0x10, 0xc9, 0x62, 0x67, 0x90, 0x63]).expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");
        let reward_transaction = Transaction::new(mint_public_key, miner_reward_address, self.reward);
        let mut final_transactions = vec![reward_transaction];
        final_transactions.append(&mut self.transactions);
        self.add_block(Block::new(final_transactions));
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
    let john_wallet: WALLET = WALLET::new();
    let jenifer_wallet: WALLET = WALLET::new();
    let miner_wallet: WALLET = WALLET::new();
    
    let mut blockchain = Blockchain::new();
    let mut transaction = Transaction::new(john_wallet.public_key, jenifer_wallet.public_key, 100.0);
    transaction.sign(&john_wallet);
    // println!("{:#?}", transaction);
    blockchain.add_transaction(transaction);
    blockchain.mine_transactions(miner_wallet.public_key);
    // let mut block1 = Block::new("First block".to_string());
    // let mut block2 = Block::new("Second block".to_string());
    // let mut block3 = Block::new("Third block".to_string());

    // blockchain.add_block(block1);
    // blockchain.add_block(block2);
    // blockchain.add_block(block3);
    // blockchain.chain[1].data = "This is hacked!".to_string();
    println!("{:#?}", blockchain.chain);
    // println!("Is the blockchain valid: {}", blockchain.is_valid());
    // let (private_key, public_key) = keypair::generate_keypair().expect("Failed to generate key pair");
    // Print the keys
    // println!("Private key: {:?}", private_key);
    // println!("Public key: {:?}", public_key);
}
