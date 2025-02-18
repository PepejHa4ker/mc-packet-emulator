use crate::packet_field;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

packet_field! {
    VarString(String) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let s = crate::protocol::io::read_varstring(r).await?;
            Ok(VarString(s))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_varstring(w, &self.0).await
        }
    }
}
