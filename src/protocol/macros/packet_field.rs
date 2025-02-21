#[macro_export]
macro_rules! packet_field {
    (
        $( #[$meta:meta] )*
        $name:ident ( $inner:ty ) {
            async fn read($r:ident : &mut $rty:ty) -> $ret:ty $read_block:block

            async fn write(&$selfvar:ident, $w:ident : &mut $wty:ty) -> $retw:ty $write_block:block
        }
    ) => {
        $( #[$meta] )*
        #[derive(Debug, Clone, PartialOrd, PartialEq)]
        pub struct $name(pub $inner);

        impl std::ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            pub async fn read($r: &mut $rty) -> $ret $read_block
            pub async fn write(&$selfvar, $w: &mut $wty) -> $retw $write_block
        }

        #[async_trait::async_trait]
        impl $crate::protocol::fields::AsyncReadField for $name {
            async fn read_field<R>(r: &mut R) -> std::io::Result<Self>
            where
                R: tokio::io::AsyncRead + Unpin + Send,
            {
                Self::read(r).await
            }
        }

        #[async_trait::async_trait]
        impl $crate::protocol::fields::AsyncWriteField for $name {
            async fn write_field<W>(&self, w: &mut W) -> std::io::Result<()>
            where
                W: tokio::io::AsyncWrite + Unpin + Send,
            {
                self.write(w).await
            }
        }
    }
}


#[macro_export]
macro_rules! map_field {
    ($name:ident, $key:ty, $value:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name(pub std::collections::HashMap<<$key as std::ops::Deref>::Target, $value>);

        impl $name {
            pub fn new() -> Self {
                $name(std::collections::HashMap::new())
            }
        }

        #[async_trait::async_trait]
        impl $crate::protocol::fields::AsyncReadField for $name {
            async fn read_field<R>(r: &mut R) -> std::io::Result<Self>
            where
                R: tokio::io::AsyncRead + std::marker::Unpin + Send,
            {
                let size = <$crate::protocol::fields::VarInt>::read_field(r).await?.0 as usize;
                let mut map = std::collections::HashMap::new();
                for _ in 0..size {
                    let key_val = <$key as $crate::protocol::fields::AsyncReadField>::read_field(r).await?;
                    let value = <$value as $crate::protocol::fields::AsyncReadField>::read_field(r).await?;
                    map.insert((*key_val).clone(), value);
                }
                Ok($name(map))
            }
        }

        #[async_trait::async_trait]
        impl $crate::protocol::fields::AsyncWriteField for $name {
            async fn write_field<W>(&self, w: &mut W) -> std::io::Result<()>
            where
                W: tokio::io::AsyncWrite + std::marker::Unpin + Send,
            {
                let size = self.0.len() as i32;
                $crate::protocol::fields::VarInt(size).write_field(w).await?;
                for (key, value) in &self.0 {
                    let key_wrapper: $key = <$key>::from(key.clone());
                    key_wrapper.write_field(w).await?;
                    value.write_field(w).await?;
                }
                Ok(())
            }
        }
    }
}
