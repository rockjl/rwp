/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{net::SocketAddr, time::Duration};

use crate::{common::{ratelimiter::RatelimiterCommon, redis::Redis}, error::{GatewayError, RResult}};



#[derive(Debug)]
pub(crate) struct Service {
    pub(crate) external_ip: Option<String>,
    pub(crate) disable_upgrade_insecure_requests: bool,
    pub(crate) interfaces: Vec<AddressInterface>,
    pub(crate) tokio_type: TokioType,
    pub(crate) cache: Vec<CacheType>,
    pub(crate) ratelimiter: 
        Option<std::sync::Arc<
                RatelimiterCommon
        >>,
}
impl Service {
    pub(crate) fn has_cache(&self, cache_type_str: &str) -> bool {
        for cache_type in &self.cache {
            match cache_type {
                CacheType::Memory { .. } if cache_type_str == "memory" => {
                    return true;
                },
                CacheType::Redis { .. } if cache_type_str == "redis" => {
                    return true;
                },
                _ => {}
            }
        }
        return false;
    }
    pub(crate) fn crate_redis(&self, prefix: String) -> RResult<Redis> {
        for cache_type in &self.cache {
            if let CacheType::Redis { ip, port, pwd } = cache_type {
                return Ok(Redis::new(ip.clone(), port.clone(), pwd.clone(), prefix)?);
            }
        }
        Err("ERROR: Not Found Redis Config!".into())
    }
}
#[derive(Debug)]
pub(crate) enum TokioType {
    CurrentThread(TokioSettings),
    MultiThread(TokioSettings),
    None,
}
#[derive(Debug)]
pub(crate) enum CacheType {
    Memory {
        clear_time_interval: Duration,
    },
    Redis {
        ip: String,
        port: u16,
        pwd: String,
    }
}
#[derive(Debug, Default)]
pub(crate) struct TokioSettings {
    pub(crate) event_interval: Option<u32>,
    pub(crate) thread_name: Option<String>,
    pub(crate) thread_stack_size: Option<usize>,
    pub(crate) global_queue_interval: Option<u32>,
    pub(crate) max_blocking_threads: Option<usize>,
    pub(crate) nevents: Option<usize>,
    pub(crate) core_threads: Option<usize>,
    pub(crate) bind_cpu: Option<TokioBindCpuType>,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum TokioBindCpuType {
    All,
    Half,
    Num(usize),
    Even,
    Odd,
}
#[derive(Debug, Clone)]
pub(crate) enum AddressInterface {
    Http {
        addr: SocketAddr,
    },
    Https {
        addr: SocketAddr,
        cert: String,
        key: String,
    },
    Tcp {
        addr: SocketAddr
    }
}
impl Default for AddressInterface {
    fn default() -> Self {
        AddressInterface::Http { addr: "0.0.0.0:8888".parse().unwrap() }
    }
}
impl Default for Service {
    fn default() -> Self {
        Service { 
            external_ip: None,
            disable_upgrade_insecure_requests: true,
            interfaces: Vec::new(), 
            tokio_type: TokioType::MultiThread(TokioSettings::default()) ,
            cache: Vec::new(),
            ratelimiter: None,
        }
    }
}