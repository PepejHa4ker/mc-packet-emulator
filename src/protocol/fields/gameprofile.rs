use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::packet_field;
use crate::protocol::fields::{VarInt, VarString};
use crate::protocol::fields::uuid::Uuid;


packet_field! {
    Property((VarString, VarString, Option<VarString>)) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let name = VarString::read(r).await?;
            let value = VarString::read(r).await?;
            let signature = if let Ok(sig) = VarString::read(r).await {
                Some(sig)
            } else {
                None
            };

            Ok(Property((name, value, signature)))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            self.0.0.write(w).await?;
            self.0.1.write(w).await?;
            if let Some(signature) = &self.0.2 {
                signature.write(w).await?;
            }

            Ok(())
        }
    }
}

packet_field! {
    GameProfile((Uuid, VarString, Vec<Property>)) {
        async fn read(r: &mut impl AsyncRead + Unpin) -> io::Result<Self> {
            let uuid_str = VarString::read(r).await?;
            let uuid = uuid::Uuid::parse_str(&uuid_str.0)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UUID"))?;
            let name = VarString::read(r).await?;

            let property_count = VarInt::read(r).await?.0 as usize;
            let mut properties = Vec::with_capacity(property_count);
            for _ in 0..property_count {
                let name = VarString::read(r).await?;
                let value = VarString::read(r).await?;
                let signature = if let Ok(sig) = VarString::read(r).await {
                    Some(sig)
                } else {
                    None
                };

                properties.push(Property { 0: (name, value, signature) });
            }

            Ok(GameProfile((Uuid(uuid), name, properties)))
        }

        async fn write(&self, w: &mut impl AsyncWrite + Unpin) -> io::Result<()> {
            VarString(self.0.0.to_string()).write(w).await?;
            self.0.1.write(w).await?;

            VarInt(self.0.2.len() as i32).write(w).await?;
            for property in &self.0.2 {
                property.0.0.write(w).await?;
                property.0.1.write(w).await?;
                if let Some(signature) = &property.0.2 {
                    signature.write(w).await?;
                }
            }

            Ok(())
        }
    }
}
