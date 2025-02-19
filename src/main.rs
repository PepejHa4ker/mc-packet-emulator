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
    let mut handles = Vec::new();

    for i in 0..1 {
        let addr = addr.to_string();
        let handle = tokio::spawn(async move {
            if let Ok(conn) = Connection::connect(&addr).await {
                let conn = Arc::new(Mutex::new(conn));
                let conn_listener = Arc::clone(&conn);

                let run_handle = tokio::spawn(async move {
                    if let Err(e) = conn_listener.lock().await.run().await {
                        eprintln!("Ошибка в run() для клиента {}: {:?}", i, e);
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
                    conn_lock.send_packet(&handshake).await.expect("Ошибка отправки Handshake");
                    println!("Handshake отправлен от клиента Truncator{}", i);
                    conn_lock.state = ConnectionState::Login;

                    let login_start = LoginStart {
                        name: VarString(String::from_utf8_lossy(format!("Truncator{}", i).as_bytes()).to_string()),
                    };

                    conn_lock.send_packet(&login_start).await.expect("Ошибка отправки LoginStart");
                    println!("LoginStart отправлен от клиента Truncator{}", i);
                }


                run_handle.await.expect("Ошибка в run()");
            } else {
                eprintln!("Не удалось подключиться клиенту Truncator{}", i);
            }
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.await.expect("Ошибка в одном из потоков");
    }

    Ok(())
}
