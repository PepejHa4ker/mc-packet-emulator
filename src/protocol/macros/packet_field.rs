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
        #[derive(Debug, Clone)]
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

        impl $crate::protocol::fields::AsyncReadField for $name {
            fn read_field<'a, R>(r: &'a mut R) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<Self>> + Send + 'a>>
            where
                R: tokio::io::AsyncRead + Unpin + Send + 'a,
            {
                Box::pin(Self::read(r))
            }
        }

        impl $crate::protocol::fields::AsyncWriteField for $name {
            fn write_field<'a, W>(&'a self, w: &'a mut W) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>>
            where
                W: tokio::io::AsyncWrite + Unpin + Send + 'a,
            {
                Box::pin(self.write(w))
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

        impl $crate::protocol::fields::AsyncReadField for $name {
            fn read_field<'a, R>(
                r: &'a mut R
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<Self>> + Send + 'a>>
            where
                R: tokio::io::AsyncRead + std::marker::Unpin + Send + 'a,
            {
                Box::pin(async move {
                                        let size = <$crate::protocol::fields::VarInt>::read_field(r).await?.0 as usize;
                    let mut map = std::collections::HashMap::new();
                    for _ in 0..size {
                        let key_val = <$key as $crate::protocol::fields::AsyncReadField>::read_field(r).await?;
                        let value = <$value as $crate::protocol::fields::AsyncReadField>::read_field(r).await?;
                                                map.insert((*key_val).clone(), value);
                    }
                    Ok($name(map))
                })
            }
        }

        impl $crate::protocol::fields::AsyncWriteField for $name {
            fn write_field<'a, W>(
                &'a self,
                w: &'a mut W
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>>
            where
                W: tokio::io::AsyncWrite + std::marker::Unpin + Send + 'a,
            {
                Box::pin(async move {
                                        let size = self.0.len() as i32;
                    $crate::protocol::fields::VarInt(size).write_field(w).await?;
                    for (key, value) in &self.0 {
                                                                        let key_wrapper: $key = <$key>::from(key.clone());
                        key_wrapper.write_field(w).await?;
                        value.write_field(w).await?;
                    }
                    Ok(())
                })
            }
        }
    }
}
