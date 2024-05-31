/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{
    common::max_fails_durn::MaxFailsDurn, 
    context::GatewayContext, 
    error::{BuilderError, BuilderErrorKind, GatewayError, PipeError, PipeErrorKind, RResult}, 
    instance::errors::Errs
};

use super::Modules;

pub(crate) mod round_robin_balance;
pub(crate) mod random_balance;
pub(crate) mod ip_match_round_robin_balance;

#[derive(Debug)]
pub(crate) struct LoadBalanceProfile {
    pub(crate) hosts: Arc<tokio::sync::RwLock<HashMap<u16, Host>>>,
    pub(crate) permanent_failure: Arc<tokio::sync::RwLock<HashMap<u16, Host>>>,
    pub(crate) host_index: u16,
    pub(crate) ip_matchs: HashMap<String, u16>,
    pub(crate) hosts_error: Option<Arc<Errs>>,
}
impl LoadBalanceProfile {
    pub(crate) fn take_hosts(&self) -> Arc<tokio::sync::RwLock<HashMap<u16, Host>>> {
        self.hosts.clone()
    }
    pub(crate) fn take_permanent_failure(&self) -> Arc<tokio::sync::RwLock<HashMap<u16, Host>>> {
        self.permanent_failure.clone()
    }
    /*
    if weight is not set, the execution effect is equivalent to weight equal to 1.
     */
    pub(crate) async fn take_host(&mut self, ctx: GatewayContext) -> RResult<(Arc<String>, Arc<u16>, Option<std::time::Duration>, u16, GatewayContext)> {
        //if there were no errors in the previous execution, continue to execute downwards.
        let mut map_write_lock = self.hosts.write().await;
        if map_write_lock.len() == 0 {
            return Ok((Arc::new("".to_string()), Arc::new(0), Some(std::time::Duration::from_millis(0)), 0, ctx));
        }
        let index = self.host_index % (map_write_lock.len() as u16);
        let host = map_write_lock.get_mut(&index);
        if let Some(h) = host {
            if let Some(weight) = h.weight {
                /* 
                if weight is set, first increase the cur_weight of the current host.
                then determine the cur_weight%weight of the current host.if it is equal to 0,
                increase the host_index and return the current host.if it is not equal 0, 
                return the current host directly.
                 */
                h.cur_weight = h.cur_weight + 1;
                if h.cur_weight % weight == 0 {
                    self.host_index = self.host_index.wrapping_add(1);
                    return Ok((h.host.clone(), h.port.clone(), h.timeout, index, ctx));
                } else {
                    return Ok((h.host.clone(), h.port.clone(), h.timeout, index, ctx));
                }
            } else {    //if weight is not set, increase the host_index and then take the current value.
                self.host_index = self.host_index.wrapping_add(1);
                return Ok((h.host.clone(), h.port.clone(), h.timeout, index, ctx));
            }
        } else {
            return Err("dff".into());
        }
    }
}
#[derive(Debug)]
pub(crate) struct Host {
    pub(crate) host: Arc<String>,
    pub(crate) port: Arc<u16>,
    pub(crate) weight: Option<u32>,
    pub(crate) cur_weight: u32,
    pub(crate) max_fails: Option<u32>,
    pub(crate) fail_timeout: Option<std::time::Duration>,
    pub(crate) timeout: Option<std::time::Duration>,
    pub(crate) max_fails_durn: MaxFailsDurn,
}
