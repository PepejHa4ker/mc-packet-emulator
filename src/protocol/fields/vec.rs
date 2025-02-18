use std::pin::Pin;
use std::future::Future;
use tokio::io::{AsyncRead, AsyncWrite};
use std::io;

impl<T> crate::protocol::fields::AsyncReadField for Vec<T>
where
    T: crate::protocol::fields::AsyncReadField + Send,
{
    fn read_field<'a, R>(r: &'a mut R) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send + 'a>>
    where
        R: AsyncRead + Unpin + Send + 'a,
    {
        Box::pin(async move {
            let count_val = crate::protocol::fields::Short::read_field(r).await?.0;
            let count = if count_val < 0 {
                0
            } else {
                count_val as usize
            };
            let mut vec = Vec::with_capacity(count);
            for _ in 0..count {
                vec.push(T::read_field(r).await?);
            }
            Ok(vec)
        })
    }
}

impl<T> crate::protocol::fields::AsyncWriteField for Vec<T>
where
    T: crate::protocol::fields::AsyncWriteField + Sync,
{
    fn write_field<'a, W>(&'a self, w: &'a mut W)
                          -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>>
    where
        W: AsyncWrite + Unpin + Send + 'a,
    {
        Box::pin(async move {
            crate::protocol::fields::Short(self.len() as i16).write_field(w).await?;
            for item in self {
                item.write_field(w).await?;
            }
            Ok(())
        })
    }
}
