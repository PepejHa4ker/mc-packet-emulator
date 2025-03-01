use crate::protocol::fields::*;
use crate::server_packets;

server_packets! {
    pub enum ServerPacket {
        LoginDisconnect (0x00, Login) {
            reason: VarString
        },
        EncryptionRequest (0x01, Login) {
            server_id: VarString,
            public_key: ByteArrayShort,
            verify_token: ByteArrayShort
        },
        LoginSuccess (0x02, Login) {
            uuid: VarString,
            username: VarString
        },
        KeepAlive (0x00, Play) {
            keep_alive_id: Int
        },
        JoinGame (0x01, Play) {
            entity_id: Int,
            game_mode: Byte,
            dimension: Byte,
            difficulty: Byte,
            max_players: Byte,
            level_type: VarString
        },
        SChatMessage (0x02, Play) {
            json_data: VarString
        },
        TimeUpdate (0x03, Play) {
            world_age: Long,
            time_of_day: Long
        },
        EntityEquipment (0x04, Play) {
            entity_id: VarInt,
            slot: Short,
            item: Int
        },
        SpawnPosition (0x05, Play) {
            x: Int,
            y: Int,
            z: Int
        },
        UpdateHealth (0x06, Play) {
            health: Float,
            food: Short,
            saturation: Float
        },
        Respawn (0x07, Play) {
            dimension: Int,
            difficulty: Byte,
            game_mode: Byte,
            level_type: VarString
        },
        PlayerPositionAndLook (0x08, Play) {
            x: Double,
            y: Double,
            z: Double,
            yaw: Float,
            pitch: Float,
            on_ground: Boolean
        },
        HeldItemChange (0x09, Play) {
            slot: Byte
        },
        UseBed (0x0A, Play) {
            entity_id: VarInt,
            bed_x: Int,
            bed_y: Byte,
            bed_z: Int
        },
        Animation (0x0B, Play) {
            entity_id: VarInt,
            animation: Byte
        },
        SpawnPlayer (0x0C, Play) {
            entity_id: VarInt,
            profile: GameProfile,
            x: Double,
            y: Double,
            z: Double,
            yaw: Float,
            pitch: Float,
            current_item: Byte
        },
        CollectItem (0x0D, Play) {
            collector_entity_id: VarInt,
            collected_entity_id: VarInt,
            pickup_count: Short
        },
        SpawnObject (0x0E, Play) {
            entity_id: VarInt,
            ty: Byte,
            x: Int,
            y: Int,
            z: Int,
            pitch: Byte,
            yaw: Byte,
            extra: Int
        },
        SpawnMob (0x0F, Play) {
            entity_id: VarInt,
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
        SpawnPainting (0x10, Play) {
            entity_id: VarInt,
            title: VarString,
            x: Int,
            y: Int,
            z: Int,
            direction: Byte
        },
        SpawnExperienceOrb (0x11, Play) {
            entity_id: VarInt,
            x: Int,
            y: Int,
            z: Int,
            count: Short
        },
        EntityVelocity (0x12, Play) {
            entity_id: VarInt,
            velocity_x: Short,
            velocity_y: Short,
            velocity_z: Short
        },
        DestroyEntities (0x13, Play) {
            unimpl: Unimplemented
        },
        Entity (0x14, Play) {
            entity_id: Int,
        },
        EntityRelMove (0x15, Play) {
            entity_id: Int,
            x: Byte,
            y: Byte,
            z: Byte
        },
        EntityLookAndMovement (0x16, Play) {
            entity_id: Int,
            yaw: Byte,
            pitch: Byte
        },
        EntityLookMove (0x17, Play) {
            entity_id: Int,
            x: Byte,
            y: Byte,
            z: Byte,
            yaw: Byte,
            pitch: Byte
        },
        EntityTeleport (0x18, Play) {
            entity_id: Int,
            x: Int,
            y: Int,
            z: Int,
            yaw: Byte,
            pitch: Byte,
        },
        EntityStatus (0x19, Play) {
            entity_id: VarInt,
            status: Byte
        },
        AttachEntity (0x1A, Play) {
            entity_id: VarInt,
            vehicle_id: Int
        },
        EntityMetadata (0x1B, Play) {
            entity_id: VarInt,
            metadata: ByteArrayVarInt
        },
        EntityEffect (0x1C, Play) {
            entity_id: VarInt,
            effect_id: Byte,
            amplifier: Byte,
            duration: VarInt
        },
        RemoveEntityEffect (0x1D, Play) {
            entity_id: VarInt,
            effect_id: Byte
        },
        Experience (0x1E, Play) {
            experience_bar: Float,
            total_experience: VarInt,
            level: VarInt
        },
        SetExperience (0x1F, Play) {
            experience_bar: Float,
            level: Short,
            total_experience: Short
        },
        EntityProperties (0x20, Play) {
            entity_id: Int,
            properties: Vec<EntityProperty>
        },
        ChunkData (0x21, Play) {
            unimpl: Unimplemented
        },
        MultiBlockChange (0x22, Play) {
            unimpl: Unimplemented
        },
        BlockChange (0x23, Play) {
            x: Int,
            y: Byte,
            z: Int,
            block_id: VarInt,
            meta: Byte
        },
        MapChunkBulk (0x26, Play) {
            unimpl: Unimplemented
        },
        Explosion (0x27, Play) {
            unimpl: Unimplemented
        },
        Effect (0x28, Play) {
            effect_id: Int,
            x: Int,
            y: Byte,
            z: Int,
            data: Int,
            disable_relative: Boolean
        },
        SoundEffect (0x29, Play) {
            sound_name: VarString,
            x: Int,
            y: Int,
            z: Int,
            volume: Float,
            pitch: Byte
        },
        ChangeGameState (0x2B, Play) {
            reason: Byte,
            value: Float,
        },
        CloseWindow (0x2E, Play) {
            window_id: Byte
        },
        SetSlot (0x2F, Play) {
            window_id: Byte,
            slot: Short,
            item: ItemStack
        },
        WindowItems (0x30, Play) {
            window_id: Byte,
            slot_count: Short,
            items: Vec<ItemStack>
        },
        UpdateTileEntity (0x35, Play) {
            unimpl: Unimplemented
        },
        Statistics (0x37, Play) {
            stats: Properties
        },
        PlayerListItem (0x38, Play) {
            username: VarString,
            gamemode: Byte,
            ping: Short
        },
        PlayerAbilities (0x39, Play) {
            flags: Byte,
            fly_speed: Float,
            walk_speed: Float
        },
        CustomPayload (0x3F, Play) {
            channel: VarString,
            data: ByteArrayVarInt
        }
    }
}
