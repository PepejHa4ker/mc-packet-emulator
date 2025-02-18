use std::future::Future;
use std::pin::Pin;
use tokio::io;

pub trait AsyncReadField: Sized {
    fn read_field<'a, R>(
        r: &'a mut R,
    ) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send + 'a>>
    where
        R: io::AsyncRead + Unpin + Send + 'a;
}

pub trait AsyncWriteField {
    fn write_field<'a, W>(
        &'a self,
        w: &'a mut W,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>>
    where
        W: io::AsyncWrite + Unpin + Send + 'a;
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

pub use boolean::Boolean;
pub use byte::Byte;
pub use byte_array::ByteArray;
pub use double::Double;
pub use entity_property::EntityAttributeModifier;
pub use entity_property::EntityProperty;
pub use float::Float;
pub use int::Int;
pub use item_stack::ItemStack;
pub use long::Long;
pub use properties::Properties;
pub use short::Short;
pub use ushort::UShort;
pub use uuid::Uuid;
pub use varint::VarInt;
pub use varstring::VarString;
