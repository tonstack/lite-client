#![cfg_attr(not(feature = "std"), no_std)]

pub use helper_types::{AdnlAddress, AdnlAesParams, AdnlError, AdnlPublicKey, AdnlSecret, Empty};
pub use primitives::handshake::AdnlHandshake;
pub use primitives::receive::AdnlReceiver;
pub use primitives::send::AdnlSender;
pub use wrappers::client::AdnlClient;
pub use wrappers::builder::AdnlBuilder;
pub use integrations::dalek;

mod helper_types;

#[cfg(test)]
mod tests;
mod primitives;
mod integrations;
mod wrappers;

