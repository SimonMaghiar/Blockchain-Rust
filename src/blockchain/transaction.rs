use secp256k1::{Message, PublicKey, Secp256k1};
use secp256k1::ecdsa::Signature;
use sha2::{Digest, Sha256};

use crate::wallets::Wallet;
use crate::Blockchain;

#[derive(Debug)]
pub struct Transaction {
    pub from: PublicKey, 
    pub to: PublicKey, 
    pub amount: f64,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn new(from: PublicKey, to: PublicKey, amount: f64) -> Self {
        Transaction {
            from, 
            to, 
            amount,
            signature: None,
        }
    }

    pub fn sign(&mut self, wallet: &Wallet) {
        if wallet.public_key == self.from {
            let secp = Secp256k1::new();
            let message = self.create_message();
            let message_hash = Sha256::digest(&message); // Create a SHA256 hash of the message
            let message = Message::from_digest_slice(&message_hash).expect("32 bytes");
            self.signature = Some(secp.sign_ecdsa(&message, &wallet.private_key));
        }
    }

    pub fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.amount.to_le_bytes());
        bytes.extend(&self.from.serialize_uncompressed());
        bytes.extend(&self.to.serialize_uncompressed());
        bytes
    }

    pub fn is_valid(&self, chain: &Blockchain) -> bool {
        if self.amount < 0.0 || chain.get_balance(self.from) < self.amount || self.signature.is_none() || !self.verify() {
            return false;
        }
        true
    }

    pub fn verify(&self) -> bool {
        if let Some(signature) = self.signature {
            let secp = Secp256k1::new();
            let message = self.create_message();
            let message_hash = Sha256::digest(&message); // Create a SHA256 hash of the message
            let message = Message::from_slice(&message_hash).expect("32 bytes");
            return secp.verify_ecdsa(&message, &signature, &self.from).is_ok();
        }
        false // No signature or error occurred
    }
}
