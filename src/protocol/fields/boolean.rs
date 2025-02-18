use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::packet_field;

packet_field! {
    Boolean(bool) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let b = crate::protocol::io::read_bool(r).await?;
            Ok(Boolean(b))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_bool(w, self.0).await
        }
    }
}
