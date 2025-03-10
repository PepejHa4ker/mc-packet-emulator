use crate::connection::connection::Connection;
use crate::connection::connection_state::ConnectionState;
use crate::protocol::fields::ByteArrayShort;
use crate::protocol::packets::{Handshake, LoginStart};
use protocol::fields::{UShort, VarInt, VarString};
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::protocol::{ping, query};

pub mod connection;
pub mod protocol;
mod auth;

const HWID_BYTES: &[u8] = &[0, 44, 104, 52, 86, 103, 86, 70, 89, 85, 110, 49, 116, 71, 97, 111, 121, 50, 55, 47, 97, 83, 108, 115, 65, 81, 71, 90, 101, 97, 106, 99, 88, 74, 68, 109, 114, 66, 75, 115, 118, 71, 90, 65, 77, 61, 0, 32, 83, 118, 114, 70, 122, 78, 52, 105, 104, 68, 119, 55, 97, 113, 118, 76, 105, 77, 85, 72, 90, 43, 101, 67, 69, 90, 77, 122, 57, 80, 109, 114, 0, 56, 84, 43, 69, 53, 117, 43, 50, 85, 105, 90, 88, 50, 65, 47, 119, 49, 82, 108, 76, 111, 47, 43, 109, 107, 88, 89, 107, 68, 90, 116, 53, 79, 118, 54, 43, 85, 55, 100, 108, 83, 120, 97, 107, 51, 113, 65, 52, 105, 117, 73, 52, 111, 107, 81, 61, 61, 0, 56, 47, 53, 48, 107, 86, 69, 51, 110, 87, 109, 65, 114, 75, 101, 68, 57, 78, 121, 89, 78, 114, 104, 114, 49, 103, 100, 76, 110, 101, 68, 102, 122, 53, 99, 99, 68, 121, 57, 112, 89, 70, 50, 111, 121, 108, 121, 101, 122, 109, 114, 115, 72, 48, 65, 61, 61, 0, 108, 83, 73, 52, 114, 55, 107, 118, 88, 65, 118, 77, 105, 111, 100, 76, 69, 87, 75, 56, 85, 103, 116, 86, 121, 49, 66, 118, 106, 105, 106, 106, 87, 80, 72, 85, 69, 43, 101, 80, 49, 50, 84, 74, 88, 68, 71, 72, 86, 90, 48, 80, 54, 78, 79, 74, 101, 119, 85, 51, 75, 110, 98, 55, 71, 122, 103, 65, 57, 104, 48, 52, 49, 65, 102, 98, 79, 54, 73, 118, 56, 82, 53, 104, 119, 117, 78, 81, 81, 73, 47, 65, 77, 54, 89, 108, 85, 122, 82, 90, 110, 74, 112, 104, 74, 71, 52, 111, 61, 0, 76, 81, 73, 116, 120, 104, 57, 65, 84, 119, 106, 66, 119, 90, 68, 105, 47, 65, 52, 47, 79, 75, 77, 43, 56, 48, 86, 67, 48, 55, 98, 88, 70, 55, 102, 48, 110, 68, 89, 49, 53, 70, 100, 52, 47, 102, 66, 117, 86, 88, 48, 47, 49, 104, 117, 100, 87, 70, 68, 50, 100, 122, 88, 115, 57, 88, 71, 115, 122, 70, 103, 83, 47, 76, 122, 65, 61, 0, 108, 97, 107, 57, 68, 102, 106, 115, 104, 113, 81, 107, 86, 47, 113, 52, 122, 47, 66, 57, 100, 73, 47, 77, 81, 81, 122, 86, 82, 85, 82, 56, 86, 70, 80, 67, 55, 116, 98, 74, 121, 48, 85, 47, 115, 76, 110, 117, 117, 66, 115, 97, 82, 55, 82, 75, 85, 116, 73, 82, 53, 83, 114, 48, 106, 104, 67, 85, 103, 103, 65, 56, 102, 70, 115, 82, 69, 75, 43, 120, 50, 51, 72, 68, 116, 120, 80, 74, 89, 102, 79, 70, 114, 80, 111, 79, 115, 78, 108, 84, 69, 77, 106, 73, 84, 77, 89, 99, 61, 1, 192, 69, 114, 78, 72, 72, 52, 69, 109, 107, 104, 101, 113, 75, 77, 76, 98, 53, 90, 47, 51, 75, 97, 65, 80, 120, 106, 107, 54, 49, 88, 51, 75, 83, 57, 84, 111, 118, 57, 105, 57, 65, 51, 108, 120, 76, 102, 116, 83, 121, 74, 66, 55, 105, 116, 116, 110, 108, 72, 76, 76, 67, 53, 110, 100, 111, 116, 53, 80, 98, 108, 114, 122, 82, 117, 83, 72, 113, 77, 82, 84, 118, 103, 114, 102, 76, 119, 109, 53, 82, 55, 100, 51, 121, 66, 74, 122, 86, 98, 90, 116, 86, 89, 88, 118, 78, 47, 118, 114, 75, 79, 47, 66, 71, 87, 84, 73, 105, 77, 72, 103, 80, 68, 74, 79, 117, 99, 107, 71, 87, 70, 68, 78, 85, 48, 67, 89, 43, 43, 57, 54, 109, 112, 98, 55, 102, 82, 111, 83, 103, 57, 67, 84, 71, 67, 76, 102, 112, 69, 75, 103, 102, 111, 66, 81, 109, 69, 52, 105, 57, 72, 105, 79, 74, 51, 106, 88, 90, 109, 71, 86, 51, 112, 48, 81, 113, 90, 57, 87, 67, 88, 113, 71, 67, 87, 54, 55, 118, 82, 100, 83, 87, 47, 88, 54, 55, 50, 78, 101, 99, 105, 69, 50, 90, 114, 85, 83, 50, 51, 69, 113, 112, 81, 49, 118, 71, 50, 100, 81, 57, 84, 85, 109, 101, 110, 113, 55, 88, 117, 48, 56, 111, 52, 75, 122, 57, 51, 49, 115, 110, 102, 79, 101, 106, 83, 100, 52, 69, 108, 87, 105, 68, 105, 120, 43, 113, 73, 76, 105, 68, 76, 109, 97, 102, 43, 43, 47, 122, 55, 99, 70, 115, 81, 84, 67, 50, 122, 66, 57, 71, 109, 81, 75, 51, 76, 114, 98, 55, 115, 76, 118, 70, 76, 102, 88, 119, 114, 105, 99, 66, 53, 97, 111, 80, 110, 121, 87, 68, 85, 109, 56, 101, 89, 110, 112, 85, 83, 47, 75, 73, 70, 102, 119, 102, 113, 120, 99, 68, 70, 57, 53, 80, 100, 116, 110, 108, 72, 76, 76, 67, 53, 110, 100, 111, 116, 53, 80, 98, 108, 114, 122, 82, 117, 83, 72, 113, 77, 82, 84, 118, 103, 114, 102, 76, 119, 109, 53, 82, 55, 100, 51, 121, 66, 74, 122, 86, 98, 90, 116, 86, 89, 88, 118, 78, 47, 118, 114, 75, 79, 47, 66, 71, 87, 84, 73, 105, 77, 72, 103, 80, 68, 74, 79, 117, 99, 107, 71, 110, 86, 65, 84, 83, 73, 73, 67, 86, 116, 47, 115, 77, 121, 115, 100, 79, 116, 67, 109, 89, 89, 49, 107, 47, 110, 101, 54, 119, 80, 106, 68];

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "s36.mcskill.net:25565";
    let username = "flowler";

    match query::full_query(addr).await {
        Ok(query_resp) => {
            println!("Full Query Response:");
            println!("MOTD: {}", query_resp.motd);
            println!("Game Type: {}", query_resp.game_type);
            println!("Map: {}", query_resp.map);
            println!(
                "Players: {} / {}",
                query_resp.online_players, query_resp.max_players
            );
            if let Some(plugins) = query_resp.plugins {
                println!("Plugins: {}", plugins);
            }
            if let Some(plugin_list) = query_resp.plugin_list {
                println!("Plugin List: {:?}", plugin_list);
            }
            println!("Player names: {:?}", query_resp.players);
        }
        Err(e) => {
            eprintln!("Error executing full query: {}", e);
        }
    }

    match ping::ping_status(addr).await {
        Ok(ping_resp) => {
            println!("Ping Response:");
            println!("MOTD: {}", ping_resp.motd);
            println!("Version: {} (protocol {})", ping_resp.version_name, ping_resp.protocol);
            println!(
                "Players: {} / {}",
                ping_resp.online_players, ping_resp.max_players
            );
            println!("Latency: {} ms", ping_resp.latency_ms);
        }
        Err(e) => {
            eprintln!("Error executing ping: {}", e);
        }
    }
    

    let mut handles = Vec::new();

    for i in 0..1 {
        let addr = addr.to_string();
        let handle = tokio::spawn(async move {
            if let Ok(conn) = Connection::connect(&addr).await {
                println!("Connection for {} by {} was established!", addr, username);
                let conn = Arc::new(Mutex::new(conn));
                let conn_listener = Arc::clone(&conn);

                let run_handle = tokio::spawn(async move {
                    if let Err(e) = conn_listener.lock().await.run().await {
                        eprintln!("Got error for client {}: {:?}", i, e);
                    }
                });
                {
                    let mut conn_lock = conn.lock().await;
                    let handshake = Handshake {
                        protocol_version: VarInt(5),
                        server_address: VarString(addr.to_string()),
                        server_port: UShort(25565),
                        next_state: VarInt(2),
                    };
                    conn_lock.send_packet(&handshake).await;
                    conn_lock.state = ConnectionState::Login;
                    println!("Handshake for client {} was sent.", username);
                    let login_start = LoginStart {
                        name: VarString(username.to_string()),
                        devices: ByteArrayShort(HWID_BYTES.to_vec()),
                    };

                    conn_lock.send_packet(&login_start).await;
                    println!("LoginStart was sent for client {}", username);
                }

                run_handle.await.expect("run() Error");
            } else {
                eprintln!("Failed to establish connection for {} by {}", addr, username);
            }
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.await.expect("Ошибка в одном из потоков");
    }

    Ok(())
}
