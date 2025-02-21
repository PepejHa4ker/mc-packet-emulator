use crate::protocol::io::read_varint;
use crate::protocol::packets::server::*;
use crate::protocol::packets::*;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use crate::connection::connection_state::ConnectionState;

static DECODED_PACKETS: AtomicUsize = AtomicUsize::new(0);

#[macro_export]
macro_rules! try_decode_packet {
    ($reader:expr, $packet_id:expr, { $( $id:expr => $Type:ident ),* $(,)? }) => {
        match $packet_id {
            $(
                $id => {
                    // println!("Decoding packet with id 0x{:X}", $packet_id);
                    let pkt = $Type::read_from($reader).await?;
                    if $packet_id != 0x26 {
                        // println!("Got packet 0x{:X}({}): {:?}", $packet_id, $packet_id, pkt);
                    }
                    DECODED_PACKETS.fetch_add(1, Ordering::Relaxed);
                    Box::new(pkt) as Box<dyn $crate::protocol::packets::AsyncPacket + Send>
                }
            ),*,
            other => {
                let mut remaining = vec![0u8; 1024];
                let bytes_read = $reader.read(&mut remaining).await.unwrap_or(0);
                remaining.truncate(bytes_read);
                println!("DEBUG: Unknown packet. Packet ID: {:#X}, Remaining data: {:?}", other, &remaining[..]);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unknown packet id: 0x{:X}({}). Data: {:?}", other, other, remaining),
                ));
            }
        }
    }
}


pub async fn read_server_packet_by_state<R>(
    reader: &mut BufReader<R>,
    state: ConnectionState,
) -> io::Result<Box<dyn AsyncPacket + Send>>
where
    R: AsyncRead + Unpin + Send,
{
    let remaining = reader.buffer();
    println!("There are {} remaining bytes", remaining.len());
    let packet_length = read_varint(reader).await?;

    let mut limited_reader = reader.take(packet_length as u64);

    let packet_id = read_varint(&mut limited_reader).await?;

    let packet = match state {
        ConnectionState::Login => Ok(try_decode_packet!(&mut limited_reader, packet_id, {
            0x00 => LoginDisconnect,
            0x01 => EncryptionRequest,
            0x02 => LoginSuccess
        })),
        ConnectionState::Play => {
            let packet = try_decode_packet!(&mut limited_reader, packet_id, {
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
                0x14 => Entity,
                0x15 => EntityRelMove,
                0x16 => EntityLookAndMovement,
                0x17 => EntityLookMove,
                0x18 => EntityTeleport,
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
                0x2E => CloseWindow,
                0x27 => Explosion,
                0x1F => SetExperience,
                0x21 => ChunkData,
                0x2F => SetSlot,
                0x26 => MapChunkBulk,
                0x20 => EntityProperties,
                0x35 => UpdateTileEntity,
                0x29 => SoundEffect,
                0x23 => BlockChange,
                0x28 => Effect,
                0x22 => MultiBlockChange,
            });

            assert_eq!(packet.get_bound(), Bound::Server);
            Ok(packet)
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Unsupported state for decoding: {:?}", state),
        )),
    };

    let remaining = limited_reader.limit();
    if remaining > 0 {
        let mut discard = vec![0u8; remaining as usize];
        limited_reader.read_exact(&mut discard).await?;
    }




    packet
}

