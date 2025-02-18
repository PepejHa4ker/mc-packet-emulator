use crate::protocol::fields::{AsyncReadField, AsyncWriteField, VarInt, VarString};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub struct Properties(pub HashMap<<VarString as std::ops::Deref>::Target, VarInt>);

impl Properties {
    pub fn new() -> Self {
        Properties(HashMap::new())
    }
}

impl AsyncReadField for Properties {
    fn read_field<'a, R>(
        r: &'a mut R,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<Self>> + Send + 'a>>
    where
        R: AsyncRead + Unpin + Send + 'a,
    {
        Box::pin(async move {
            let size = VarInt::read_field(r).await?.0 as usize;
            let mut map = HashMap::new();
            for _ in 0..size {
                let key = <VarString as AsyncReadField>::read_field(r).await?;
                let value = <VarInt as AsyncReadField>::read_field(r).await?;
                map.insert((*key).clone(), value);
            }
            Ok(Properties(map))
        })
    }
}

impl AsyncWriteField for Properties {
    fn write_field<'a, W>(
        &'a self,
        w: &'a mut W,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<()>> + Send + 'a>>
    where
        W: AsyncWrite + Unpin + Send + 'a,
    {
        Box::pin(async move {
            let size = self.0.len() as i32;
            VarInt(size).write_field(w).await?;
            for (key, value) in &self.0 {
                let key_wrapper: VarString = VarString(key.clone());
                key_wrapper.write_field(w).await?;
                value.write_field(w).await?;
            }
            Ok(())
        })
    }
}
