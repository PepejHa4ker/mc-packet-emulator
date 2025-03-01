use aes::Aes128;
use cfb8::cipher::{AsyncStreamCipher, NewCipher};
use cfb8::Cfb8;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::io;

type AesCfb8 = Cfb8<Aes128>;

pub struct EncryptedStream {
    stream: TcpStream,
    encryptor: AesCfb8,
    decryptor: AesCfb8,
}

impl EncryptedStream {
    pub fn new(stream: TcpStream, key: &[u8]) -> io::Result<Self> {
        if key.len() != 16 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Shared secret must be 16 bytes",
            ));
        }
        let encryptor = AesCfb8::new_from_slices(key, key)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let decryptor = AesCfb8::new_from_slices(key, key)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(Self {
            stream,
            encryptor,
            decryptor,
        })
    }
}

impl AsyncRead for EncryptedStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // Сохраняем, сколько уже "набито" в буфере
        let before = buf.filled().len();

        let poll = Pin::new(&mut self.stream).poll_read(cx, buf);

        if let Poll::Ready(Ok(())) = &poll {
            let filled = buf.filled_mut();
            let newly_received = &mut filled[before..];
            self.decryptor.decrypt(newly_received);
        }
        poll
    }
}

impl AsyncWrite for EncryptedStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        // При записи мы сначала шифруем данные в локальный буфер,
        // а потом отправляем их в self.stream
        let mut encrypted = buf.to_vec();
        self.encryptor.encrypt(&mut encrypted);

        Pin::new(&mut self.stream).poll_write(cx, &encrypted)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}
