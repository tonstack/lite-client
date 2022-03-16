use ctr::cipher::StreamCipher;
use sha2::{Sha256, Digest};
use aes::cipher::KeyIvInit;
use crate::helper_types::AdnlAes;
use crate::{AdnlAesParams, IntegrityError};

pub struct AdnlReceiver {
    aes: AdnlAes
}

impl AdnlReceiver {
    pub fn new(aes_params: &AdnlAesParams) -> Self {
        Self {
            aes: AdnlAes::new(aes_params.rx_key().into(), aes_params.rx_nonce().into())
        }
    }

    pub fn receive_packet_length(&mut self, buffer: [u8; 4]) -> u32 {
        let mut buffer = buffer;
        self.aes.apply_keystream(&mut buffer);
        u32::from_le_bytes(buffer)
    }

    pub fn receive_packet<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], IntegrityError> {
        if buffer.len() < 32 {
            return Err(IntegrityError);
        }
        self.aes.apply_keystream(buffer);
        let (buffer, given_hash) = buffer.split_at_mut(buffer.len() - 32);
        let computed_hash = Sha256::digest(&buffer);
        if computed_hash.as_slice() != given_hash {
            return Err(IntegrityError);
        }
        let (_, buffer) = buffer.split_at_mut(32);
        Ok(buffer)
    }
}