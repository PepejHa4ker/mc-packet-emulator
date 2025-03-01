use std::collections::HashMap;
use std::io;
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};


#[derive(Debug)]
pub struct QueryResponse {
    pub motd: String,
    pub game_type: String,
    pub map: String,
    pub online_players: u32,
    pub max_players: u32,
    pub host_ip: String,
    pub host_port: u16,
    pub plugins: Option<String>,
    pub plugin_list: Option<Vec<String>>,
    pub players: Vec<String>,
}

pub async fn full_query(addr: &str) -> io::Result<QueryResponse> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    // Отправляем handshake-запрос на addr
    socket.send_to(&[0xFE, 0xFD, 0x09, 0, 0, 0, 2], addr).await?;

    let timeout_duration = Duration::from_secs(5);
    let mut recv_buf = [0u8; 256];
    let (n, src) = timeout(timeout_duration, socket.recv_from(&mut recv_buf))
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Timeout waiting for handshake"))??;
    println!("Handshake received from: {:?}", src);
    if n < 5 || recv_buf[0] != 0x09 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid handshake response"));
    }
    let token_bytes = &recv_buf[5..n];
    let token_str = match token_bytes.iter().position(|&b| b == 0) {
        Some(pos) => std::str::from_utf8(&token_bytes[..pos]).unwrap_or(""),
        None => std::str::from_utf8(token_bytes).unwrap_or(""),
    };
    let token_filtered: String = token_str
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    println!("Filtered token: {:?}", token_filtered);
    if token_filtered.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Challenge token is empty after filtering"));
    }
    let token_val: i32 = token_filtered.trim().parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Failed to parse challenge token")
    })?;

    let mut stat_packet = Vec::with_capacity(11);
    stat_packet.extend(&[0xFE, 0xFD, 0x00]);
    stat_packet.extend(&[0, 0, 0, 2]);
    stat_packet.extend(&token_val.to_be_bytes());
    stat_packet.extend(&[0x00, 0x00, 0x00, 0x00]);
    socket.send_to(&stat_packet, addr).await?;

    // Собираем все UDP-пакеты, которые приходят в течение 5 секунд
    let mut full_response = Vec::new();
    loop {
        let mut buf = vec![0u8; 4096];
        match timeout(Duration::from_secs(5), socket.recv_from(&mut buf)).await {
            Ok(Ok((n, src))) if n > 0 => {
                full_response.extend_from_slice(&buf[..n]);
            }
            Ok(Ok((n, src))) => {
                println!("Received 0 bytes from {}", src);
                break;
            }
            Ok(Err(e)) => {
                println!("Error receiving packet: {}", e);
                break;
            }
            Err(_) => {
                println!("Timeout waiting for additional packets");
                break;
            }
        }
    }

    if full_response.is_empty() {
        return Err(io::Error::new(io::ErrorKind::TimedOut, "Timeout waiting for stat response"));
    }

    if full_response.len() < 16 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Stat response too short"));
    }
    let data = &full_response[16..];
    let marker = b"\x00\x00\x01player_\x00\x00";
    let marker_index = data
        .windows(marker.len())
        .position(|w| w == marker)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Player marker not found"))?;
    let info_bytes = &data[..marker_index];
    let players_bytes = &data[marker_index + marker.len()..];

    let info_parts: Vec<&[u8]> = info_bytes.split(|&b| b == 0).collect();
    let mut info_map = HashMap::new();
    for chunk in info_parts.chunks(2) {
        if chunk.len() == 2 {
            let key = std::str::from_utf8(chunk[0]).unwrap_or("").to_lowercase();
            let value = std::str::from_utf8(chunk[1]).unwrap_or("");
            info_map.insert(key, value);
        }
    }

    let motd = info_map.get("hostname").unwrap_or(&"").to_string();
    let game_type = info_map.get("gametype").unwrap_or(&"").to_string();
    let map = info_map.get("map").unwrap_or(&"").to_string();
    let online_players = info_map
        .get("numplayers")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let max_players = info_map
        .get("maxplayers")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let host_ip = info_map.get("hostip").unwrap_or(&"").to_string();
    let host_port = info_map
        .get("hostport")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let (plugins, plugin_list) = if let Some(&plugins_str) = info_map.get("plugins") {
        if !plugins_str.is_empty() {
            let list = if let Some(idx) = plugins_str.find(": ") {
                plugins_str[idx + 2..]
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<String>>()
            } else {
                Vec::new()
            };
            (
                Some(plugins_str.to_string()),
                if list.is_empty() { None } else { Some(list) },
            )
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let mut players = Vec::new();
    if !players_bytes.is_empty() {
        let player_list_bytes = &players_bytes[..players_bytes.len().saturating_sub(2)];
        for name_bytes in player_list_bytes.split(|&b| b == 0) {
            if name_bytes.is_empty() { continue; }
            if let Ok(name) = std::str::from_utf8(name_bytes) {
                players.push(name.to_string());
            }
        }
    }

    Ok(QueryResponse {
        motd,
        game_type,
        map,
        online_players,
        max_players,
        host_ip,
        host_port,
        plugins,
        plugin_list,
        players,
    })
}
