/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{borrow::BorrowMut, sync::Arc};

use crate::{
    context::ContextType, 
    error::{
        GatewayError, PipeError, RResult
    },
    modules::{
        cache::{http_cache_cell::{
            HttpCacheKey, 
            HttpCacheShared
        }, CacheProfile}, ModuleType, PipeData, PipeModule
    }
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct MemoryCacheGet {}
impl PipeModule for MemoryCacheGet {
    fn name(&self) -> ModuleType {
        ModuleType::MemoryGet
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::MemoryCacheGetData { profile } = pipe_data {
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                let profile_read_lock = profile.read().await;
                let mut cache_lock = profile_read_lock.cache.lock();
                let http_cache_key = HttpCacheKey::for_key(http_context.request_context.uri.clone());
                let mut after_remove = false;
                if let Some((key, value)) = cache_lock.get_key_value(&http_cache_key) {
                    if HttpCacheKey::is_expire(key) {
                        cache_lock.remove(&http_cache_key);
                        return Ok(ctx);
                    }
                    if profile_read_lock.expire != std::time::Duration::from_millis(0) {
                        *key.expire.borrow_mut() = Some(profile_read_lock.expire);
                        *key.save_point.borrow_mut() = Some(std::time::Instant::now());
                    }
                    if profile_read_lock.hit != -1 {
                        let hit_borrow = key.hit.borrow();
                        if let Some(hit) = *hit_borrow {
                            if hit >= profile_read_lock.hit {
                                after_remove = true;
                            } else {
                                drop(hit_borrow);
                                *key.hit.borrow_mut() = Some(hit + 1);
                            }
                        } else {
                            drop(hit_borrow);
                            *key.hit.borrow_mut() = Some(1);
                        }
                    }
                    http_context.response_context.status = value.clone_status();
                    http_context.response_context.version = value.clone_version();
                    http_context.response_context.headers = value.clone_headers();
                    http_context.response_context.body = value.clone_body();
                    http_context.response_context.refresh();
                    http_context.cache_hit = true;
                }
                if after_remove {
                    cache_lock.remove(&http_cache_key);
                }
                drop(cache_lock);
                if http_context.cache_hit && profile_read_lock.back {
                    ctx.prompt_return = true;
                }
                drop(profile_read_lock);
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}