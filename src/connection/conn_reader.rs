use tokio::io::{self, AsyncRead, AsyncWrite, BufReader, ReadBuf};
use tokio::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::protocol::crypto::EncryptedStream; 

pub enum ConnReader {
    Plain(BufReader<TcpStream>),
    Encrypted(BufReader<EncryptedStream>),
}

impl AsyncRead for ConnReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match &mut *self {
            ConnReader::Plain(r) => Pin::new(r).poll_read(cx, buf),
            ConnReader::Encrypted(r) => Pin::new(r).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for ConnReader {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        data: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            ConnReader::Plain(r) => Pin::new(r.get_mut()).poll_write(cx, data),
            ConnReader::Encrypted(r) => Pin::new(r.get_mut()).poll_write(cx, data),
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        match &mut *self {
            ConnReader::Plain(r) => Pin::new(r.get_mut()).poll_flush(cx),
            ConnReader::Encrypted(r) => Pin::new(r.get_mut()).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        match &mut *self {
            ConnReader::Plain(r) => Pin::new(r.get_mut()).poll_shutdown(cx),
            ConnReader::Encrypted(r) => Pin::new(r.get_mut()).poll_shutdown(cx),
        }
    }
}
