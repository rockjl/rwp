/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub mod http_cache_cell;
pub(crate) mod redis;
pub(crate) mod memory;

use std::{sync::Arc, time::Duration};

use crate::{common::redis::Redis, error::{BuilderError, BuilderErrorKind, GatewayError}};

use self::{http_cache_cell::HttpCacheShared, memory::{memory_get::MemoryCacheGet, memory_set::MemoryCacheSet}, redis::{redis_get::RedisCacheGet, redis_set::RedisCacheSet}};

use super::Modules;

#[derive(Debug)]
pub(crate) struct CacheProfile {
    pub(crate) cache: Arc<HttpCacheShared>,
    pub(crate) expire: Duration,
    pub(crate) hit: i32,
    pub(crate) redis: Option<Redis>,
    pub(crate) back: bool,
}
impl CacheProfile {
    pub(crate) fn new(cache: Arc<HttpCacheShared>, expire: Duration, hit: i32, redis: Option<Redis>, back: bool) -> Self {
        Self { cache, expire, hit, redis, back }
    }
}