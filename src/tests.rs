use x25519_dalek::{PublicKey, StaticSecret};
use std::convert::TryInto;

#[test]
fn test_ecdh() {
    let their_public: [u8; 32] = hex::decode("2615edec7d5d6538314132321a2615e1ff5550046e0f1165ff59150632d2301f").unwrap().try_into().unwrap();
    let their_public = PublicKey::from(their_public);
    let our_private: [u8; 32] = hex::decode("786437859a11dbc755afa4517e1b004743a5bda08c54d95d14384ca99cbe2151").unwrap().try_into().unwrap();
    let our_private = StaticSecret::from(our_private);
    let expected_ecdh: [u8; 32] = hex::decode("dbddc0d957850329766fd8e9a370a115d229625013ea5849900c063f9dc6ec6b").unwrap().try_into().unwrap();
    let expected_public: [u8; 32] = hex::decode("50347216d43ab2e87f4146abaabd2e01e59aa6e49ba5174304a95df7239f01b1").unwrap().try_into().unwrap();
    let ecdh = our_private.diffie_hellman(&their_public).to_bytes();
    let our_public = PublicKey::from(&our_private);
    assert_eq!(our_public.to_bytes(), expected_public);
    assert_eq!(x25519_dalek::x25519(our_private.to_bytes(), our_public.to_bytes()), expected_ecdh);
    //assert_eq!(ecdh, expected_ecdh);
}