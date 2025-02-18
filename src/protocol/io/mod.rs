use tokio::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;
use crate::protocol::fields::{AsyncReadField, AsyncWriteField, Long};

pub async fn read_u16_be<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf).await?;
    Ok(u16::from_be_bytes(buf))
}

pub async fn write_u16_be<W: AsyncWrite + Unpin>(writer: &mut W, value: u16) -> io::Result<()> {
    let bytes = value.to_be_bytes();
    writer.write_all(&bytes).await
}

pub async fn read_bool<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<bool> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).await?;
    Ok(buf[0] != 0)
}

pub async fn read_i8<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<i8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).await?;
    Ok(buf[0] as i8)
}

pub async fn read_u8_async<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).await?;
    Ok(buf[0])
}

pub async fn read_i16_be<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<i16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf).await?;
    Ok(i16::from_be_bytes(buf))
}

pub async fn read_i32_be<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<i32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf).await?;
    Ok(i32::from_be_bytes(buf))
}

pub async fn read_i64_be<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<i64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf).await?;
    Ok(i64::from_be_bytes(buf))
}

pub async fn write_bool<W: AsyncWrite + Unpin>(writer: &mut W, value: bool) -> io::Result<()> {
    let byte = if value { 1 } else { 0 };
    writer.write_all(&[byte]).await
}

pub async fn write_i8<W: AsyncWrite + Unpin>(writer: &mut W, value: i8) -> io::Result<()> {
    writer.write_all(&[value as u8]).await
}

pub async fn write_u8_async<W: AsyncWrite + Unpin>(writer: &mut W, value: u8) -> io::Result<()> {
    writer.write_all(&[value]).await
}

pub async fn write_i16_be<W: AsyncWrite + Unpin>(writer: &mut W, value: i16) -> io::Result<()> {
    let bytes = value.to_be_bytes();
    writer.write_all(&bytes).await
}

pub async fn write_i32_be<W: AsyncWrite + Unpin>(writer: &mut W, value: i32) -> io::Result<()> {
    let bytes = value.to_be_bytes();
    writer.write_all(&bytes).await
}

pub async fn write_i64_be<W: AsyncWrite + Unpin>(writer: &mut W, value: i64) -> io::Result<()> {
    let bytes = value.to_be_bytes();
    writer.write_all(&bytes).await
}

pub async fn read_varint<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<i32> {
    let mut num_read = 0;
    let mut result = 0i32;
    loop {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).await?;
        let byte = buf[0];
        let value = (byte & 0x7F) as i32;
        result |= value << (7 * num_read);
        num_read += 1;
        if num_read > 5 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt is too big"));
        }
        if (byte & 0x80) == 0 {
            break;
        }
    }
    Ok(result)
}

pub async fn write_varint<W: AsyncWrite + Unpin>(writer: &mut W, mut value: i32) -> io::Result<()> {
    loop {
        let mut temp = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0x80;
        }
        writer.write_all(&[temp]).await?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}
pub async fn read_varstring<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<String> {
    let len = read_varint(reader).await?;
    if len < 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "String length < 0"));
    }
    let len = len as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    match String::from_utf8(buf) {
        Ok(s) => Ok(s),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string")),
    }
}

pub async fn write_varstring<W: AsyncWrite + Unpin>(writer: &mut W, s: &str) -> io::Result<()> {
    write_varint(writer, s.len() as i32).await?;
    writer.write_all(s.as_bytes()).await?;
    Ok(())
}

pub async fn read_f32_be<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<f32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf).await?;
    Ok(f32::from_be_bytes(buf))
}

pub async fn write_f32_be<W: AsyncWrite + Unpin>(writer: &mut W, value: f32) -> io::Result<()> {
    let bytes = value.to_be_bytes();
    writer.write_all(&bytes).await
}

pub async fn read_f64_be<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<f64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf).await?;
    Ok(f64::from_be_bytes(buf))
}

pub async fn write_f64_be<W: AsyncWrite + Unpin>(writer: &mut W, value: f64) -> io::Result<()> {
    let bytes = value.to_be_bytes();
    writer.write_all(&bytes).await
}

pub async fn read_bytearray<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<Vec<u8>> {
    let len = read_varint(reader).await?;
    if len < 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Negative array length"));
    }
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf).await?;
    Ok(buf)
}

pub async fn write_bytearray<W: AsyncWrite + Unpin>(writer: &mut W, data: &[u8]) -> io::Result<()> {
    write_varint(writer, data.len() as i32).await?;
    writer.write_all(data).await?;
    Ok(())
}

pub async fn read_uuid<R: AsyncRead + Unpin + Send>(reader: &mut R) -> io::Result<Uuid> {
    let most_sig = Long::read_field(reader).await?.0 as u64;
    let least_sig = Long::read_field(reader).await?.0 as u64;
    Ok(Uuid::from_u64_pair(most_sig, least_sig))
}

pub async fn write_uuid<W: AsyncWrite + Unpin + Send>(writer: &mut W, data: &Uuid) -> io::Result<()> {
    let (most_sig, least_sig) = data.as_u64_pair();
    Long(most_sig as i64 ^ (1 << 63)).write_field(writer).await?;
    Long(least_sig as i64 ^ (1 << 63)).write_field(writer).await?;
    Ok(())
}