use crate::client_packets;
use crate::protocol::fields::*;

client_packets! {
    pub enum ClientPacket {
        Handshake (0x00, Handshaking) {
            protocol_version: VarInt,
            server_address: VarString,
            server_port: UShort,
            next_state: VarInt
        },
        LoginStart (0x00, Login) {
            name: VarString
        },
        KeepAlive (0x00, Play) {
            keep_alive_id: Int
        },
        ChatMessage (0x01, Play) {
            message: VarString
        },
        PlayerPosLook (0x06, Play) {
            x: Double,
            y: Double,
            stance: Double,
            z: Double,
            yaw: Float,
            pitch: Float,
            on_ground: Boolean
        },
        ClientSettings (0x15, Play) {
            locale: VarString,
            view_distance: Byte,
            chat_flags: Byte,
            chat_colors: Boolean,
            difficulty: Byte,
            show_cape: Boolean
        },
        ClientStatus (0x16,  Play) {
            action_id: VarInt
        },
        CustomPayload (0x17, Play) {
            channel: VarString,
            data: ByteArrayShort
        }
    }
}
