use crate::protocol::fields::{Byte, Short};
use async_trait::async_trait;
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
    async fn read_field<R>(r: &mut R) -> io::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
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
    }
}

#[async_trait]
impl crate::protocol::fields::AsyncWriteField for ItemStack {
    async fn write_field<W>(&self, w: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        if self.item_id == -1 {
            Short(-1).write_field(w).await
        } else {
            Short(self.item_id).write_field(w).await?;
            Byte(self.count).write_field(w).await?;
            Short(self.damage).write_field(w).await
        }
    }
}
