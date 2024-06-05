use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_keypair() -> Result<(SecretKey, PublicKey), secp256k1::Error> {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // Generate a random private key
    let mut private_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut private_key_bytes);
    let private_key = SecretKey::from_slice(&private_key_bytes)?;

    // Derive the corresponding public key
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    Ok((private_key, public_key))
}