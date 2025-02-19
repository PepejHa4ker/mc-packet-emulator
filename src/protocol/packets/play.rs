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
    SKeepAlive (0x00, S, Play) {
        keep_alive_id: Int
    },
    CKeepAlive (0x00, C, Play) {
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
        entity_id: VarInt,
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
        food: Short,
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
        entity_id: VarInt,
        bed_x: Int,
        bed_y: Byte,
        bed_z: Int
    },
    Animation (0x0B, S, Play) {
        entity_id: VarInt,
        animation: Byte
    },
    SpawnPlayer (0x0C, S, Play) {
        entity_id: VarInt,
        profile: GameProfile,
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        current_item: Byte
    },
    CollectItem (0x0D, S, Play) {
        collector_entity_id: VarInt,
        collected_entity_id: VarInt,
        pickup_count: Short
    },
    SpawnObject (0x0E, S, Play) {
        entity_id: VarInt,
        ty: Byte,
        x: Int,
        y: Int,
        z: Int,
        pitch: Byte,
        yaw: Byte,
        extra: Int
    },
    SpawnMob (0x0F, S, Play) {
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
    SpawnPainting (0x10, S, Play) {
        entity_id: VarInt,
        title: VarString,
        x: Int,
        y: Int,
        z: Int,
        direction: Byte
    },
    SpawnExperienceOrb (0x11, S, Play) {
        entity_id: VarInt,
        x: Int,
        y: Int,
        z: Int,
        count: Short
    },
    EntityVelocity (0x12, S, Play) {
        entity_id: VarInt,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short
    },
    DestroyEntities (0x13, S, Play) {
        count: VarInt,
        entity_ids: ByteArray
    },
    Entity(0x14, S, Play) {
        entity_id: Int,
    },
    EntityRelMove (0x15, S, Play) {
        entity_id: Int,
        x: Byte,
        y: Byte,
        z: Byte
    },
    EntityLookAndMovement (0x16, S, Play) {
        entity_id: Int,
        yaw: Byte,
        pitch: Byte
    },
    EntityLookMove (0x17, S, Play) {
       entity_id: Int,
        x: Byte,
        y: Byte,
        z: Byte,
        yaw: Byte,
        pitch: Byte
    },
    ClientSettings (0x15, C, Play) {
        locale: VarString,
        view_distance: Byte,
        chat_flags: Byte,
        chat_colors: Boolean,
        difficulty: Byte,
        show_cape: Boolean
    },
    ClientStatus (0x16, C, Play) {
        action_id: VarInt
    },
    SetExperience (0x1F, S, Play) {
        experience_bar: Float,
        level: Short,
        total_experience: Short
    },
    EntityTeleport (0x18, S, Play) {
        entity_id: Int,
        x: Int,
        y: Int,
        z: Int,
        yaw: Byte,
        pitch: Byte,
    },
    EntityStatus (0x19, S, Play) {
        entity_id: VarInt,
        status: Byte
    },
    AttachEntity (0x1A, S, Play) {
        entity_id: VarInt,
        vehicle_id: Int
    },
    EntityMetadata (0x1B, S, Play) {
        entity_id: VarInt,
        metadata: ByteArray
    },
    EntityEffect (0x1C, S, Play) {
        entity_id: VarInt,
        effect_id: Byte,
        amplifier: Byte,
        duration: VarInt
    },
    RemoveEntityEffect (0x1D, S, Play) {
        entity_id: VarInt,
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
    CloseWindow (0x2E, S, Play) {
        window_id: Byte
    },
    Explosion (0x27, S, Play) {
        unimpl: Unimplemented
    },
    EntityProperties (0x20, S, Play) {
        entity_id: Int,
        properties: Vec<EntityProperty>
    },
    UpdateTileEntity (0x35, S, Play) {
        unimpl: Unimplemented
    },
    MapChunkBulk (0x26, S, Play) {
        unimpl: Unimplemented
    },
    ChunkData (0x21, S, Play) {
        unimpl: Unimplemented
    },
    SoundEffect (0x29, S, Play) {
        sound_name: VarString,
        x: Int,
        y: Int,
        z: Int,
        volume: Float,
        pitch: Byte
    },
    BlockChange (0x23, S, Play) {
        x: Int,
        y: Byte,
        z: Int,
        block_id: VarInt,
        meta: Byte
    },
    Effect (0x28, S, Play) {
        effect_id: Int,
        x: Int,
        y: Byte,
        z: Int,
        data: Int,
        disable_relative: Boolean
    },
    MultiBlockChange (0x22, S, Play) {
        unimpl: Unimplemented
    },
    CPlayerPosLook (0x06, C, Play) {
        x: Double,
        y: Double,
        stance: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        on_ground: Boolean
    }
}

use crate::packets;

use crate::protocol::fields::*;



























































































































