use rand::rngs::OsRng;
use rand::RngCore;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use std::io;
use rsa::pkcs8::DecodePublicKey;

pub (crate) fn encrypt_with_server_pubkey(data: &[u8], server_pub_key_der: &[u8]) -> io::Result<Vec<u8>> {
    let public_key = RsaPublicKey::from_public_key_der(server_pub_key_der)
        .map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let mut rng = OsRng;
    public_key.encrypt(&mut rng, padding, data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub (crate) fn generate_shared_secret() -> [u8; 16] {
    let mut secret = [0u8; 16];
    let mut rng = OsRng;
    rng.fill_bytes(&mut secret);
    secret
}

pub mod encrypted_stream;
pub use encrypted_stream::EncryptedStream;
