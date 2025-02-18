use crate::protocol::fields::{Byte, Short};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item_id: i16,
    pub count: u8,
    pub damage: i16,
}

#[async_trait]
impl crate::protocol::fields::AsyncReadField for ItemStack {
    fn read_field<'a, R>(
        r: &'a mut R,
    ) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send + 'a>>
    where
        R: AsyncRead + Unpin + Send + 'a,
    {
        Box::pin(async move {
            let id = Short::read_field(r).await?.0;
            if id == -1 {
                Ok(ItemStack {
                    item_id: -1,
                    count: 0,
                    damage: 0,
                })
            } else {
                let count = Byte::read_field(r).await?.0;
                let damage = Short::read_field(r).await?.0;
                Ok(ItemStack {
                    item_id: id,
                    count,
                    damage,
                })
            }
        })
    }
}

#[async_trait]
impl crate::protocol::fields::AsyncWriteField for ItemStack {
    fn write_field<'a, W>(
        &'a self,
        w: &'a mut W,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>>
    where
        W: AsyncWrite + Unpin + Send + 'a,
    {
        Box::pin(async move {
            if self.item_id == -1 {
                Short(-1).write_field(w).await
            } else {
                Short(self.item_id).write_field(w).await?;
                Byte(self.count).write_field(w).await?;
                Short(self.damage).write_field(w).await
            }
        })
    }
}
