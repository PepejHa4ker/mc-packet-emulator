use crate::connection::ConnectionState;
use crate::packets;
use crate::protocol::fields::*;
use crate::protocol::packets::AsyncPacket;
use async_trait::async_trait;
use flate2::{Decompress, FlushDecompress};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use tokio::{io, task};

packets! {
    EncryptionRequest (0x01, S, Login) {
        server_id: VarString,
        public_key: ByteArray,
        verify_token: ByteArray
    },
    LoginSuccess (0x02, S, Login) {
        uuid: VarString,
        username: VarString
    },
    LoginDisconnect (0x00, S, Login) {
        reason: VarString
    },
    KeepAlive (0x00, S, Play) {
        keep_alive_id: Int
    },
    JoinGame (0x01, S, Play) {
        entity_id: Int,
        game_mode: Byte,
        dimension: Byte,
        difficulty: Byte,
        max_players: Byte,
        level_type: VarString
    },
    CChatMessage (0x01, C, Play) {
        message: VarString
    },
    SChatMessage (0x02, S, Play) {
        json_data: VarString
    },
    TimeUpdate (0x03, S, Play) {
        world_age: Long,
        time_of_day: Long
    },
    EntityEquipment (0x04, S, Play) {
        entity_id: Int,
        slot: Short,
        item: Int
    },
    SpawnPosition (0x05, S, Play) {
        x: Int,
        y: Int,
        z: Int
    },
    UpdateHealth (0x06, S, Play) {
        health: Float,
        food: VarInt,
        saturation: Float
    },
    Respawn (0x07, S, Play) {
        dimension: Int,
        difficulty: Byte,
        game_mode: Byte,
        level_type: VarString
    },
    PlayerPositionAndLook (0x08, S, Play) {
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        on_ground: Boolean
    },
    HeldItemChange (0x09, S, Play) {
        slot: Byte
    },
    UseBed (0x0A, S, Play) {
        entity_id: Int,
        bed_x: Int,
        bed_y: Byte,
        bed_z: Int
    },
    Animation (0x0B, S, Play) {
        entity_id: Int,
        animation: Byte
    },
    SpawnPlayer (0x0C, S, Play) {
        entity_id: Int,
        player_uuid: VarString,
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        current_item: Byte
    },
    CollectItem (0x0D, S, Play) {
        collector_entity_id: Int,
        collected_entity_id: Int,
        pickup_count: Short
    },
    SpawnObject (0x0E, S, Play) {
        entity_id: Int,
        ty: Byte,
        x: Int,
        y: Int,
        z: Int,
        pitch: Byte,
        yaw: Byte,
        extra: Int
    },
    SpawnMob (0x0F, S, Play) {
        entity_id: Int,
        mob_type: Byte,
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        head_yaw: Float,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short
    },
    SpawnPainting (0x10, S, Play) {
        entity_id: Int,
        title: VarString,
        x: Int,
        y: Int,
        z: Int,
        direction: Byte
    },
    SpawnExperienceOrb (0x11, S, Play) {
        entity_id: Int,
        x: Double,
        y: Double,
        z: Double,
        count: Short
    },
    EntityVelocity (0x12, S, Play) {
        entity_id: Int,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short
    },
    DestroyEntities (0x13, S, Play) {
        count: VarInt,
        entity_ids: ByteArray
    },
    EntityMovement (0x14, S, Play) {
        entity_id: Int,
        delta_x: Byte,
        delta_y: Byte,
        delta_z: Byte,
        on_ground: Boolean
    },
    EntityLook (0x15, S, Play) {
        entity_id: Int,
        yaw: Byte,
        pitch: Byte,
        on_ground: Boolean
    },
    EntityLookAndMovement (0x16, S, Play) {
        entity_id: Int,
        delta_x: Byte,
        delta_y: Byte,
        delta_z: Byte,
        yaw: Byte,
        pitch: Byte,
        on_ground: Boolean
    },
    EntityTeleport (0x17, S, Play) {
        entity_id: Int,
        x: Double,
        y: Double,
        z: Double,
        yaw: Byte,
        pitch: Byte,
        on_ground: Boolean
    },
    EntityHeadLook (0x18, S, Play) {
        entity_id: Int,
        head_yaw: Byte
    },
    EntityStatus (0x19, S, Play) {
        entity_id: Int,
        status: Byte
    },
    AttachEntity (0x1A, S, Play) {
        entity_id: Int,
        vehicle_id: Int
    },
    EntityMetadata (0x1B, S, Play) {
        entity_id: Int,
        metadata: ByteArray
    },
    EntityEffect (0x1C, S, Play) {
        entity_id: Int,
        effect_id: Byte,
        amplifier: Byte,
        duration: VarInt
    },
    RemoveEntityEffect (0x1D, S, Play) {
        entity_id: Int,
        effect_id: Byte
    },
    Experience (0x1E, S, Play) {
        experience_bar: Float,
        total_experience: VarInt,
        level: VarInt
    },
    PlayerAbilities (0x39, S, Play) {
        flags: Byte,
        fly_speed: Float,
        walk_speed: Float
    },
    CustomPayload (0x3F, S, Play) {
        channel: VarString,
        data: ByteArray
    },
    Statistics (0x37, S, Play) {
        stats: Properties
    },
    PlayerListItem (0x38, S, Play) {
        username: VarString,
        gamemode: Byte,
        ping: Short
    },
    ChangeGameState(0x2B, S, Play) {
        reason: Byte,
        value: Float,
    },
    WindowItems (0x30, S, Play) {
        window_id: Byte,
        slot_count: Short,
        items: Vec<ItemStack>
    },
    SetSlot (0x2F, S, Play) {
        window_id: Byte,
        slot: Short,
        item: ItemStack
    },
    EntityProperties (0x20, S, Play) {
        entity_id: Int,
        properties: Vec<EntityProperty>
    }
}


