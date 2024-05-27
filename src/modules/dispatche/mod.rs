/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{collections::HashMap, sync::Arc};

use crate::{
    client::ClientProvider, common::{content_type::ContentTypeAndExtension, file_system::FileSystem}, error::{BuilderError, BuilderErrorKind, GatewayError, RResult}
};

use self::{file::FileServerDispatche, network::NetworkDispatche};

use super::{balance::Host, PipeTask};

pub(crate) mod network;
pub(crate) mod file;

#[derive(Debug)]
pub(crate) enum DispatcheProfile {
    File {
        file_system: FileSystem,
        content_type: ContentTypeAndExtension,
    },
    Network {
        path: Option<String>,
        out_host: String,
        client: ClientProvider,
    }
}

#[inline(always)]
async fn load_balance(mut ctx: crate::context::GatewayContext, hosts: &crate::instance::hosts::Hosts, pipe_task: &PipeTask) -> crate::error::RResult<crate::context::GatewayContext> {
    match pipe_task.types {
        super::ModuleType::IpMatchRoundRobinLB => {
            ctx = hosts.modules.IpMatchRoundRobinLB(ctx, &pipe_task.pipe_data).await?;   // execute load_balance
        }
        super::ModuleType::RandomLB => {
            ctx = hosts.modules.RandomLB(ctx, &pipe_task.pipe_data).await?;   // execute load_balance
        }
        super::ModuleType::RoundBorinLB => {
            ctx = hosts.modules.RoundBorinLB(ctx, &pipe_task.pipe_data).await?;   // execute load_balance
        }
        _ => {
            return Err(gateway_err!(PipeExecuteError, "Pipe Execute Error Dispatche Network Error > not found load balance.", crate::error::PipeError::new(crate::error::PipeErrorKind::DISPATCHE)));
        }
    }
    return Ok(ctx);
}

pub(crate) async fn host_point_exception_handing(
    hosts: &Arc<tokio::sync::RwLock<HashMap<u16, Host>>>, 
    permanent_failure: &Arc<tokio::sync::RwLock<HashMap<u16, Host>>>, 
    previous_host: Option<u16>,
    gateway_err: &GatewayError) -> RResult<Option<(Arc<String>, Arc<u16>, Option<std::time::Duration>, u16)>> {
    if let Some(ph) = previous_host {
        let mut map_write_lock = hosts.write().await;
        let host = map_write_lock.get_mut(&ph);
        if let Some(h) = host {
            if let Some(_) = h.max_fails {
                match h.max_fails_durn.check_key(&crate::util::r#const::MAX_FAILS_DURN_CHECK_KEY) {
                    Ok(_) => {
                        return Ok(Some((h.host.clone(), h.port.clone(), h.timeout, ph)));
                    }
                    Err(_) => {
                        drop(map_write_lock);
                        fail_out_event(hosts, permanent_failure, ph).await;
                    }
                }
            } else {
                drop(map_write_lock);
            }
        }
    }
    Ok(None)
}
pub(crate) async fn fail_out_event(
    hosts: &Arc<tokio::sync::RwLock<HashMap<u16, Host>>>, 
    permanent_failure: &Arc<tokio::sync::RwLock<HashMap<u16, Host>>>, 
    index: u16) {
    let mut hosts_write_lock = hosts.write().await;
    //first: clear the host;
    let host = match hosts_write_lock.remove(&index) {  
        Some(h) => { h }
        None => { return }
    };
    drop(hosts_write_lock);
    //second: final clear task
    let hosts_clone = hosts.clone();
    if let Some(fail_timeout) = host.fail_timeout {
        tokio::spawn(async move {
            tokio::time::sleep(fail_timeout).await;
            log::info!("fail_out_event running...{:#?}", host);
            let mut hosts_write_lock = hosts_clone.write().await;
            hosts_write_lock.insert(index, host);
            drop(hosts_write_lock);
        });
    } else {
        let mut permanent_failure_write_lock = permanent_failure.write().await;
        permanent_failure_write_lock.insert(index, host);
        drop(permanent_failure_write_lock);
    }
}