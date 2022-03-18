use ciborium_io::{Read, Write};
use crate::{AdnlAddress, AdnlAesParams, AdnlHandshake, AdnlPublicKey, AdnlReceiver, AdnlSecret, AdnlSender, AdnlError, Empty};

pub struct AdnlClient<T: Read + Write> {
    sender: AdnlSender,
    receiver: AdnlReceiver,
    transport: T,
}

impl<T: Read + Write> AdnlClient<T> {
    pub fn handshake(mut transport: T, receiver: AdnlAddress, sender: AdnlPublicKey, secret: &AdnlSecret) -> Result<Self, AdnlError<T, T, Empty>> {
        let aes_params = AdnlAesParams::generate();
        let handshake = AdnlHandshake::new(receiver, sender, secret, &aes_params);

        // send handshake
        transport.write_all(&handshake.to_bytes()).map_err(|e| AdnlError::WriteError(e))?;

        // receive empty message to ensure that server knows our AES keys
        let mut client = Self {
            sender: AdnlSender::new(&aes_params),
            receiver: AdnlReceiver::new(&aes_params),
            transport,
        };
        client.receive(&mut Empty).map_err(|e| match e {
            AdnlError::ReadError(err) => AdnlError::ReadError(err),
            AdnlError::WriteError(_) => unreachable!(),
            AdnlError::ConsumeError(err) => AdnlError::ConsumeError(err),
            AdnlError::IntegrityError => AdnlError::IntegrityError,
            AdnlError::TooShortPacket => AdnlError::TooShortPacket
        })?;
        Ok(client)
    }

    pub fn send(&mut self, data: &mut [u8]) -> Result<(), AdnlError<Empty, T, Empty>> {
        let mut nonce = rand::random::<[u8; 32]>();
        self.sender.send(&mut self.transport, &mut nonce, data)
    }

    pub fn receive<C: Write>(&mut self, consumer: &mut C) -> Result<(), AdnlError<T, Empty, C>> {
        self.receiver.receive::<_, _, 8192>(&mut self.transport, consumer)
    }
}