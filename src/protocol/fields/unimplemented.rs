use tokio::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use crate::packet_field;

packet_field! {
    Unimplemented(Vec<u8>) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let mut buffer = Vec::new();
            r.read_to_end(&mut buffer).await?;
            Ok(Unimplemented(buffer))
        }
        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_varint(w, self.0.len() as i32).await?;
            w.write_all(&self.0).await?;
            Ok(())
        }
    }
}
