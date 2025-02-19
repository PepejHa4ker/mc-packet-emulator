use crate::protocol::fields::{Boolean, Byte, Double, Float, Int, VarInt, VarString};
use crate::protocol::packets::decoder::read_server_packet_by_state;
use crate::protocol::packets::LoginSuccess;
use crate::protocol::packets::{AsyncPacket, AsyncPacketExt, CKeepAlive, CPlayerPosLook, ClientSettings, ClientStatus, EntityLookAndMovement, EntityLookMove, EntityRelMove, EntityTeleport, JoinGame, Respawn, SChatMessage, SKeepAlive, UpdateHealth};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite, BufReader, ReadBuf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

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

    async fn handle_login_success(mut self) -> io::Result<()> {
        self.state = ConnectionState::Play;
        println!("Переход в состояние Play");
        Ok(())
    }

    async fn handle_join_game(&self, join_game: &JoinGame) -> io::Result<()>{
        println!("Joined game: {:?}", join_game);
        Ok(())
    }

    pub async fn run(&mut self) -> io::Result<()> {
        let mut state = self.state;

        loop {
            let packet = read_server_packet_by_state(&mut self.reader, state).await?;

            if state == ConnectionState::Login {
                if packet.as_packet::<LoginSuccess>().is_some() {
                    println!("Play!");









                    state = ConnectionState::Play;
                }
            }
            if state == ConnectionState::Play {
                if let Some(join_game) = packet.as_packet::<JoinGame>() {
                    let confirm_respawn = ClientStatus { action_id: VarInt(0) };

                    println!("Joined game! {:?}", join_game);
                    self.entity_id = Some(join_game.entity_id.clone());
                    let settings = ClientSettings {
                        locale: VarString("en_US".to_string()),
                        view_distance: Byte(10),
                        chat_flags: Byte(1),
                        chat_colors: Boolean(true),
                        difficulty: Byte(0),
                        show_cape: Boolean(true),
                    };
                    self.send_packet(&settings).await?;
                    self.send_packet(&confirm_respawn).await?;

                }
                if let Some(s_keep_alive) = packet.as_packet::<SKeepAlive>() {
                    println!("Answering KEEP ALIVE!!!! {:?}", s_keep_alive);

                    let c_keep_alive = CKeepAlive {
                        keep_alive_id: s_keep_alive.keep_alive_id.clone()
                    };
                    self.send_packet(&c_keep_alive).await?

                }
                if let Some(chat_packet) = packet.as_packet::<SChatMessage>() {
                    println!("Got chat message! {:?}", chat_packet.json_data.0);

                }

                if let Some(update_health) = packet.as_packet::<UpdateHealth>() {
                    println!("UpdateHealth: {:?}", &update_health.health);
                    if update_health.health.0 <= 0.0 {
                        println!("Бот умер, отправляем запрос на респавн...");
                        let respawn_packet = ClientStatus { action_id: VarInt(0) };
                        self.send_packet(&respawn_packet).await?;
                    }
                }

                if let Some(_) = packet.as_packet::<Respawn>() {
                    println!("Бот респавнится...");
                    let confirm_respawn = ClientStatus { action_id: VarInt(0) };
                    self.send_packet(&confirm_respawn).await?;
                }
                if let Some(look_move) = packet.as_packet::<EntityLookMove>() {
                    if let Some(entity_id) = self.entity_id.clone() {
                            println!("LookMove: {:?}", look_move);

                    }
                }
                if let Some(look_move) = packet.as_packet::<EntityLookAndMovement>() {
                    if let Some(entity_id) = self.entity_id.clone() {
                            println!("LookMovement: {:?}", look_move);

                    }
                    if let Some(look_move) = packet.as_packet::<EntityRelMove>() {
                        if let Some(entity_id) = self.entity_id.clone() {
                                println!("LookMovement: {:?}", look_move);

                        }
                    }
                    if let Some(look_move) = packet.as_packet::<EntityTeleport>() {
                        if let Some(entity_id) = self.entity_id.clone() {
                                println!("EntityTeleport: {:?}", look_move);

                        }
                    }
                }
            }
        }
    }

    pub async fn send_packet<P>(&mut self, packet: &P) -> io::Result<()>
    where
        P: crate::protocol::packets::AsyncPacket,
    {
        packet.write_to_boxed(self.reader.get_mut()).await
    }

    pub async fn start_moving_in_circle(conn: Arc<Mutex<Connection>>, start_x: f64, start_y: f64, start_z: f64) {
        let radius: f32 = 5.0;
        let step_count = 100;

        let mut angle: f32 = 0.0;
        let interval = Duration::from_millis(100);

        let mut interval_timer = tokio::time::interval(interval);

        loop {
            interval_timer.tick().await;
            let mut conn_lock = conn.lock().await;

            let new_x = ((start_x as f32) + radius * (angle).cos()) as f64;
            let new_z = ((start_z as f32) + radius * (angle).sin()) as f64;

            let move_packet = CPlayerPosLook {
                x: Double(new_x),
                y: Double(64.0),
                stance: Double(16.0),
                z: Double(new_z),
                yaw: Float(angle.to_degrees()),
                pitch: Float(0.0),
                on_ground: Boolean(true),
            };

            if let Err(e) = conn_lock.send_packet(&move_packet).await {
                eprintln!("Ошибка при отправке пакета движения: {:?}", e);
            }





        }
    }

}
impl AsyncRead for Connection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

impl AsyncWrite for Connection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.reader).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.reader).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.reader).poll_shutdown(cx)
    }
}
