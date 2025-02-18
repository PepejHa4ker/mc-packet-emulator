use async_trait::async_trait;
use std::any::Any;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bound {
    Client,
    Server,
}

#[macro_export]
macro_rules! bound_from_ident {
    (C) => {
        $crate::protocol::packets::Bound::Client
    };
    (S) => {
        $crate::protocol::packets::Bound::Server
    };
}

#[async_trait::async_trait]
pub trait AsyncPacket: Any + Send + Sync {
    fn get_id(&self) -> i32;
    fn get_state(&self) -> Option<ConnectionState>;
    fn get_bound(&self) -> Bound;
    fn as_any(&self) -> &dyn Any;
    async fn write_to_boxed(&self, writer: &mut (dyn AsyncWrite + Unpin + Send)) -> io::Result<()>;
}

pub trait AsyncPacketExt {
    fn as_packet<T: 'static>(&self) -> Option<&T>;
}

impl AsyncPacketExt for dyn AsyncPacket + Send {
    fn as_packet<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub mod decoder;
pub mod handshake;
pub mod login;
pub mod play;

use crate::connection::ConnectionState;
pub use handshake::*;
pub use login::*;
pub use play::*;
