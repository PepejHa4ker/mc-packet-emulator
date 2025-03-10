use crate::packet_field;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

packet_field! {
    Float(f32) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let f = crate::protocol::io::read_f32_be(r).await?;
            Ok(Float(f))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_f32_be(w, self.0).await
        }
    }
}
