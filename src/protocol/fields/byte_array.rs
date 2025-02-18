use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::packet_field;

packet_field! {
    ByteArray(Vec<u8>) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let arr = crate::protocol::io::read_bytearray(r).await?;
            Ok(ByteArray(arr))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_bytearray(w, &self.0).await
        }
    }
}
