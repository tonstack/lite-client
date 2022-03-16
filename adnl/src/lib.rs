#![cfg_attr(not(feature = "std"), no_std)]

mod helper_types;
mod handshake;
mod send;
mod receive;

#[cfg(feature = "std")]
mod io;

#[cfg(test)]
mod tests;

pub use helper_types::{AdnlAesParams, IntegrityError, AdnlSecret, AdnlAddress, AdnlPublicKey};
pub use handshake::AdnlHandshake;
pub use send::{AdnlSender, AdnlPacketBuilder};
pub use receive::AdnlReceiver;

#[cfg(feature = "std")]
pub use io::AdnlClient;

