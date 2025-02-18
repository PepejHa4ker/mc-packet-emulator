use std::io;
use std::io::Cursor;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::connection::ConnectionState;
use crate::protocol::io::read_varint;
use crate::protocol::packets::{AsyncPacket, Bound, PlayerAbilities};
use crate::protocol::packets::play::*;

#[macro_export]
macro_rules! try_decode_packet {
    ($cursor:expr, $packet_id:expr, { $( $id:expr => $Type:ident ),* $(,)? }) => {
        match $packet_id {
            $(
                $id => {
                    let pkt = $Type::read_from(&mut $cursor).await?;
                    if $packet_id != 0x26 {
                        println!("Got packet {:?} 0x{:X} ({})", pkt, $packet_id, $packet_id);
                    }
                    Box::new(pkt) as Box<dyn $crate::protocol::packets::AsyncPacket + Send>
                }
            ),*,
            other => {
                let pos = $cursor.position() as usize;
                let remaining = $cursor.into_inner()[pos..].to_vec();
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unknown packet id: {}. Data: {:?}", other, remaining)));
            }
        }
    }
}



pub async fn read_server_packet_by_state<R>(
    reader: &mut R,
    state: ConnectionState,
) -> io::Result<Box<dyn AsyncPacket + Send>>
where
    R: AsyncRead + Unpin + Send,
{
    let packet_length = read_varint(reader).await?;
    let mut buf = vec![0u8; packet_length as usize];
    reader.read_exact(&mut buf).await?;
    let mut cursor = Cursor::new(buf);

    let packet_id = read_varint(&mut cursor).await?;

    match state {
        ConnectionState::Login => {
            Ok(try_decode_packet!(cursor, packet_id, {
                0x00 => LoginDisconnect,
                0x01 => EncryptionRequest,
                0x02 => LoginSuccess
            }))
        }
        ConnectionState::Play => {
            let packet = try_decode_packet!(cursor, packet_id, {
                0x00 => KeepAlive,
                0x01 => JoinGame,
                0x02 => SChatMessage,
                0x03 => TimeUpdate,
                0x04 => EntityEquipment,
                0x05 => SpawnPosition,
                0x06 => UpdateHealth,
                0x07 => Respawn,
                0x08 => PlayerPositionAndLook,
                0x09 => HeldItemChange,
                0x0A => UseBed,
                0x0B => Animation,
                0x0C => SpawnPlayer,
                0x0D => CollectItem,
                0x0E => SpawnObject,
                0x0F => SpawnMob,
                0x10 => SpawnPainting,
                0x11 => SpawnExperienceOrb,
                0x12 => EntityVelocity,
                0x13 => DestroyEntities,
                0x14 => EntityMovement,
                0x15 => EntityLook,
                0x16 => EntityLookAndMovement,
                0x17 => EntityTeleport,
                0x18 => EntityHeadLook,
                0x19 => EntityStatus,
                0x1A => AttachEntity,
                0x1B => EntityMetadata,
                0x1C => EntityEffect,
                0x1D => RemoveEntityEffect,
                0x1E => Experience,
                0x37 => Statistics,
                0x38 => PlayerListItem,
                0x39 => PlayerAbilities,
                0x3F => CustomPayload,
                0x2B => ChangeGameState,
                0x30 => WindowItems,
                0x2F => SetSlot,
                0x26 => MapChunkBulk,
                0x20 => EntityProperties,
            });
            assert_eq!(packet.get_bound(), Bound::Server);
            Ok(packet)
        }
        _ => {
            let pos = cursor.position() as usize;
            let remaining = cursor.into_inner()[pos..].to_vec();
            Err(io::Error::new(io::ErrorKind::Other, format!("Unsupported state for decoding: {:?}, packet_id: {}. Data: {:?}", state, packet_id, remaining)))
        }
    }
}

