/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::fmt::Pointer;

use deadpool_redis::{Config, Runtime};

use crate::error::RResult;

pub(crate) struct Redis {
    ip: String,
    port: u16,
    pwd: String,
    prefix: String,
    pool: deadpool_redis::Pool
}
impl std::fmt::Debug for Redis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Redis")
        .field("ip", &self.ip)
        .field("port", &self.port)
        .field("pwd", &self.pwd)
        .field("prefix", &self.prefix)
        .field("pool", &"pool")
        .finish()
    }
}
impl Redis {
    /*
    pexpire unit ms
     */
    pub(crate) async fn set(&self, key: String, value: String, pexpire: Option<u128>, count: Option<i32>) -> RResult<()> {
        let mut conn = self.pool.get().await?;
        deadpool_redis::redis::cmd("SET")
            .arg(&[&key, &value]).query_async::<_,()>(&mut conn).await?;
        if let Some(p) = pexpire {
            deadpool_redis::redis::cmd("PEXPIRE")
                .arg(&[&key, p.to_string().as_str()])
                .query_async::<_, ()>(&mut conn)
                .await.unwrap();
        }
        if let Some(count) = count {
            let count_key = self.count_key(&key);
            deadpool_redis::redis::cmd("SET")
                .arg(&[&count_key, count.to_string().as_str()]).query_async::<_,()>(&mut conn).await?;
            if let Some(p) = pexpire {
                deadpool_redis::redis::cmd("PEXPIRE")
                    .arg(&[&count_key, p.to_string().as_str()])
                    .query_async::<_, ()>(&mut conn)
                    .await.unwrap();
            }
        }
        Ok(())
    }
    pub(crate) async fn get(&self, key: String, is_count: bool) -> RResult<(String, i32)> {
        let mut conn = self.pool.get().await?;
        let value: String = deadpool_redis::redis::cmd("GET")
            .arg(&[&key]).query_async(&mut conn).await.unwrap_or_default();
        let count = if is_count {
            let count_key = self.count_key(&key);
            let count: i32 = deadpool_redis::redis::cmd("GET")
                .arg(&[&count_key]).query_async(&mut conn).await.unwrap_or_default();
            count
        } else { 0 };
        
        Ok((value, count))
    }
    pub(crate) async fn del(&self, key: String, is_count: bool) -> RResult<()> {
        let mut conn = self.pool.get().await?;
        deadpool_redis::redis::cmd("DEL")
            .arg(&[&key]).query_async::<_, ()>(&mut conn).await.unwrap_or_default();
        if is_count {
            let count_key = self.count_key(&key);
            deadpool_redis::redis::cmd("DEL")
                .arg(&[&count_key]).query_async::<_, ()>(&mut conn).await.unwrap_or_default();
        }
        Ok(())
    }
    pub(crate) async fn update_count(&self, key: String, pexpire: Option<u128>, count: i32) -> RResult<()> {
        let mut conn = self.pool.get().await?;
        let count_key = self.count_key(&key);
        deadpool_redis::redis::cmd("SET")
            .arg(&[&count_key, count.to_string().as_str()]).query_async::<_,()>(&mut conn).await?;
        if let Some(p) = pexpire {
            deadpool_redis::redis::cmd("PEXPIRE")
                .arg(&[&count_key, p.to_string().as_str()])
                .query_async::<_, ()>(&mut conn)
                .await?;
            return Ok(());
        }
        let old_expire: u32 = deadpool_redis::redis::cmd("PTTL")
            .arg(&[&count_key, ])
            .query_async(&mut conn)
            .await?;
        if old_expire > 0 {
            deadpool_redis::redis::cmd("PEXPIRE")
                .arg(&[&count_key, old_expire.to_string().as_str()])
                .query_async::<_, ()>(&mut conn)
                .await?;
        }
        Ok(())
    }
    pub(crate) async fn update_expire(&self, key: String, pexpire: Option<u128>) -> RResult<()> {
        let mut conn = self.pool.get().await?;
        if let Some(p) = pexpire {
            deadpool_redis::redis::cmd("PEXPIRE")
                .arg(&[&key, p.to_string().as_str()])
                .query_async::<_, ()>(&mut conn)
                .await?;
            return Ok(());
        }
        Ok(())
    }
    pub(crate) fn new(ip: String, port: u16, pwd: String, prefix: String) -> RResult<Self> {
        let url = format!("redis://:{}@{}:{}", pwd, ip, port);
        let cfg = Config::from_url(url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self { ip, port, pwd, prefix, pool })
    }
    fn count_key(&self, key: &str) -> String {
        self.prefix.clone() + key
    }
}