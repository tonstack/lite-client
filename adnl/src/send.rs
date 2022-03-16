use ctr::cipher::StreamCipher;
use sha2::{Sha256, Digest};
use aes::cipher::KeyIvInit;
use crate::AdnlAesParams;
use crate::helper_types::AdnlAes;

pub struct AdnlSender {
    aes: AdnlAes,
}

impl AdnlSender {
    pub fn new(aes_params: &AdnlAesParams) -> Self {
        Self {
            aes: AdnlAes::new(aes_params.tx_key().into(), aes_params.tx_nonce().into())
        }
    }

    pub fn send_packet<'a>(&'a mut self, nonce: &'a mut [u8; 32], buffer: &'a mut [u8]) -> AdnlPacketBuilder<'a> {
        AdnlPacketBuilder::build(&mut self.aes, nonce, buffer)
    }
}

pub struct AdnlPacketBuilder<'a> {
    length: [u8; 4],
    nonce: &'a mut [u8; 32],
    buffer: &'a mut [u8],
    hash: [u8; 32],
}

impl<'a> AdnlPacketBuilder<'a> {
    pub fn length(&'a self) -> &'a [u8; 4] {
        &self.length
    }

    pub fn nonce(&'a self) -> &'a [u8; 32] {
        &self.nonce
    }

    pub fn buffer(&'a self) -> &'a [u8] {
        &self.buffer
    }

    pub fn hash(&'a self) -> &'a [u8; 32] {
        &self.hash
    }

    pub fn build(aes: &'a mut AdnlAes, nonce: &'a mut [u8; 32], buffer: &'a mut [u8]) -> Self {
        // remember not to send more than 4 GiB in a single packet
        let mut length = ((buffer.len() + 64) as u32).to_le_bytes();
        let mut hasher = Sha256::new();
        hasher.update(&*nonce);
        hasher.update(&*buffer);
        let mut hash: [u8; 32] = hasher.finalize().try_into().unwrap();
        aes.apply_keystream(&mut length);
        aes.apply_keystream(nonce);
        aes.apply_keystream(buffer);
        aes.apply_keystream(&mut hash);
        Self {
            length,
            nonce,
            buffer,
            hash,
        }
    }
}