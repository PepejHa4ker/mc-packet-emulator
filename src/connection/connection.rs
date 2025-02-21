use crate::connection::connection_state::ConnectionState;
use crate::protocol::fields::{Boolean, Double, Float, Int};
use crate::protocol::packets::decoder::read_server_packet_by_state;
use crate::protocol::packets::server::*;
use crate::protocol::packets::{AsyncPacket, PlayerPosLook};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite, BufReader, ReadBuf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;


#[macro_export]
macro_rules! process_packet {
    ($reader:expr, $state:expr, $handler:expr, $packet_enum:ident) => {{
        let boxed_packet = read_server_packet_by_state($reader, $state).await?;
        if let Some(packet) = $packet_enum::try_from(boxed_packet) {
            packet.handle_by($handler).await;
        } else {
        }
    }};
}

pub struct Connection {
    reader: BufReader<TcpStream>,
    pub state: ConnectionState,
    pub entity_id: Option<Int>,
}




impl Connection {
    pub async fn connect(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self {
            reader: BufReader::new(stream),
            state: ConnectionState::Handshaking,
            entity_id: None,
        })
    }


    pub async fn run(&mut self) -> io::Result<()> {
        loop {
            process_packet!(&mut self.reader, self.state, self, ServerPacket);
        }
    }

    pub async fn send_packet<P>(&mut self, packet: &P)
    where
        P: AsyncPacket,
    {
        packet.write_to_boxed(self.reader.get_mut()).await.expect("Failed to send packet");
    }

    pub async fn start_moving_in_circle(
        conn: Arc<Mutex<Connection>>,
        start_x: f64,
        start_z: f64,
    ) {
        let radius: f32 = 5.0;

        let angle: f32 = 0.0;
        let interval = Duration::from_millis(100);

        let mut interval_timer = tokio::time::interval(interval);

        loop {
            interval_timer.tick().await;
            let mut conn_lock = conn.lock().await;

            let new_x = ((start_x as f32) + radius * angle.cos()) as f64;
            let new_z = ((start_z as f32) + radius * angle.sin()) as f64;

            let move_packet = PlayerPosLook {
                x: Double(new_x),
                y: Double(64.0),
                stance: Double(16.0),
                z: Double(new_z),
                yaw: Float(angle.to_degrees()),
                pitch: Float(0.0),
                on_ground: Boolean(true),
            };

            conn_lock.send_packet(&move_packet).await;
        }
    }
}
impl AsyncRead for Connection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.reader.get_mut()).poll_read(cx, buf)
    }
}

impl AsyncWrite for Connection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.reader.get_mut()).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.reader.get_mut()).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.reader.get_mut()).poll_shutdown(cx)
    }
}
