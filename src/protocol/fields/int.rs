use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::packet_field;

packet_field! {
    Int(i32) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let i = crate::protocol::io::read_i32_be(r).await?;
            Ok(Int(i))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_i32_be(w, self.0).await
        }
    }
}
