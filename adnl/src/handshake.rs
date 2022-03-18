use ctr::cipher::StreamCipher;
use aes::cipher::KeyIvInit;
use sha2::{Sha256, Digest};
use crate::{AdnlAddress, AdnlAesParams, AdnlPublicKey, AdnlSecret};
use crate::helper_types::AdnlAes;

// make channel
// - AES: aes_params or random generator
// - EC:
//   1. sender public key + receiver public key or address + secret
//   2. sender private key + receiver public key

pub trait CryptoRandom: rand::RngCore + rand::CryptoRng {}

enum AesOptions<'a> {
    StaticParams(&'a AdnlAesParams),
    RandomParams(&'a dyn CryptoRandom),
}

pub struct HandshakeBuilder<'a> {
    aes_options: AesOptions<'a>
}

impl<'a> HandshakeBuilder<'a> {
    fn perform_ecdh(sender_private: AsPrivateKey, receiver_public: AsPublicKey) {

    }

    fn use_static_ecdh(sender_public: AsPublicKey, receiver_address: AsAddress, ecdh_secret: AsSecret) {

    }
}

pub struct AdnlHandshake {
    receiver: AdnlAddress,
    sender: AdnlPublicKey,
    hash: [u8; 32],
    encrypted_params: [u8; 160],
}

impl AdnlHandshake {
    pub fn with_static_aes(aes_params: &AdnlAesParams) {

    }

    pub fn with_random_aes(rng: &dyn CryptoRandom) {

    }

    pub fn new(receiver: AdnlAddress, sender: AdnlPublicKey, secret: &AdnlSecret, aes_params: &AdnlAesParams) -> Self {
        let mut raw_params = aes_params.to_bytes();
        let mut hasher = Sha256::new();
        hasher.update(&raw_params);
        let hash: [u8; 32] = hasher.finalize().try_into().unwrap();

        let mut key = [0u8; 32];
        key[..16].copy_from_slice(&secret.as_bytes()[..16]);
        key[16..32].copy_from_slice(&hash[16..32]);
        let mut nonce = [0u8; 16];
        nonce[..4].copy_from_slice(&hash[..4]);
        nonce[4..16].copy_from_slice(&secret.as_bytes()[20..32]);
        let mut handshake_aes = AdnlAes::new(&key.into(), &nonce.into());
        handshake_aes.apply_keystream(&mut raw_params);

        Self {
            receiver,
            sender,
            hash,
            encrypted_params: raw_params,
        }
    }

    pub fn to_bytes(self) -> [u8; 256] {
        let mut packet = [0u8; 256];
        packet[..32].copy_from_slice(self.receiver.as_bytes());
        packet[32..64].copy_from_slice(self.sender.as_bytes());
        packet[64..96].copy_from_slice(&self.hash);
        packet[96..256].copy_from_slice(&self.encrypted_params);
        packet
    }
}