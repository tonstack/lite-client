use aes::Aes256;
use ctr::cipher::StreamCipher;
use ctr::Ctr128LE;
use sha2::Sha256;

type AdnlAes = Ctr128LE<Aes256>;

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
        let hasher = Sha256::new();
        hasher.update([0xc6, 0xb4, 0x13, 0x48]);  // type id - always ed25519
        hasher.update(public_key.0);
        Self(hasher.finalize())
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

pub struct AdnlHandshake {
    receiver: AdnlAddress,
    sender: AdnlPublicKey,
    hash: [u8; 32],
    encrypted_params: [u8; 160],
}

impl AdnlHandshake {
    pub fn new(receiver: &AdnlAddress, sender: &AdnlPublicKey, secret: &AdnlSecret, aes_params: &AdnlAesParams) -> Self {
        let mut raw_params = aes_params.to_bytes();
        let hasher = Sha256::new();
        hasher.update(&raw_params);
        let hash = hasher.finalize();

        let mut key = [0u8; 32];
        key[..16].copy_from_slice(&secret.0[..16]);
        key[16..32].copy_from_slice(&hash[16..32]);
        let mut nonce = [0u8; 16];
        nonce[..4].copy_from_slice(&hash[..4]);
        nonce[4..16].copy_from_slice(&secret.0[20..32]);
        let handshake_aes = AdnlAes::new(key, nonce);
        handshake_aes.apply_keystream(&mut raw_params);

        Self {
            receiver,
            sender,
            hash,
            encrypted_params: raw_params,
        }
    }

    pub fn to_bytes(&self) -> [u8; 256] {
        let mut packet = [0u8; 256];
        packet[..32].copy_from_slice(&self.receiver.0);
        packet[32..64].copy_from_slice(&self.sender.0);
        packet[64..96].copy_from_slice(&self.hash);
        packet[96..256].copy_from_slice(&self.encrypted_params);
        packet
    }
}

pub struct AdnlPacket<'a> {
    length: [u8; 4],
    nonce: &'a mut [u8; 32],
    buffer: &'a mut [u8],
    hash: [u8; 32],
}

impl<'a> AdnlPacket<'a> {
    pub fn send(aes: &mut AdnlAes, nonce: &'a mut [u8; 32], buffer: &'a mut [u8]) -> Self {
        let hasher = Sha256::new();
        hasher.update(nonce);
        hasher.update(buffer);
        // remember not to send more than 4 GiB in a single packet
        let mut length = ((buffer.len() + 64) as u32).to_le_bytes();
        let mut hash = hasher.finalize();
        aes.apply_keystream(&mut length);
        aes.apply_keystream(nonce);
        aes.apply_keystream(buffer);
        aes.apply_keystream(hash);
        Self {
            length,
            nonce,
            buffer,
            hash,
        }
    }
}

pub struct IntegrityError;

pub struct AdnlClientProtocol {
    aes_tx: AdnlAes,
    aes_rx: AdnlAes,
}

impl AdnlClientProtocol {
    pub fn new(aes_params: AdnlAesParams) -> Self {
        Self {
            aes_rx: AdnlAes::new(aes_params.rx_key, aes_params.rx_nonce),
            aes_tx: AdnlAes::new(aes_params.tx_key, aes_params.tx_nonce),
        }
    }

    pub fn send_packet<'a>(&mut self, nonce: &'a mut [u8; 32], buffer: &'a mut [u8]) -> AdnlPacket<'a> {
        AdnlPacket::send(&mut self.aes_tx, nonce, buffer)
    }

    pub fn receive_packet_length(&mut self, buffer: [u8; 4]) -> u32 {
        let mut buffer = buffer;
        self.aes_rx.apply_keystream(&mut buffer);
        u32::from_le_bytes(buffer)
    }

    pub fn receive_packet<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], IntegrityError> {
        if buffer.len() < 32 {
            return Err(IntegrityError);
        }
        self.aes_rx.apply_keystream(buffer);
        let (buffer, given_hash) = buffer.split_at_mut(buffer.len() - 32);
        let computed_hash = Sha256::digest(buffer).as_slice();
        if computed_hash != given_hash {
            return Err(IntegrityError);
        }
        let (_, buffer) = buffer.split_at_mut(32);
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_protocol() {
        let nonce: [u8; 160] = rand::thread_rng().gen();
        let aes_params = AdnlAesParams::from(nonce);

    }
}