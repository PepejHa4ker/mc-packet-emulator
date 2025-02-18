use crate::packet_field;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

packet_field! {
    Byte(u8) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let b = crate::protocol::io::read_u8_async(r).await?;
            Ok(Byte(b))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_u8_async(w, self.0).await
        }
    }
}
