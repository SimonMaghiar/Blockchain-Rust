use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub const JOHN_PRIVATE_KEY: &str = "c87509a1c067bbde78beb793e6faafc2647b4db9ac64b0d1e3aa1c6c0bbecf7c";
pub const JENIFER_PRIVATE_KEY: &str = "ae6ae8e5ccbfb04590405997ee2d52d2b330726137b875053c36d94e974d162f";
pub const MINER_PRIVATE_KEY: &str = "b60e8dd61c5d32be8058bbd42a9254f5f01b1b6cb7eaa1f63b9b22d5af77b726";
pub const MINT_PRIVATE_KEY: &str = "a14e8dd61c5d32be8058bbd42a9254f5f01b1b6cb7eaa1f63b9b22d5af1acf17";
pub const PRE_RELEASE_PRIVATE_KEY: &str = "b14e8dd61c5d32be8058bbd42a9254f5f01b1b6cb7eaa1f63b9b22d5af1acf17";

#[derive(Debug)]
pub struct Wallet {
    pub public_key: PublicKey,
    pub private_key: SecretKey,
}

impl Wallet {
    // Create a new wallet with the given private key
    pub fn new(private_key_hex: &str) -> Self {
        let secp = Secp256k1::new();
        // Convert the hex string to bytes
        let private_key_bytes = hex::decode(private_key_hex).expect("Hex decode failed");
        let private_key = SecretKey::from_slice(&private_key_bytes).expect("32 bytes");
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        
        Wallet {
            public_key,
            private_key,
        }
    }
}