#[derive(Debug, Clone)]
pub struct MapChunkBulk {
    pub chunk_x: Vec<i32>,
    pub chunk_z: Vec<i32>,
    pub section_mask: Vec<i32>,
    pub add_mask: Vec<i32>,
    pub compressed_data: Vec<u8>,
    pub chunk_data: Vec<Vec<u8>>,
    pub compressed_length: i32,
    pub has_sky: bool,
    pub chunk_count: usize,
}

impl MapChunkBulk {
    pub async fn read_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let chunk_count = Short::read_field(reader).await?.0 as usize;
        let compressed_length = Int::read_field(reader).await?.0;
        let has_sky = Boolean::read_field(reader).await?.0;

        let mut compressed_data = vec![0u8; compressed_length as usize];
        reader.read_exact(&mut compressed_data).await?;

        let expected_decompressed_size = chunk_data_multiplier() * chunk_count;
        let compressed_data_clone = compressed_data.clone();
        let decompressed_buf = task::spawn_blocking(move || -> io::Result<Vec<u8>> {
            let mut decompressor = Decompress::new(true);
            let mut output = vec![0u8; expected_decompressed_size];
            decompressor.decompress(&compressed_data_clone, &mut output, FlushDecompress::Finish)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Bad compressed data format"))?;
            output.truncate(decompressor.total_out() as usize);
            Ok(output)
        }).await??;

        let mut chunk_x = Vec::with_capacity(chunk_count);
        let mut chunk_z = Vec::with_capacity(chunk_count);
        let mut section_mask = Vec::with_capacity(chunk_count);
        let mut add_mask = Vec::with_capacity(chunk_count);
        let mut chunk_data = Vec::with_capacity(chunk_count);

        let mut offset: i32 = 0;
        for _ in 0..chunk_count {
            let x = Int::read_field(reader).await?.0;
            let z = Int::read_field(reader).await?.0;
            let sec_mask = Short::read_field(reader).await?.0 as i32;
            let a_mask = Short::read_field(reader).await?.0 as i32;

            chunk_x.push(x);
            chunk_z.push(z);
            section_mask.push(sec_mask);
            add_mask.push(a_mask);

            let mut k = 0;
            let mut l = 0;
            for i in 0..16 {
                k += (sec_mask >> i) & 1;
                l += (a_mask >> i) & 1;
            }

            let mut chunk_len = 2048 * 4 * k + 256;
            chunk_len += 2048 * l;
            if has_sky {
                chunk_len += 2048 * k;
            }

            if offset + chunk_len > decompressed_buf.len() as i32 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough decompressed data"));
            }
            let data = decompressed_buf[offset as usize..(offset + chunk_len) as usize].to_vec();
            chunk_data.push(data);
            offset += chunk_len;
        }

        Ok(MapChunkBulk {
            chunk_x,
            chunk_z,
            section_mask,
            add_mask,
            compressed_data,
            chunk_data,
            compressed_length,
            has_sky,
            chunk_count,
        })
    }
}

const fn chunk_data_multiplier() -> usize {
    196864
}

#[async_trait]
impl AsyncPacket for MapChunkBulk {
    fn get_id(&self) -> i32 {
        0x26
    }
    fn get_state(&self) -> Option<ConnectionState> {
        Some(ConnectionState::Play)
    }

    fn get_bound(&self) -> crate::protocol::packets::Bound {
        crate::protocol::packets::Bound::Server
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn write_to_boxed(&self, _: &mut (dyn AsyncWrite + Unpin + Send)) -> io::Result<()>
    {
        unimplemented!("Write for MapChunkBulk is not implemented")
    }
}

