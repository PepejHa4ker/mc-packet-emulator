use crate::packet_field;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

packet_field! {
    Long(i64) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let l = crate::protocol::io::read_i64_be(r).await?;
            Ok(Long(l))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_i64_be(w, self.0).await
        }
    }
}
