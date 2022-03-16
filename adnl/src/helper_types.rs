use aes::Aes256;
use ctr::Ctr128BE;
use sha2::{Sha256, Digest};

pub struct IntegrityError;

pub type AdnlAes = Ctr128BE<Aes256>;

pub struct AdnlAddress([u8; 32]);

pub struct AdnlPublicKey([u8; 32]);

pub struct AdnlSecret([u8; 32]);

pub struct AdnlAesParams {
    rx_key: [u8; 32],
    tx_key: [u8; 32],
    rx_nonce: [u8; 16],
    tx_nonce: [u8; 16],
    padding: [u8; 64],
}

impl From<[u8; 160]> for AdnlAesParams {
    fn from(raw_buffer: [u8; 160]) -> Self {
        Self {
            rx_key: raw_buffer[..32].try_into().unwrap(),
            tx_key: raw_buffer[32..64].try_into().unwrap(),
            rx_nonce: raw_buffer[64..80].try_into().unwrap(),
            tx_nonce: raw_buffer[80..96].try_into().unwrap(),
            padding: raw_buffer[96..160].try_into().unwrap(),
        }
    }
}

impl AdnlAesParams {
    #[cfg(feature = "std")]
    pub fn generate() -> Self {
        Self::from(rand::random::<[u8; 160]>())
    }

    pub fn rx_key(&self) -> &[u8; 32] {
        &self.rx_key
    }

    pub fn tx_key(&self) -> &[u8; 32] {
        &self.tx_key
    }

    pub fn rx_nonce(&self) -> &[u8; 16] {
        &self.rx_nonce
    }

    pub fn tx_nonce(&self) -> &[u8; 16] {
        &self.tx_nonce
    }

    pub fn to_bytes(&self) -> [u8; 160] {
        let mut result = [0u8; 160];
        result[..32].copy_from_slice(&self.rx_key);
        result[32..64].copy_from_slice(&self.tx_key);
        result[64..80].copy_from_slice(&self.rx_nonce);
        result[80..96].copy_from_slice(&self.tx_nonce);
        result[96..160].copy_from_slice(&self.padding);
        result
    }
}

impl From<[u8; 32]> for AdnlSecret {
    fn from(secret: [u8; 32]) -> Self {
        Self(secret)
    }
}

impl From<[u8; 32]> for AdnlPublicKey {
    fn from(public_key: [u8; 32]) -> Self {
        Self(public_key)
    }
}

impl From<AdnlPublicKey> for AdnlAddress {
    fn from(public_key: AdnlPublicKey) -> Self {
        let mut hasher = Sha256::new();
        hasher.update([0xc6, 0xb4, 0x13, 0x48]);  // type id - always ed25519
        hasher.update(public_key.0);
        Self(hasher.finalize().try_into().unwrap())
    }
}

impl AdnlAddress {
    #[inline]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AdnlPublicKey {
    #[inline]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AdnlSecret {
    #[inline]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}