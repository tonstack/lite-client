use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::{Deref, DerefMut};
use adnl::AdnlPublicKey;
use serde::{Deserialize, Serialize};
use x25519_dalek::PublicKey;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type")]
pub enum ConfigPublicKey {
    #[serde(rename = "pub.ed25519")]
    Ed25519 {
        #[serde_as(as = "serde_with::base64::Base64")]
        key: [u8; 32]
    },
}

#[derive(Debug, Clone)]
pub struct LiteServerAddress(Ipv4Addr);

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigLiteServer {
    #[serde_as(as = "serde_with::FromInto<i32>")]
    pub ip: LiteServerAddress,
    pub port: u16,
    pub id: ConfigPublicKey
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigGlobal {
    pub liteservers: Vec<ConfigLiteServer>
}

impl AdnlPublicKey for ConfigPublicKey {
    fn to_bytes(&self) -> [u8; 32] {
        match self {
            ConfigPublicKey::Ed25519 { key } => *key
        }
    }
}

impl From<ConfigPublicKey> for PublicKey {
    fn from(public: ConfigPublicKey) -> Self {
        Self::from(public.to_bytes())
    }
}

impl Deref for LiteServerAddress {
    type Target = Ipv4Addr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LiteServerAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i32> for LiteServerAddress {
    fn from(v: i32) -> Self {
        Self(Ipv4Addr::from(v as u32))
    }
}

impl From<LiteServerAddress> for i32 {
    fn from(v: LiteServerAddress) -> Self {
        u32::from(v.0) as i32
    }
}

impl ConfigLiteServer {
    pub fn socket_addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(*self.ip, self.port)
    }
}