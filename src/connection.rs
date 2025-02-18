use crate::protocol::packets::decoder::read_server_packet_by_state;
use crate::protocol::packets::AsyncPacketExt;
use crate::protocol::packets::{CChatMessage, LoginSuccess};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::TcpStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Handshaking,
    Login,
    Status,
    Play,
}


#[macro_export]
macro_rules! process_packet {
    ($packet:expr, $packet_type:ty, $conn:expr, $handler:expr) => {
        if let Some(inner) = (&*$packet).as_packet::<$packet_type>() {
            $handler(inner, $conn).await;
        }
    };
}

pub struct Connection {
    pub stream: TcpStream,
    pub state: ConnectionState,
    }

impl Connection {
    pub async fn connect(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self {
            stream,
            state: ConnectionState::Handshaking,
                    })
    }


    async fn handle_login_success(&mut self) -> io::Result<()> {
        self.state = ConnectionState::Play;
        println!("Переход в состояние Play");
        Ok(())
    }


    pub async fn run(&mut self) -> io::Result<()> {
        loop {
            let packet = read_server_packet_by_state(&mut self.stream, self.state).await?;
            if self.state == ConnectionState::Login {
                if packet.as_packet::<LoginSuccess>().is_some() {
                    self.handle_login_success().await?;
                }
            }

        }
    }


    pub async fn send_packet<P>(&mut self, packet: &P) -> io::Result<()>
    where
        P: crate::protocol::packets::AsyncPacket,
    {
        packet.write_to_boxed(&mut self.stream).await
    }
}

impl AsMut<TcpStream> for Connection {
    fn as_mut(&mut self) -> &mut TcpStream {
        &mut self.stream
    }
}

impl AsyncRead for Connection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for Connection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.stream).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}
