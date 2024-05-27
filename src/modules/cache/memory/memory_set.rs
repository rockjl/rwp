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
pub(crate) struct MemoryCacheSet {}

impl PipeModule for MemoryCacheSet {
    fn name(&self) -> ModuleType {
        ModuleType::MemorySet
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::MemoryCacheSetData { profile } = pipe_data {
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                let profile_read_lock = profile.read().await;
                let mut cache_lock = profile_read_lock.cache.lock();
                let expire = if profile_read_lock.expire == std::time::Duration::from_nanos(0) {
                    std::cell::RefCell::new(None)
                } else {
                    std::cell::RefCell::new(Some(profile_read_lock.expire))
                };
                let hit = if profile_read_lock.hit == -1 {
                    std::cell::RefCell::new(None)
                } else {
                    std::cell::RefCell::new(Some(profile_read_lock.hit))
                };
                let http_cache_key = HttpCacheKey {
                    uri: http_context.request_context.uri.clone(),
                    save_point: std::cell::RefCell::new(Some(std::time::Instant::now())),
                    expire,
                    hit,
                };
                cache_lock.insert(http_cache_key, HttpCacheShared::memory_cache_value(
                    http_context.response_context.status, 
                    http_context.response_context.version, 
                    http_context.response_context.headers.clone(), 
                    http_context.response_context.body.clone()
                ));
                drop(cache_lock);
                drop(profile_read_lock);
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}