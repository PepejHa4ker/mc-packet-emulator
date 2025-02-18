use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::packet_field;

packet_field! {
    Short(i16) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let s = crate::protocol::io::read_i16_be(r).await?;
            Ok(Short(s))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_i16_be(w, self.0).await
        }
    }
}
