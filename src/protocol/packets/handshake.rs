use crate::packets;
use crate::protocol::fields::varint::VarInt;
use crate::protocol::fields::{UShort, VarString};

packets! {
    Handshake (0x00, C, Handshaking) {
        protocol_version: VarInt,
        server_address: VarString,
        server_port: UShort,
        next_state: VarInt
    }
}
