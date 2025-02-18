use tokio::io;
use std::pin::Pin;
use std::future::Future;

pub trait AsyncReadField: Sized {
    fn read_field<'a, R>(r: &'a mut R) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send + 'a>>
    where
        R: io::AsyncRead + Unpin + Send + 'a;
}

pub trait AsyncWriteField {
    fn write_field<'a, W>(&'a self, w: &'a mut W) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>>
    where
        W: io::AsyncWrite + Unpin + Send + 'a;
}

pub mod varint;
pub mod varstring;
pub mod byte;
pub mod short;
pub mod ushort;
pub mod int;
pub mod long;
pub mod float;
pub mod double;
pub mod boolean;
pub mod byte_array;
pub mod properties;
pub mod item_stack;
pub mod vec;
pub mod uuid;
pub mod entity_property;

pub use varint::VarInt;
pub use varstring::VarString;
pub use byte::Byte;
pub use short::Short;
pub use ushort::UShort;
pub use int::Int;
pub use long::Long;
pub use float::Float;
pub use double::Double;
pub use boolean::Boolean;
pub use byte_array::ByteArray;
pub use properties::Properties;
pub use item_stack::ItemStack;
pub use uuid::Uuid;
pub use entity_property::EntityProperty;
pub use entity_property::EntityAttributeModifier;
