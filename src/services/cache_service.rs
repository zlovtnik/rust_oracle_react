use redis::aio::ConnectionManager;
use redis::{RedisError, RedisResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info};

pub struct CacheService {
    manager: ConnectionManager,
}

impl CacheService {
    pub fn new(manager: ConnectionManager) -> Self {
        Self { manager }
    }

    pub async fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.manager.clone();
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        match value {
            Some(v) => {
                debug!("Cache hit for key: {}", key);
                serde_json::from_str(&v)
                    .map_err(|e| {
                        RedisError::from((redis::ErrorKind::TypeError, "Deserialization failed"))
                    })
                    .map(Some)
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
        let mut conn = self.manager.clone();
        let serialized = serde_json::to_string(value)
            .map_err(|_| RedisError::from((redis::ErrorKind::TypeError, "Serialization failed")))?;
        let mut cmd = redis::cmd("SET");
        cmd.arg(key).arg(serialized);
        if let Some(ttl) = ttl {
            cmd.arg("EX").arg(ttl.as_secs());
        }
        cmd.query_async(&mut conn).await?;
        debug!("Cached value for key: {}", key);
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.manager.clone();
        if key.ends_with('*') {
            // Handle pattern deletion
            let keys: Vec<String> = redis::cmd("KEYS").arg(key).query_async(&mut conn).await?;
            if !keys.is_empty() {
                let keys_ref: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
                redis::cmd("DEL")
                    .arg(keys_ref)
                    .query_async(&mut conn)
                    .await?;
                debug!("Deleted {} keys matching pattern: {}", keys.len(), key);
            }
        } else {
            redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
            debug!("Deleted key: {}", key);
        }
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.manager.clone();
        let exists: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
        Ok(exists)
    }
}
