use crate::protocol::fields::{AsyncReadField, AsyncWriteField, VarInt, VarString};
use std::collections::HashMap;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub struct Properties(pub HashMap<<VarString as std::ops::Deref>::Target, VarInt>);

impl Properties {
    pub fn new() -> Self {
        Properties(HashMap::new())
    }
}

#[async_trait::async_trait]
impl AsyncReadField for Properties {
    async fn read_field<R>(r: &mut R) -> std::io::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let size = VarInt::read_field(r).await?.0 as usize;
        let mut map = HashMap::new();
        for _ in 0..size {
            let key = VarString::read_field(r).await?;
            let value = VarInt::read_field(r).await?;
            map.insert((*key).clone(), value);
        }
        Ok(Properties(map))
    }
}

#[async_trait::async_trait]
impl AsyncWriteField for Properties {
    async fn write_field<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let size = self.0.len() as i32;
        VarInt(size).write_field(w).await?;
        for (key, value) in &self.0 {
            let key_wrapper: VarString = VarString(key.clone());
            key_wrapper.write_field(w).await?;
            value.write_field(w).await?;
        }
        Ok(())
    }
}
