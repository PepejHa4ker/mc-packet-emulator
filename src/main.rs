use crate::protocol::packets::LoginStart;
use connection::{Connection, ConnectionState};
use protocol::fields::{UShort, VarInt, VarString};
use protocol::packets::handshake::Handshake;
use protocol::packets::AsyncPacket;
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod connection;
pub mod protocol;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:25565";

    let conn = Connection::connect(addr).await?;
    let conn = Arc::new(Mutex::new(conn));
    let conn_listener = Arc::clone(&conn);
    tokio::spawn(async move {
        if let Err(e) = conn_listener.lock().await.run().await {
            eprintln!("Ошибка в run(): {:?}", e);
        }
    });

    {
        let mut conn_lock = conn.lock().await;
        let handshake = Handshake {
            protocol_version: VarInt(5),
            server_address: VarString("localhost".to_string()),
            server_port: UShort(25565),
            next_state: VarInt(2),
        };
        conn_lock.send_packet(&handshake).await?;
        println!("Handshake отправлен.");
        conn_lock.state = ConnectionState::Login;

        let login_start = LoginStart {
            name: VarString("Truncator".to_string()),
        };
        conn_lock.send_packet(&login_start).await?;
        println!("LoginStart отправлен.");
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
