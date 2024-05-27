/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use bytes::Bytes;
use http::{header, HeaderMap, StatusCode, Version};
use package_info::PackageInfo;

use crate::{
    context::ContextType, error::{
        GatewayError, PipeError, PipeErrorKind, RResult
    }, modules::{
        ModuleType, PipeData, PipeModule
    }, util::gateway_info::CargoPackageInfo
};

use super::{RatelimiterProfile, RatelimiterType};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RatelimiterModule {}
impl PipeModule for RatelimiterModule {
    fn name(&self) -> ModuleType {
        ModuleType::RateLimiter
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::RatelimiterModuleData { profile } = pipe_data {
            match ctx.context_type {
                ContextType::HttpContext(ref mut http_context) => {
                    let profile_read_lock = profile.read().await;
                    let check_ret = match &profile_read_lock.ratelimiter.ratelimiter_type {
                        RatelimiterType::IpRoute => {
                            profile_read_lock.ratelimiter.check_key(&http_context.request_context.remote_addr.ip().to_string())
                        }
                        RatelimiterType::IpService => {
                            profile_read_lock.ratelimiter.check_key(&http_context.request_context.remote_addr.ip().to_string())
                        }
                        RatelimiterType::Service => {
                            profile_read_lock.ratelimiter.check_key(&crate::util::r#const::ROCKJIANG_SERVICE.to_string())
                        }
                        RatelimiterType::Route(route_name) => {
                            profile_read_lock.ratelimiter.check_key(route_name)
                        }
                    };
                    match check_ret {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(gateway_err!(RatelimiterArrival, "ratelimiter arrival", PipeError::new(PipeErrorKind::RATELIMITER)));
                        }
                    }
                }
                ContextType::TcpContext(tcp_context) => {
                    unreachable!("tcp not support ratelimiter");
                }
            }
            
            return Ok(ctx);
        }
        unreachable!()
    }
}