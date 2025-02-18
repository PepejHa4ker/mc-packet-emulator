use crate::packet_field;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

packet_field! {
    Uuid(uuid::Uuid) {
        async fn read(r: &mut impl AsyncRead + Unpin + Send) -> io::Result<Self> {
            let value = crate::protocol::io::read_uuid(r).await?;
            Ok(Uuid(value))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin + Send) -> io::Result<()> {
            crate::protocol::io::write_uuid(w, &self.0).await
        }
    }
}
