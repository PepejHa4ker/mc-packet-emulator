#[macro_export]
macro_rules! __packet_state {
    ($state:ident) => {
         Some($crate::connection::connection_state::ConnectionState::$state)
    };
    () => {
         None
    };
}

#[macro_export]
macro_rules! server_packets {
    (
        $(#[$attr:meta])*
        $vis:vis enum $ty_name:ident {
            $(
                $name:ident ( $id:expr, $state:ident ) {
                    $( $field_name:ident : $field_ty:ty ),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $ty_name {
            $( $name($name), )*
        }

        impl $ty_name {
            pub fn try_from(packet: Box<dyn $crate::protocol::packets::AsyncPacket>) -> Option<Self> {
                let id = packet.get_id();
                let state = packet.get_state();
                match (id, state) {
                    $(
                        ($id, $crate::__packet_state!($state)) => {
                            if let Some(concrete) = packet.as_any().downcast_ref::<$name>() {
                                return Some(Self::$name(concrete.clone()));
                            }
                        }
                    )*
                    _ => {}
                }
                None
            }
        }

        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                $( pub $field_name: $field_ty, )*
            }

            impl $name {
                pub const PACKET_ID: i32 = $id;
                pub const PACKET_STATE: Option<$crate::connection::connection_state::ConnectionState> =
                    $crate::__packet_state!($state);
                pub const BOUND: $crate::protocol::packets::Bound =
                    $crate::protocol::packets::Bound::Server;

                pub fn get_id(&self) -> i32 { Self::PACKET_ID }
                pub fn get_state(&self) -> Option<$crate::connection::connection_state::ConnectionState> { Self::PACKET_STATE }
                pub fn get_bound(&self) -> $crate::protocol::packets::Bound { Self::BOUND }

                #[allow(unused_variables)]
                pub async fn read_from<R>(reader: &mut R) -> std::io::Result<Self>
                where R: tokio::io::AsyncRead + std::marker::Unpin + Send {
                    $(
                        let $field_name = <$field_ty as $crate::protocol::fields::AsyncReadField>::read_field(reader).await?;
                    )*
                    Ok(Self { $($field_name),* })
                }
            }

            #[async_trait::async_trait]
            impl $crate::protocol::packets::AsyncPacket for $name {
                fn get_id(&self) -> i32 { Self::PACKET_ID }
                fn get_state(&self) -> Option<$crate::connection::connection_state::ConnectionState> { Self::PACKET_STATE }
                fn get_bound(&self) -> $crate::protocol::packets::Bound { Self::BOUND }
                fn as_any(&self) -> &dyn std::any::Any { self }
                async fn write_to_boxed(&self, _writer: &mut (dyn tokio::io::AsyncWrite + Unpin + Send)) -> std::io::Result<()> {
                    if Self::BOUND != $crate::protocol::packets::Bound::Client {
                        panic!("write_to_boxed() called on a server-bound packet: {:?}", Self::PACKET_ID);
                    }
                    Ok(())
                }
            }
        )*

        paste::paste! {
            impl $ty_name {
                pub async fn handle_by<H>(self, handler: &mut H)
                where H: [<$ty_name Handler>] {
                    match self {
                        $(
                            Self::$name(v) => {
                                handler.[<handle_ $name:snake>](v).await;
                            }
                        )*
                    }
                }

                $(
                    pub fn [<$name:snake>]($($field_name: $field_ty),*) -> Self {
                        Self::$name($name { $($field_name),* })
                    }
                )*
            }

            pub trait [<$ty_name Handler>] {
                $(
                    async fn [<handle_ $name:snake>](&mut self, _packet: $name) {
                        // NOOP
                    }
                )*
            }
        }
    }
}



#[macro_export]
macro_rules! client_packets {
    (
        $(#[$attr:meta])*
        $vis:vis enum $ty_name:ident {
            $(
                $name:ident ( $id:expr, $state:ident ) {
                    $( $field_name:ident : $field_ty:ty ),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $ty_name {
            $( $name($name), )*
        }

        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                $( pub $field_name: $field_ty, )*
            }

            impl $name {
                pub const PACKET_ID: i32 = $id;
                pub const PACKET_STATE: Option<$crate::connection::connection_state::ConnectionState> =
                    $crate::__packet_state!($state);
                pub const BOUND: $crate::protocol::packets::Bound =
                    $crate::protocol::packets::Bound::Client;

                pub fn get_id(&self) -> i32 { Self::PACKET_ID }
                pub fn get_state(&self) -> Option<$crate::connection::connection_state::ConnectionState> { Self::PACKET_STATE }
                pub fn get_bound(&self) -> $crate::protocol::packets::Bound { Self::BOUND }

                #[allow(unused_variables)]
                pub async fn read_from<R>(reader: &mut R) -> std::io::Result<Self>
                where R: tokio::io::AsyncRead + std::marker::Unpin + Send {
                    if Self::BOUND != $crate::protocol::packets::Bound::Server {
                        panic!("read_from() called on a client-bound packet: {:?}", Self::PACKET_ID);
                    }
                    $(
                        let $field_name = <$field_ty as $crate::protocol::fields::AsyncReadField>::read_field(reader).await?;
                    )*
                    Ok(Self { $($field_name),* })
                }
            }

            #[async_trait::async_trait]
            impl $crate::protocol::packets::AsyncPacket for $name {
                fn get_id(&self) -> i32 { Self::PACKET_ID }
                fn get_state(&self) -> Option<$crate::connection::connection_state::ConnectionState> { Self::PACKET_STATE }
                fn get_bound(&self) -> $crate::protocol::packets::Bound { Self::BOUND }
                fn as_any(&self) -> &dyn std::any::Any { self }
                async fn write_to_boxed(&self, writer: &mut (dyn tokio::io::AsyncWrite + Unpin + Send)) -> std::io::Result<()> {
                    if Self::BOUND != $crate::protocol::packets::Bound::Client {
                        panic!("write_to_boxed() called on a server-bound packet: {:?}", Self::PACKET_ID);
                    }
                    use tokio::io::AsyncWriteExt;
                    let mut buf = Vec::new();
                    $crate::protocol::io::write_varint(&mut buf, Self::PACKET_ID).await?;
                    $(
                        <$field_ty as $crate::protocol::fields::AsyncWriteField>::write_field(&self.$field_name, &mut buf).await?;
                    )*
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
