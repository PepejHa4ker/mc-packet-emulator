use crate::packet_field;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};

packet_field! {
    ByteArrayVarInt(Vec<u8>) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let arr = crate::protocol::io::read_bytearray_varint(r).await?;
            Ok(ByteArrayVarInt(arr))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_bytearray_varint(w, &self.0).await
        }
    }
}

packet_field! {
    ByteArrayShort(Vec<u8>) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let arr = crate::protocol::io::read_bytearray_varint(r).await?;
            Ok(ByteArrayShort(arr))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_bytearray_varint(w, &self.0).await
        }
    }
}



packet_field! {
    ByteArrayInt(Vec<u8>) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let arr = crate::protocol::io::read_bytearray_int(r).await?;
            Ok(ByteArrayInt(arr))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            crate::protocol::io::write_bytearray_int(w, &self.0).await
        }
    }
}
