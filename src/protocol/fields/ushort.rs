use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::packet_field;

packet_field! {
    UShort(u16) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let v = crate::protocol::io::read_u16_be(r).await?;
            Ok(UShort(v))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_u16_be(w, self.0).await
        }
    }
}
