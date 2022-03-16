use aes::Aes256;
use ctr::cipher::StreamCipher;
use ctr::Ctr128BE;
use sha2::{Sha256, Digest};
use core::convert::TryInto;
use aes::cipher::KeyIvInit;

type AdnlAes = Ctr128BE<Aes256>;

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

pub struct AdnlHandshake {
    receiver: AdnlAddress,
    sender: AdnlPublicKey,
    hash: [u8; 32],
    encrypted_params: [u8; 160],
}

impl AdnlHandshake {
    pub fn new(receiver: AdnlAddress, sender: AdnlPublicKey, secret: &AdnlSecret, aes_params: &AdnlAesParams) -> Self {
        let mut raw_params = aes_params.to_bytes();
        let mut hasher = Sha256::new();
        hasher.update(&raw_params);
        let hash: [u8; 32] = hasher.finalize().try_into().unwrap();

        let mut key = [0u8; 32];
        key[..16].copy_from_slice(&secret.0[..16]);
        key[16..32].copy_from_slice(&hash[16..32]);
        let mut nonce = [0u8; 16];
        nonce[..4].copy_from_slice(&hash[..4]);
        nonce[4..16].copy_from_slice(&secret.0[20..32]);
        let mut handshake_aes = AdnlAes::new(&key.into(), &nonce.into());
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

    pub fn send(aes: &'a mut AdnlAes, nonce: &'a mut [u8; 32], buffer: &'a mut [u8]) -> Self {
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

pub struct IntegrityError;

pub struct AdnlClientProtocol {
    aes_tx: AdnlAes,
    aes_rx: AdnlAes,
}

impl AdnlClientProtocol {
    pub fn new(aes_params: AdnlAesParams) -> Self {
        Self {
            aes_rx: AdnlAes::new(&aes_params.rx_key.into(), &aes_params.rx_nonce.into()),
            aes_tx: AdnlAes::new(&aes_params.tx_key.into(), &aes_params.tx_nonce.into()),
        }
    }

    pub fn send_packet<'a>(&'a mut self, nonce: &'a mut [u8; 32], buffer: &'a mut [u8]) -> AdnlPacket<'a> {
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
        let computed_hash = Sha256::digest(&buffer);
        if computed_hash.as_slice() != given_hash {
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
    use x25519_dalek::{StaticSecret, PublicKey};
    use hex::FromHex;

    #[test]
    fn test_handshake_1() {
        let aes_params = hex::decode("b3d529e34b839a521518447b68343aebaae9314ac95aaacfdb687a2163d1a98638db306b63409ef7bc906b4c9dc115488cf90dfa964f520542c69e1a4a495edf9ae9ee72023203c8b266d552f251e8d724929733428c8e276ab3bd6291367336a6ab8dc3d36243419bd0b742f76691a5dec14edbd50f7c1b58ec961ae45be58cbf6623f3ec9705bd5d227761ec79cee377e2566ff668f863552bddfd6ff3a16b").unwrap();
        let remote_public = hex::decode("2615edec7d5d6538314132321a2615e1ff5550046e0f1165ff59150632d2301f").unwrap();
        let ecdh = hex::decode("1f4d11789a5559b238f7ac8213e112184f16a97593b4a059c878af288a784b79").unwrap();
        let expected_handshake = hex::decode("a3fc70bfeff13b04ed4f2581045ff95a385df762eb82ab9902066061c2e6033e67d45a90e775d8f78d9feb9bdd222446e07c3de4a54e29220d18c18c5b340db36c06a61a8eb209b2b4f9d7359d76e3e0722024579d2b8bc920a6506238d6d88d14a880eb99b4996df8a11bb1a7124e39825848c74fc3d7bfab034e71dbc2e2d1606c14db1b04bb25b544a83b47815e9ec0590a9f4dd011b4bae7b01ddb376570d6641919e63933bf297a073b8febfae0c4dd298215e5db929c6764c43502874b7b5af6380fd52d3fd072b7046d6ccadecc771f54b461b5a157fe3e059df9575dc72dfc89e36b26a7cf9a4e7925c96e88d5342c139154c4a6e4e9d683d9373e3a").unwrap();
        let local_public = hex::decode("67d45a90e775d8f78d9feb9bdd222446e07c3de4a54e29220d18c18c5b340db3").unwrap();
        test_handshake(remote_public, local_public, ecdh, aes_params, expected_handshake);
    }

    #[test]
    fn test_handshake_2() {
        let aes_params = hex::decode("7e3c66de7c64d4bee4368e69560101991db4b084430a336cffe676c9ac0a795d8c98367309422a8e927e62ed657ba3eaeeb6acd3bbe5564057dfd1d60609a25a48963cbb7d14acf4fc83ec59254673bc85be22d04e80e7b83c641d37cae6e1d82a400bf159490bbc0048e69234ad89e999d792eefdaa56734202546d9188706e95e1272267206a8e7ee1f7c077f76bd26e494972e34d72e257bf20364dbf39b0").unwrap();
        let remote_public = hex::decode("2615edec7d5d6538314132321a2615e1ff5550046e0f1165ff59150632d2301f").unwrap();
        let ecdh = hex::decode("10a28a56cce723b2ab75aeba51039f5f3f72bca49f22b7f8039690811bb0606e").unwrap();
        let expected_handshake = hex::decode("a3fc70bfeff13b04ed4f2581045ff95a385df762eb82ab9902066061c2e6033ed86dac237d94b1b611dcac497f952edb63756910dbf625f5c5806e159d1047270f372a88fd1f76b0a574620cf47202369359bdeff8e709d6c0578cf08d2499cb949ecaaf892f11fc772932182269f9e5f2f44150066ae65fbb5fc9f51dab26825bd6fd4d72de9ccc80bbddcb9d47f9c3cfd00b80a5d9faf15007abb480f9fd85e2f671484e82f3b67f58197c5438dab575062faa9acd821ca6a10e7061c40535112650f1730d03484de0d01aa7912ed64655e672bd077c1f1e50b231556ecfd5e5009f47804c317abec6310165a6618125a2204b0370d40e672e1a640817b894").unwrap();
        let local_public = hex::decode("d86dac237d94b1b611dcac497f952edb63756910dbf625f5c5806e159d104727").unwrap();
        test_handshake(remote_public, local_public, ecdh, aes_params, expected_handshake);
    }

    fn test_handshake(remote_public: Vec<u8>, local_public: Vec<u8>, ecdh: Vec<u8>, aes_params: Vec<u8>, expected_handshake: Vec<u8>) {
        let aes_params: [u8; 160] = aes_params.try_into().unwrap();
        let aes_params = AdnlAesParams::from(aes_params);
        let remote_public: [u8; 32] = remote_public.try_into().unwrap();
        let remote_public = AdnlPublicKey::from(remote_public);
        let local_public: [u8; 32] = local_public.try_into().unwrap();
        let local_public = AdnlPublicKey::from(local_public);
        let ecdh: [u8; 32] = ecdh.try_into().unwrap();
        let ecdh = AdnlSecret::from(ecdh);
        let handshake = AdnlHandshake::new(AdnlAddress::from(remote_public), local_public, &ecdh, &aes_params);
        assert_eq!(handshake.to_bytes(), expected_handshake.as_slice(), "handshake is not the same!");
    }

    #[test]
    fn test_send_1() {
        let aes_params = hex::decode("b3d529e34b839a521518447b68343aebaae9314ac95aaacfdb687a2163d1a98638db306b63409ef7bc906b4c9dc115488cf90dfa964f520542c69e1a4a495edf9ae9ee72023203c8b266d552f251e8d724929733428c8e276ab3bd6291367336a6ab8dc3d36243419bd0b742f76691a5dec14edbd50f7c1b58ec961ae45be58cbf6623f3ec9705bd5d227761ec79cee377e2566ff668f863552bddfd6ff3a16b").unwrap();
        let mut nonce = hex::decode("9a5ecd5d9afdfff2823e7520fa1c338f2baf1a21f51e6fdab0491d45a50066f7").unwrap();
        let mut buffer = hex::decode("7af98bb471ff48e9b263959b17a04faae4a23501380d2aa932b09eac6f9846fcbae9bbcb0cdf068c7904345aad16000000000000").unwrap();
        let mut expected_packet = hex::decode("250d70d08526791bc2b6278ded7bf2b051afb441b309dda06f76e4419d7c31d4d5baafc4ff71e0ebabe246d4ea19e3e579bd15739c8fc916feaf46ea7a6bc562ed1cf87c9bf4220eb037b9a0b58f663f0474b8a8b18fa24db515e41e4b02e509d8ef261a27ba894cbbecc92e59fc44bf5ff7c8281cb5e900").unwrap();
        test_send(aes_params, nonce, buffer, expected_packet);
    }

    #[test]
    fn test_send_2() {
        let aes_params = hex::decode("7e3c66de7c64d4bee4368e69560101991db4b084430a336cffe676c9ac0a795d8c98367309422a8e927e62ed657ba3eaeeb6acd3bbe5564057dfd1d60609a25a48963cbb7d14acf4fc83ec59254673bc85be22d04e80e7b83c641d37cae6e1d82a400bf159490bbc0048e69234ad89e999d792eefdaa56734202546d9188706e95e1272267206a8e7ee1f7c077f76bd26e494972e34d72e257bf20364dbf39b0").unwrap();
        let mut nonce = hex::decode("d36d0683da23e62910fa0e8a9331dfc257db4cde0ba8d63893e88ac4de7d8d6c").unwrap();
        let mut buffer = hex::decode("7af98bb47bcae111ea0e56457826b1aec7f0f59b9b6579678b3db3839d17b63eb60174f20cdf068c7904345aad16000000000000").unwrap();
        let mut expected_packet = hex::decode("24c709a0f676750ddaeafc8564d84546bfc831af27fb66716de382a347a1c32adef1a27e597c8a07605a09087fff32511d314970cad3983baefff01e7ee51bb672b17f7914a6d3f229a13acb14cdc14d98beae8a1e96510756726913541f558c2ffac63ed6cb076d0e888c3c0bb014d9f229c2a3f62e0847").unwrap();
        test_send(aes_params, nonce, buffer, expected_packet);
    }

    fn test_send(aes_params: Vec<u8>, nonce: Vec<u8>, buffer: Vec<u8>, expected_packet: Vec<u8>) {
        let mut nonce = nonce.try_into().unwrap();
        let mut buffer = buffer;
        let aes_params: [u8; 160] = aes_params.try_into().unwrap();
        let aes_params = AdnlAesParams::from(aes_params);
        let mut protocol_client = AdnlClientProtocol::new(aes_params);
        let packet = protocol_client.send_packet(&mut nonce, &mut buffer);
        let mut v = Vec::from(*packet.length());
        v.extend_from_slice(packet.nonce());
        v.extend_from_slice(packet.buffer());
        v.extend_from_slice(packet.hash());
        assert_eq!(v.as_slice(), &expected_packet, "outcoming packet is wrong");
    }

    #[test]
    fn test_recv_1() {
        let encrypted_len = hex::decode("81e95e43").unwrap();
        let encrypted_data = hex::decode("3c87c9ad2a716637b3a12644fbfb12dbd02996abc40ed2beb352483d6ecf9e2ad181a5abde4d4146ca3a8524739d3acebb2d7599cc6b81967692a62118997e16").unwrap();
        let expected_data = Vec::new();
        let aes_params = hex::decode("b3d529e34b839a521518447b68343aebaae9314ac95aaacfdb687a2163d1a98638db306b63409ef7bc906b4c9dc115488cf90dfa964f520542c69e1a4a495edf9ae9ee72023203c8b266d552f251e8d724929733428c8e276ab3bd6291367336a6ab8dc3d36243419bd0b742f76691a5dec14edbd50f7c1b58ec961ae45be58cbf6623f3ec9705bd5d227761ec79cee377e2566ff668f863552bddfd6ff3a16b").unwrap();
        let aes_params: [u8; 160] = aes_params.try_into().unwrap();
        let aes_params = AdnlAesParams::from(aes_params);
        let mut protocol_client = AdnlClientProtocol::new(aes_params);
        test_recv(&mut protocol_client, encrypted_len, encrypted_data, expected_data);
        let encrypted_len = hex::decode("4b72a32b").unwrap();
        let encrypted_data = hex::decode("f31894cce9ceffd2dd97176e502946524e45e62689bd8c5d31ad53603c5fd3b402771f707cd2747747fad9df52e6c23ceec9fa2ee5b0f68b61c33c7790db03d1c593798a29d716505cea75acdf0e031c25447c55c4d29d32caab29bd5a0787644843bafc04160c92140aab0ecc990927").unwrap();
        let expected_data = hex::decode("1684ac0f71ff48e9b263959b17a04faae4a23501380d2aa932b09eac6f9846fcbae9bbcb080d0053e9a3ac3062000000").unwrap();
        test_recv(&mut protocol_client, encrypted_len, encrypted_data, expected_data);
    }

    #[test]
    fn test_recv_2() {
        let encrypted_len = hex::decode("b75dcf27").unwrap();
        let encrypted_data = hex::decode("582beb4031d6d3700c9b7925bf84a78f2bd16b186484d36427a8824ac86e27cea81eb5bcbac447a37269845c65be51babd11c80627f81b4247f84df16d05c4f1").unwrap();
        let expected_data = Vec::new();
        let aes_params = hex::decode("7e3c66de7c64d4bee4368e69560101991db4b084430a336cffe676c9ac0a795d8c98367309422a8e927e62ed657ba3eaeeb6acd3bbe5564057dfd1d60609a25a48963cbb7d14acf4fc83ec59254673bc85be22d04e80e7b83c641d37cae6e1d82a400bf159490bbc0048e69234ad89e999d792eefdaa56734202546d9188706e95e1272267206a8e7ee1f7c077f76bd26e494972e34d72e257bf20364dbf39b0").unwrap();
        let aes_params: [u8; 160] = aes_params.try_into().unwrap();
        let aes_params = AdnlAesParams::from(aes_params);
        let mut protocol_client = AdnlClientProtocol::new(aes_params);
        test_recv(&mut protocol_client, encrypted_len, encrypted_data, expected_data);
        let encrypted_len = hex::decode("77ebea5a").unwrap();
        let encrypted_data = hex::decode("6e6c8758e7703d889abad16e7e3c4e0c10c4e81ca10d0d9abddabb6f008905133a070ff825ad3f4b0ae969e04dbd8b280864d3d2175f3bc7cf3deb31de5497fa43997d8e2acafb9a31de2a22ecb279b5854c00791216e39c2e65863539d82716fc020e9647b2dd99d0f14e4f553b645f").unwrap();
        let expected_data = hex::decode("1684ac0f7bcae111ea0e56457826b1aec7f0f59b9b6579678b3db3839d17b63eb60174f2080d0053e90bb03062000000").unwrap();
        test_recv(&mut protocol_client, encrypted_len, encrypted_data, expected_data);
    }

    fn test_recv(client: &mut AdnlClientProtocol, encrypted_length: Vec<u8>, encrypted_packet: Vec<u8>, expected_data: Vec<u8>) {

        let encrypted_len: [u8; 4] = encrypted_length.try_into().unwrap();
        let mut encrypted_packet = encrypted_packet;

        let expected_len = expected_data.len() + 64;

        let actual_len = client.receive_packet_length(encrypted_len);
        assert_eq!(actual_len as usize, expected_len, "len is wrong");
        let result = client.receive_packet(&mut encrypted_packet);
        if let Ok(decrypted_data) = result {
            assert_eq!(decrypted_data, expected_data.as_slice(), "incoming packet is wrong");
        } else {
            panic!("integrity error");
        }
    }
}