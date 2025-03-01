use crate::connection::connection_state::ConnectionState;
use crate::connection::conn_reader::ConnReader;
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
            // Неизвестный пакет
        }
    }};
}

pub struct Connection {
    pub state: ConnectionState,
    pub entity_id: Option<i32>,

    /// Может быть `Some(ConnReader::Plain(...))` или `Some(ConnReader::Encrypted(...))`.
    /// Если `None`, значит мы «вынули» поток или соединение разорвано.
    reader: Option<ConnReader>,
}

impl Connection {
    /// Подключается к указанному адресу, оборачивает в BufReader (Plain)
    pub async fn connect(addr: &str) -> io::Result<Self> {
        let tcp = TcpStream::connect(addr).await?;
        let reader = ConnReader::Plain(BufReader::new(tcp));
        Ok(Self {
            state: ConnectionState::Handshaking,
            entity_id: None,
            reader: Some(reader),
        })
    }

    /// Основной цикл чтения входящих пакетов
    pub async fn run(&mut self) -> io::Result<()> {
        loop {
            // Достаём &mut ConnReader из self.reader
            let r = match self.reader.as_mut() {
                Some(r) => r,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "No connection reader available",
                    ));
                }
            };

            // Передаём "r" в макрос process_packet! (нужен &mut R: AsyncRead + ...)
            process_packet!(&mut *r, self.state, self, ServerPacket);
        }
    }

    /// Отправляем пакет на сервер
    pub async fn send_packet<P>(&mut self, packet: &P)
    where
        P: AsyncPacket,
    {
        let r = match self.reader.as_mut() {
            Some(r) => r,
            None => {
                eprintln!("No connection reader available, can't send packet");
                return;
            }
        };

        if let Err(e) = packet.write_to_boxed(&mut *r).await {
            eprintln!("Failed to send packet: {:?}", e);
        }
    }

    /// Пример функции, которая двигает игрока по кругу (необязательно)
    pub async fn start_moving_in_circle(
        conn: Arc<Mutex<Connection>>,
        start_x: f64,
        start_z: f64,
    ) {
        let radius: f32 = 5.0;
        let mut angle: f32 = 0.0;
        let interval = Duration::from_millis(100);

        let mut interval_timer = tokio::time::interval(interval);

        loop {
            interval_timer.tick().await;
            let mut conn_lock = conn.lock().await;

            angle += 0.1;
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

    /// Включение шифрования:
    /// - Берём старый Plain-стрим (ConnReader::Plain)
    /// - Создаём зашифрованный EncryptedStream
    /// - Кладём обратно как ConnReader::Encrypted(...)
    pub fn enable_encryption(&mut self, key: &[u8]) -> io::Result<()> {
        let old = self.reader.take().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "No connection reader to upgrade")
        })?;

        match old {
            ConnReader::Plain(plain_buf) => {
                let tcp = plain_buf.into_inner();
                let encrypted = crate::protocol::crypto::EncryptedStream::new(tcp, key)?;
                let buf_enc = BufReader::new(encrypted);
                self.reader = Some(ConnReader::Encrypted(buf_enc));
            }
            ConnReader::Encrypted(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Already encrypted",
                ));
            }
        }
        Ok(())
    }
}
