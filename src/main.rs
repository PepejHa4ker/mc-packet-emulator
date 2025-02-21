use crate::connection::connection::Connection;
use crate::connection::connection_state::ConnectionState;
use crate::protocol::fields::ByteArrayShort;
use crate::protocol::packets::{client, Handshake, LoginStart};
use protocol::fields::{UShort, VarInt, VarString};
use protocol::packets::AsyncPacket;
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod connection;
pub mod protocol;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "s36.mcskill.net:25565";
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
                        server_address: VarString("s36.mcskill.net".to_string()),
                        server_port: UShort(25565),
                        next_state: VarInt(2),
                    };
                    conn_lock.send_packet(&handshake).await;
                    println!("Handshake отправлен от клиента flowler");
                    conn_lock.state = ConnectionState::Login;
                    let channels = VarString("REGISTER".to_string());



                    let custom_paylaod = client::CustomPayload {
                        channel: channels,
                        data: ByteArrayShort("FML|HS".to_string().into_bytes())
                    };
                    conn_lock.send_packet(&custom_paylaod).await;
                    println!("Отправлена регистрация на канал FML|HS");
                    let login_start = LoginStart {
                        name: VarString("flowler".to_string())
                    };

                    conn_lock.send_packet(&login_start).await;
                    println!("LoginStart отправлен от клиента flowler{}", i);

                    let channels = VarString("FML|HS".to_string());
                    // Если требуется регистрация нескольких каналов, их можно объединить с разделителем "\0", например:

                }

                run_handle.await.expect("Ошибка в run()");
            } else {
                eprintln!("Не удалось подключиться клиенту flowler{}", i);
            }
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.await.expect("Ошибка в одном из потоков");
    }

    Ok(())
}
