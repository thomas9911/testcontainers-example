use anyhow::Result;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};

pub struct Redis {
    client: redis::Client,
}

impl Redis {
    pub fn new(connection_string: &str) -> Result<Redis> {
        let client = redis::Client::open(connection_string)?;

        Ok(Redis { client })
    }

    pub async fn list_keys(&self) -> Result<Vec<String>> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let data: Vec<_> = con.keys("*").await?;
        Ok(data)
    }

    pub async fn put<'a, K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
    {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let _: () = con.set(key, value).await?;
        Ok(())
    }

    pub async fn get<'a, K, RV>(&self, key: K) -> Result<RV>
    where
        K: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let x: RV = con.get(key).await?;
        Ok(x)
    }

    pub async fn delete<'a, K, RV>(&self, key: K) -> Result<RV>
    where
        K: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let x: RV = con.del(key).await?;
        Ok(x)
    }
}
