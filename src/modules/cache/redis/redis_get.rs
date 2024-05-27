/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::Arc;

use crate::{
    context::ContextType, 
    error::{GatewayError, PipeError, PipeErrorKind, RResult}, 
    modules::{
        cache::{http_cache_cell::{
            HttpCacheCell, HttpCacheShared
        }, CacheProfile}, ModuleType, PipeData, PipeModule
    }
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RedisCacheGet {}
impl PipeModule for RedisCacheGet {
    fn name(&self) -> ModuleType {
        ModuleType::RedisGet
    }
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::RedisCacheGetData { profile } = pipe_data {
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                let profile_read_lock = profile.read().await;
                let is_count = if profile_read_lock.hit == -1 { false } else { true };
                let expire = if profile_read_lock.expire.as_millis() == 0 { None } else { Some(profile_read_lock.expire.as_millis()) };
                let cell = if let Some(redis) = &profile_read_lock.redis {
                    let key = http_context.request_context.uri.to_string();
                    let cell = redis.get(key.clone(), is_count).await?;
                    if &cell.0 == "" {
                        None
                    } else {
                        if is_count {
                            if cell.1 >= profile_read_lock.hit {
                                redis.del(http_context.request_context.uri.to_string(), is_count).await?;
                            } else {
                                redis.update_count(
                                    http_context.request_context.uri.to_string(), 
                                    expire,
                                    cell.1 + 1).await?;
                            }
                        }
                        redis.update_expire(key, expire).await?;
                        // let start = std::time::Instant::now();
                        let ret = serde_json::from_str::<HttpCacheCell>(&cell.0)?;
                        // println!("redis_get::convert_to_obj:{:#?}ms", start.elapsed().as_millis());
                        Some(ret)
                    }
                } else { return Err(gateway_err!(PipeExecuteError,"Pipe Execute Error Redis Get Error",PipeError::new(PipeErrorKind::REDIS))); };
                if let Some(value) = cell {
                    println!("success from redis get cache");
                    let (header, version, status, bytes) = value.to_origin();
                    http_context.response_context.status = status;
                    http_context.response_context.version = version;
                    http_context.response_context.headers = header;
                    http_context.response_context.body = bytes;
                    http_context.response_context.refresh();
                    http_context.cache_hit = true;
                }
                if http_context.cache_hit && profile_read_lock.back {
                    ctx.prompt_return = true;
                }
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}