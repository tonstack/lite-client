use crate::{AdnlAesParams, AdnlHandshake};
use crate::handshake::CryptoRandom;

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
    aes_options: AesOptions<'a>,
}

impl<'a> HandshakeBuilder<'a> {
    fn with_static_aes_params(aes_params: &'a AdnlAesParams) -> Self {
        Self {
            aes_options: AesOptions::StaticParams(aes_params)
        }
    }

    fn with_random_aes_params(rng: &'a dyn CryptoRandom) -> Self {
        Self {
            aes_options: AesOptions::RandomParams(rng)
        }
    }

    #[cfg(feature = "dalek")]
    fn perform_ecdh(sender_private: PrivateKey, receiver_public: PublicKey) -> AdnlHandshake {}

    fn use_static_ecdh(sender_public: AsPublicKey, receiver_address: AsAddress, ecdh_secret: AsSecret) -> AdnlHandshake {}
}
