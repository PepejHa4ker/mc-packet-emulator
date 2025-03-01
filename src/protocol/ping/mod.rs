use chrono::Utc;
use serde::Deserialize;
use serde_json;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Instant;

use crate::protocol::io::{read_varint, read_varstring, write_varint, write_varstring};

#[derive(Debug)]
pub struct PingResponse {
    pub motd: String,
    pub version_name: String,
    pub protocol: i32,
    pub online_players: u32,
    pub max_players: u32,
    pub latency_ms: u128,
}

#[derive(Deserialize)]
struct StatusResponse {
    version: VersionInfo,
    players: PlayersInfo,
    description: serde_json::Value,
}

#[derive(Deserialize)]
struct VersionInfo {
    name: String,
    protocol: i32,
}

#[derive(Deserialize)]
struct PlayersInfo {
    max: u32,
    online: u32,
}

pub async fn ping_status(addr: &str) -> io::Result<PingResponse> {
    let mut stream = TcpStream::connect(addr).await?;

    let (host, port) = if let Some(pos) = addr.rfind(':') {
        (
            &addr[..pos],
            addr[pos + 1..].parse::<u16>().unwrap_or(25565),
        )
    } else {
        (addr, 25565)
    };
    let protocol_version = 5;
    let mut handshake_data = Vec::new();
    handshake_data.push(0x00);
    let mut buf = Vec::new();
    write_varint(&mut buf, protocol_version).await?;
    handshake_data.extend(buf);
    let mut buf = Vec::new();
    write_varstring(&mut buf, host).await?;
    handshake_data.extend(buf);
    handshake_data.extend(&port.to_be_bytes());
    let mut buf = Vec::new();
    write_varint(&mut buf, 1).await?;
    handshake_data.extend(buf);

    let mut packet = Vec::new();
    let mut buf = Vec::new();
    write_varint(&mut buf, handshake_data.len() as i32).await?;
    packet.extend(buf);
    packet.extend(handshake_data);
    stream.write_all(&packet).await?;

    let mut packet = Vec::new();
    write_varint(&mut packet, 1).await?;
    packet.push(0x00);
    stream.write_all(&packet).await?;

    let _length = read_varint(&mut stream).await?;
    let packet_id = read_varint(&mut stream).await?;
    if packet_id != 0x00 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected status response",
        ));
    }
    let json_response = read_varstring(&mut stream).await?;

    let ping_payload: i64 = Utc::now().timestamp_millis();
    let mut ping_packet = Vec::new();
    ping_packet.push(0x01);
    ping_packet.extend(&ping_payload.to_be_bytes());
    let mut ping_full = Vec::new();
    let mut buf = Vec::new();
    write_varint(&mut buf, ping_packet.len() as i32).await?;
    ping_full.extend(buf);
    ping_full.extend(ping_packet);
    let start = Instant::now();
    stream.write_all(&ping_full).await?;

    let _pong_length = read_varint(&mut stream).await?;
    let pong_id = read_varint(&mut stream).await?;
    println!("Pong_id: 0x{:X}", pong_id);
    if pong_id != 0x01 && pong_id != 0x00 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected pong response",
        ));
    }
    let mut pong_payload = [0u8; 8];
    stream.read_exact(&mut pong_payload).await?;
    let _received_payload = i64::from_be_bytes(pong_payload);
    let latency = start.elapsed().as_millis();

    let status: StatusResponse = serde_json::from_str(&json_response)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON: {}", e)))?;
    let motd = if status.description.is_string() {
        status.description.as_str().unwrap_or("").to_string()
    } else if let Some(text) = status.description.get("text") {
        text.as_str().unwrap_or("").to_string()
    } else {
        status.description.to_string()
    };

    Ok(PingResponse {
        motd,
        version_name: status.version.name,
        protocol: status.version.protocol,
        online_players: status.players.online,
        max_players: status.players.max,
        latency_ms: latency,
    })
}
