use crate::packet_field;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

packet_field! {
    VarInt(i32) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let value = crate::protocol::io::read_varint(r).await?;
            Ok(VarInt(value))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_varint(w, self.0).await
        }
    }
}
