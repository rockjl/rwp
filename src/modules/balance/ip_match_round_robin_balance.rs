/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{context::{ContextType, GatewayContext}, error::{GatewayError, RResult}, modules::{balance::LoadBalanceProfile, ModuleType, PipeData, PipeModule}};



#[derive(Debug, Clone, Copy)]
pub(crate) struct IpRoundRobinBalancer {}
impl PipeModule for IpRoundRobinBalancer {
    fn name(&self) -> ModuleType {
        ModuleType::IpMatchRoundRobinLB
    }
    
    async fn execute(&self, mut ctx: GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<GatewayContext>  {
        if let PipeData::IpRoundRobinBalancerData { profile } = pipe_data {
            let ip = ctx.remote_addr.ip().to_string();
            let profile_read_lock = profile.read().await;
            if profile_read_lock.ip_matchs.contains_key(&ip) {
                let dest_host_index = profile_read_lock.ip_matchs.get(&ip).unwrap();
                let hosts_read_lock = profile_read_lock.hosts.read().await;
                let host = hosts_read_lock.get(&dest_host_index);
                if let Some(host) = host {
                    ctx.redirect_context.host = Some(host.host.clone());
                    ctx.redirect_context.port = Some(host.port.clone());
                    ctx.redirect_context.timeout = host.timeout;
                    drop(hosts_read_lock);
                    drop(profile_read_lock);
                    return Ok(ctx);
                } else { // Due to the possibility of removing host_point from hosts in the fail_out_event, it is necessary to add a check here.
                    drop(hosts_read_lock);
                    drop(profile_read_lock);
                    let mut profile_write_lock = profile.write().await;
                    profile_write_lock.ip_matchs.remove(&ip);
                    drop(profile_write_lock);
                }
            }
            let mut profile_write_lock = profile.write().await;
            let (host, port, timeout, prev_index, mut ctx) = profile_write_lock.take_host(ctx).await?;
            if port.as_ref() == &0 && prev_index == 0 { // If no available host is found, return directly.
                ctx.redirect_context.host = None;
                ctx.redirect_context.port = None;
                return Ok(ctx);
            }
            ctx.redirect_context.host = Some(host);
            ctx.redirect_context.port = Some(port);
            ctx.redirect_context.timeout = timeout;
            if let Some(ph) = &mut ctx.redirect_context.previous_host {
                *ph = prev_index;
            } else {
                ctx.redirect_context.previous_host = Some(prev_index);
                ctx.redirect_context.hosts = Some(profile_write_lock.take_hosts());
                ctx.redirect_context.permanent_failure = Some(profile_write_lock.take_permanent_failure());
            }
            let host_index = profile_write_lock.host_index;
            profile_write_lock.ip_matchs.insert(ip, host_index);
            drop(profile_write_lock);
            return Ok(ctx);
        }
        unreachable!()
    }
}