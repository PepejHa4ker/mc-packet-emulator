use rand::rngs::OsRng;
use rand::RngCore;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use std::io;
use tokio::net::TcpStream;

pub fn generate_random_bytes(n: usize) -> Vec<u8> {
    let mut buf = vec![0u8; n];
    let mut rng = OsRng;
    rng.fill_bytes(&mut buf);
    buf
}

pub fn encrypt_with_public_key(data: &[u8], public_key_bytes: &[u8]) -> io::Result<Vec<u8>> {
    let public_key = RsaPublicKey::from_pkcs1_der(public_key_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let mut rng = OsRng;
    public_key
        .encrypt(&mut rng, padding, data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub mod encrypted_stream;
pub use encrypted_stream::EncryptedStream;

pub fn enable_encryption<T>(stream: &mut T, shared_secret: Vec<u8>) -> io::Result<EncryptedStream>
where
    T: AsMut<TcpStream>,
{
    EncryptedStream::new(stream.as_mut(), &shared_secret)
}
