use secp256k1::{SecretKey, PublicKey};
use crate::keypair;

pub struct WALLET {
    pub private_key: SecretKey,
    pub public_key: PublicKey
}

impl WALLET {
    pub fn new() -> Self  {
        let (private_key, public_key) = keypair::generate_keypair().expect("Failed to generate key pair");
        let mut wallet = WALLET {
            private_key,
            public_key
        };
        return wallet;
    }
}