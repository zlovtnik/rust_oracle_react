use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use redis::RedisResult;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::debug;

pub struct CacheService {
    connection_manager: ConnectionManager,
}

impl CacheService {
    pub fn new(connection_manager: ConnectionManager) -> Self {
        Self { connection_manager }
    }

    pub async fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.connection_manager.clone();
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                debug!("Cache hit for key: {}", key);
                let deserialized: T = serde_json::from_str(&v).map_err(|_| {
                    redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Failed to deserialize value",
                    ))
                })?;
                Ok(Some(deserialized))
            }
            None => {
                debug!("Cache miss for key: {}", key);
                Ok(None)
            }
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> RedisResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value).map_err(|_| {
            redis::RedisError::from((redis::ErrorKind::TypeError, "Failed to serialize value"))
        })?;

        let mut conn = self.connection_manager.clone();
        let mut cmd = redis::cmd("SET");
        cmd.arg(key).arg(serialized);

        if let Some(ttl) = ttl {
            cmd.arg("EX").arg(ttl.as_secs());
        }

        let _: () = cmd.query_async(&mut conn).await?;
        debug!("Cached value for key: {}", key);
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.connection_manager.clone();
        if key.ends_with('*') {
            // Handle pattern deletion
            let keys: Vec<String> = redis::cmd("KEYS").arg(key).query_async(&mut conn).await?;
            if !keys.is_empty() {
                let keys_ref: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
                let _: () = redis::cmd("DEL")
                    .arg(keys_ref)
                    .query_async(&mut conn)
                    .await?;
                debug!("Deleted {} keys matching pattern: {}", keys.len(), key);
            }
        } else {
            let _: () = conn.del(key).await.map_err(|_| {
                redis::RedisError::from((redis::ErrorKind::TypeError, "Failed to delete key"))
            })?;
            debug!("Deleted key: {}", key);
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.connection_manager.clone();
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }
}
