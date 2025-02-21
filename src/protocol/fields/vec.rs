use std::io;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait::async_trait]
impl<T> crate::protocol::fields::AsyncReadField for Vec<T>
where
    T: crate::protocol::fields::AsyncReadField + Send,
{
    async fn read_field<R>(r: &mut R) -> io::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let count_val = crate::protocol::fields::Short::read_field(r).await?.0;
        let count = if count_val < 0 { 0 } else { count_val as usize };
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            vec.push(T::read_field(r).await?);
        }
        Ok(vec)
    }
}

#[async_trait::async_trait]
impl<T> crate::protocol::fields::AsyncWriteField for Vec<T>
where
    T: crate::protocol::fields::AsyncWriteField + Sync,
{
    async fn write_field<W>(&self, w: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        crate::protocol::fields::Short(self.len() as i16)
            .write_field(w)
            .await?;
        for item in self {
            item.write_field(w).await?;
        }
        Ok(())
    }
}
