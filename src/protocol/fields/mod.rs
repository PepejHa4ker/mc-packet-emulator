use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait]
pub trait AsyncReadField: Sized {
    async fn read_field<R>(r: &mut R) -> io::Result<Self>
    where
        R: AsyncRead + Unpin + Send;
}

#[async_trait]
pub trait AsyncWriteField {
    async fn write_field<W>(&self, w: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin + Send;
}

pub mod boolean;
pub mod byte;
pub mod byte_array;
pub mod double;
pub mod entity_property;
pub mod float;
pub mod int;
pub mod item_stack;
pub mod long;
pub mod properties;
pub mod short;
pub mod ushort;
pub mod uuid;
pub mod varint;
pub mod varstring;
pub mod vec;
pub mod gameprofile;
pub mod nbt;
pub mod unimplemented;

pub use boolean::Boolean;
pub use byte::Byte;
pub use byte_array::*;
pub use double::Double;
pub use entity_property::EntityProperty;
pub use float::Float;
pub use int::Int;
pub use item_stack::ItemStack;
pub use long::Long;
pub use properties::Properties;
pub use short::Short;
pub use ushort::UShort;
pub use varint::VarInt;
pub use varstring::VarString;
pub use gameprofile::GameProfile;
pub use unimplemented::Unimplemented;
