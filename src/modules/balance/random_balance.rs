/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::Arc;

use rand::Rng;

use crate::{context::ContextType, error::{PipeError, PipeErrorKind, RResult, GatewayError}, modules::{ModuleType, PipeData, PipeModule}};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RandomBalancer {}
impl PipeModule for RandomBalancer {
    fn name(&self) -> ModuleType {
        ModuleType::RandomLB
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::RandomBalancerData { profile } = pipe_data {
            let profile_read_lock = profile.read().await;
            let (host, port, timeout, prev_index) = {
                let hosts_read_lock = profile_read_lock.hosts.read().await;
                if hosts_read_lock.len() == 0 { // If no available host is found, return directly.
                    ctx.redirect_context.host = None;
                    ctx.redirect_context.port = None;
                    return Ok(ctx);
                }
                let index = rand::thread_rng().gen_range(0..hosts_read_lock.len()) as u16;
                let ret = &hosts_read_lock[&index];
                (ret.host.clone(), ret.port.clone(), ret.timeout, index)
            };
            if ctx.redirect_context.hosts.is_none() || ctx.redirect_context.permanent_failure.is_none() {
                ctx.redirect_context.hosts = Some(profile_read_lock.take_hosts());
                ctx.redirect_context.permanent_failure = Some(profile_read_lock.take_permanent_failure());
            }
            drop(profile_read_lock);
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