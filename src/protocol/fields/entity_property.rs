use crate::protocol::fields::uuid::Uuid;
use crate::protocol::fields::{AsyncReadField, AsyncWriteField, Byte, Double, VarInt, VarString};
use async_trait::async_trait;
use std::future::Future;
use std::io;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub struct EntityAttributeModifier {
    pub uuid: Uuid,
    pub amount: Double,
    pub operation: Byte,
}

#[derive(Debug, Clone)]
pub struct EntityProperty {
    pub key: VarString,
    pub value: Double,
    pub modifiers: Vec<EntityAttributeModifier>,
}

#[async_trait]
impl AsyncReadField for EntityProperty {
    fn read_field<'a, R>(
        reader: &'a mut R,
    ) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send + 'a>>
    where
        R: AsyncRead + Unpin + Send + 'a,
    {
        Box::pin(async move {
            let key = VarString::read_field(reader).await?;
            let value = Double::read_field(reader).await?;
            let modifier_count = VarInt::read_field(reader).await?.0 as usize;

            let mut modifiers = Vec::with_capacity(modifier_count);
            for _ in 0..modifier_count {
                let uuid = Uuid::read_field(reader).await?;
                let amount = Double::read_field(reader).await?;
                let operation = Byte::read_field(reader).await?;

                modifiers.push(EntityAttributeModifier {
                    uuid,
                    amount,
                    operation,
                });
            }

            Ok(EntityProperty {
                key,
                value,
                modifiers,
            })
        })
    }
}

#[async_trait]
impl AsyncWriteField for EntityProperty {
    fn write_field<'a, W>(
        &'a self,
        writer: &'a mut W,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>>
    where
        W: AsyncWrite + Unpin + Send + 'a,
    {
        Box::pin(async move {
            self.key.write_field(writer).await?;
            self.value.write_field(writer).await?;
            VarInt(self.modifiers.len() as i32)
                .write_field(writer)
                .await?;

            for modifier in &self.modifiers {
                modifier.uuid.write_field(writer).await?;
                modifier.amount.write_field(writer).await?;
                modifier.operation.write_field(writer).await?;
            }

            Ok(())
        })
    }
}
