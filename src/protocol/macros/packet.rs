use crate::protocol::packets::Bound;

#[macro_export]
macro_rules! __packet_state {
    ($state:ident) => {
        Some($crate::connection::ConnectionState::$state)
    };
    () => {
        None
    };
}

#[macro_export]
macro_rules! packets {
    (
        $( $name:ident ( $id:expr, $bound:ident, $state:ident ) {
            $( $field_name:ident : $field_ty:ty ),* $(,)?
        } ),* $(,)?
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                $( pub $field_name: $field_ty, )*
            }

            impl $name {
                pub const PACKET_ID: i32 = $id;
                pub const PACKET_STATE: Option<$crate::connection::ConnectionState> =
                    Some($crate::connection::ConnectionState::$state);
                pub const BOUND: $crate::protocol::packets::Bound = bound_from_ident!($bound);

                pub fn get_id(&self) -> i32 {
                    Self::PACKET_ID
                }

                pub fn get_state(&self) -> Option<$crate::connection::ConnectionState> {
                    Self::PACKET_STATE
                }

                pub fn get_bound(&self) -> $crate::protocol::packets::Bound {
                    Self::BOUND
                }

                pub async fn read_from<R>(reader: &mut R) -> std::io::Result<Self>
                where
                    R: tokio::io::AsyncRead + std::marker::Unpin + Send,
                {
                    $( let $field_name = <$field_ty as $crate::protocol::fields::AsyncReadField>::read_field(reader).await?; )*
                    assert_eq!(Self::BOUND, crate::protocol::packets::Bound::Server);
                    Ok(Self { $( $field_name, )* })
                }
            }

            #[async_trait::async_trait]
            impl $crate::protocol::packets::AsyncPacket for $name {
                fn get_id(&self) -> i32 {
                    Self::PACKET_ID
                }

                fn get_state(&self) -> Option<$crate::connection::ConnectionState> {
                    Self::PACKET_STATE
                }

                fn get_bound(&self) -> $crate::protocol::packets::Bound {
                    Self::BOUND
                }

                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }

                async fn write_to_boxed(&self, writer: &mut (dyn tokio::io::AsyncWrite + Unpin + Send)) -> std::io::Result<()> {
                    assert_eq!(Self::BOUND, crate::protocol::packets::Bound::Client);
                    use tokio::io::AsyncWriteExt;
                    let mut buf = Vec::new();
                    $crate::protocol::io::write_varint(&mut buf, Self::PACKET_ID).await?;
                    $( <$field_ty as $crate::protocol::fields::AsyncWriteField>::write_field(&self.$field_name, &mut buf).await?; )*
                    let packet_length = buf.len() as i32;
                    let mut full_packet = Vec::new();
                    $crate::protocol::io::write_varint(&mut full_packet, packet_length).await?;
                    full_packet.extend_from_slice(&buf);
                    writer.write_all(&full_packet).await
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! on_packet {
    ($conn:expr, $variant:ident, $handler:expr) => {{
        let packet_id = <$crate::protocol::packets::$variant as $crate::protocol::packets::AsyncPacket>::ID;
        $conn.on(packet_id, move |conn, packet| {
            async move {
                match packet {
                    $crate::protocol::packets::decoder::ServerPacket::$variant(inner) => {
                        $handler(&inner.clone(), conn).await;
                    }
                    _ => {
                        println!(
                            "Получен пакет, не соответствующий ожидаемому типу {}",
                            stringify!($variant)
                        );
                    }
                }
            }
        });
    }};
}
