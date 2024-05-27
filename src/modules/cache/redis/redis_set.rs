/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::Arc;

use crate::{
    context::ContextType, 
    error::RResult, 
    modules::{
        cache::{http_cache_cell::{
            HttpCacheKey, 
            HttpCacheShared
        }, CacheProfile}, ModuleType, PipeData, PipeModule
    }
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RedisCacheSet {}

impl PipeModule for RedisCacheSet {
    fn name(&self) -> ModuleType {
        ModuleType::RedisSet
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::RedisCacheSetData { profile } = pipe_data {
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                let cell = HttpCacheShared::memory_cache_value(
                    http_context.response_context.status, 
                    http_context.response_context.version, 
                    http_context.response_context.headers.clone(), 
                    http_context.response_context.body.clone()
                );
                let profile_read_lock = profile.read().await;
                if let Some(redis) = &profile_read_lock.redis {
                    let count = if profile_read_lock.hit == -1 { None } else { Some(profile_read_lock.hit) };
                    let expire = if profile_read_lock.expire.as_millis() == 0 { None } else { Some(profile_read_lock.expire.as_millis()) };
                    redis.set(
                        http_context.request_context.uri.to_string(), 
                        serde_json::to_string(&cell).unwrap(), 
                        expire, 
                        count,
                    ).await?;
                }
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}