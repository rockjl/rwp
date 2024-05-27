/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::{Arc, Mutex};

use crate::{context::{ContextType}, error::RResult, modules::{ModuleType, PipeData, PipeModule}};



#[derive(Debug, Clone, Copy)]
pub(crate) struct RoundRobinBalancer {}
impl PipeModule for RoundRobinBalancer {
    fn name(&self) -> ModuleType {
        ModuleType::RoundBorinLB
    }
    
    async fn execute(&self, ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::RoundRobinBalancerData { profile } = pipe_data {
            let mut profile_write_lock = profile.write().await;
            let (host, port, timeout, prev_index, mut ctx) = profile_write_lock.take_host(ctx).await?;
            if port.as_ref() == &0 && prev_index == 0 { // If no available host is found, return directly.
                ctx.redirect_context.host = None;
                ctx.redirect_context.port = None;
                return Ok(ctx);
            }
            if ctx.redirect_context.hosts.is_none() || ctx.redirect_context.permanent_failure.is_none() {
                ctx.redirect_context.hosts = Some(profile_write_lock.take_hosts());
                ctx.redirect_context.permanent_failure = Some(profile_write_lock.take_permanent_failure());
            }
            drop(profile_write_lock);
            ctx.redirect_context.host = Some(host);
            ctx.redirect_context.port = Some(port);
            ctx.redirect_context.timeout = timeout;
            if let Some(ph) = &mut ctx.redirect_context.previous_host {
                *ph = prev_index;
            } else {
                ctx.redirect_context.previous_host = Some(prev_index);
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}