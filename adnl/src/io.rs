extern crate std;

use std::io::{Read, Write};
use crate::{AdnlAddress, AdnlAesParams, AdnlHandshake, AdnlPublicKey, AdnlReceiver, AdnlSecret, AdnlSender};

pub struct AdnlClient<T: Read + Write> {
    sender: AdnlSender,
    receiver: AdnlReceiver,
    transport: T,
}

impl<T: Read + Write> AdnlClient<T> {
    pub fn connect(mut transport: T, receiver: AdnlAddress, sender: AdnlPublicKey, secret: &AdnlSecret) -> Result<Self, std::io::Error> {
        let aes_params = AdnlAesParams::generate();
        let handshake = AdnlHandshake::new(receiver, sender, secret, &aes_params);
        transport.write_all(&handshake.to_bytes())?;
        Ok(Self {
            sender: AdnlSender::new(&aes_params),
            receiver: AdnlReceiver::new(&aes_params),
            transport,
        })
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut nonce = rand::random::<[u8; 32]>();
        let mut data = data.to_vec();
        let packet = self.sender.send_packet(&mut nonce, &mut data);
        let mut packet_raw = Vec::<u8>::with_capacity(data.len() + 68);
        packet_raw.extend(packet.length());
        packet_raw.extend(packet.nonce());
        packet_raw.extend(packet.buffer());
        packet_raw.extend(packet.hash());
        self.transport.write_all(&packet_raw)?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut len = [0u8; 4];
        self.transport.read_exact(&mut len)?;
        let len = self.receiver.receive_packet_length(len);
        let mut result = Vec::<u8>::with_capacity(len as usize);
        result.resize(len as usize, 0);
        self.transport.read_exact(&mut result);
        let result = self.receiver.receive_packet(&mut result).map_err(|_| "Integrity error".into())?;
        Ok(result.to_vec())
    }
}