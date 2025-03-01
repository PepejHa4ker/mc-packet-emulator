use std::error::Error;
use reqwest::Client;
use sha1::{Sha1, Digest};

const MC_URL: &'static str = "https://launcher.mcskill.net/joinserver1710.php";

fn digest_to_mc_hex(digest: [u8; 20]) -> String {
    let negative = (digest[0] & 0x80) != 0;
    let mut magnitude = digest;

    if negative {
        let mut carry = true;
        for byte in magnitude.iter_mut().rev() {
            *byte = !*byte;
            if carry {
                let (new_val, overflow) = byte.overflowing_add(1);
                *byte = new_val;
                carry = overflow;
            }
        }
    }

    let mut hex_str = hex::encode(magnitude);
    while hex_str.starts_with('0') {
        hex_str.remove(0);
    }
    if hex_str.is_empty() {
        hex_str.push('0');
    }
    if negative {
        format!("-{}", hex_str)
    } else {
        hex_str
    }
}

pub async fn join_auth_server(
    server_id: &str,
    shared_secret: &[u8],
    public_key: &[u8],
    access_token: &str,
    selected_profile: &str,
) -> Result<(), Box<dyn Error>> {
    let mut hasher = Sha1::new();
    hasher.update(server_id.as_bytes());
    hasher.update(shared_secret);
    hasher.update(public_key);
    let digest = hasher.finalize(); 

    let server_hash_hex = digest_to_mc_hex(digest.into());

    println!("Server Hash hex: {}", server_hash_hex);
    let body = serde_json::json!({
        "accessToken": access_token,
        "selectedProfile": selected_profile,
        "serverId": server_hash_hex
    });

    let client = Client::new();
    let resp = client
        .post(MC_URL.to_string())
        .json(&body)
        .send()
        .await?;

    let status = resp.status();

    let text = resp.text().await?;

    if status == reqwest::StatusCode::OK {
        println!("{}", text);
        Ok(())
    } else {
        Err(format!(
            "joinAuthServer failed: status={} body={}",
            status, text
        ))?
    }
}