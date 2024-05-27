/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub mod cache_builder;
pub mod tokio_settings_builder;
pub mod interfaces_builder;
use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    instance::service::{AddressInterface, CacheType, Service, TokioSettings, TokioType}, 
    modules::ratelimiter::RatelimiterType, 
    util::time_unit::TimeUnit, 
    RockGateway,
    error::GatewayError,
};

use self::{
    cache_builder::CacheBuilder, 
    interfaces_builder::InterfaceBuilder, 
    tokio_settings_builder::TokioSettingsBuilder
};

use super::{ratelimiter_builder::{RatelimiterBuilder, RatelimiterBuilderType}, Builder};


#[derive(Debug, Deserialize)]
pub(crate) struct ServiceBuilder {
    pub(crate) external_ip: Option<String>,
    pub(crate) disable_upgrade_insecure_requests: Option<bool>,
    pub(crate) ratelimiter: Option<RatelimiterBuilder>,
    pub(crate) interfaces: HashMap<String, Vec<InterfaceBuilder>>,
    pub(crate) multi_thread: Option<TokioSettingsBuilder>,
    pub(crate) current_thread: Option<TokioSettingsBuilder>,
    pub(crate) cache: Option<CacheBuilder>,
}

impl Builder<Service> for ServiceBuilder {
    fn build(&self, _: std::sync::Arc<RockGateway>) -> crate::error::RResult<Service> {
        let mut interfaces_instance = Vec::new();
        for (protocol, interfaces) in &self.interfaces {
            let protocol_clone = protocol.clone();
            let mut address_s = interfaces.iter().map(|interface_builder| {
                interface_builder.make_address_interface(&protocol_clone)
            }).collect::<crate::error::RResult<Vec<AddressInterface>>>()?;
            interfaces_instance.append(&mut address_s);
        }
        let tokio_type_instance = if let Some(ref tokio_settings_builder) = self.multi_thread {
            let tokio_settings = tokio_settings_builder.clone_to_tokio_settings()?;
            TokioType::MultiThread(tokio_settings)
        } else if let Some(ref tokio_settings_builder) = self.current_thread {
            let tokio_settings = tokio_settings_builder.clone_to_tokio_settings()?;
            TokioType::CurrentThread(tokio_settings)
        } else {
            TokioType::None
        };
        let caches = if let Some(cache) = &self.cache {
            let mut caches = Vec::new();
            if let Some(ref memory) = cache.memory {
                caches.push(CacheType::Memory { 
                    clear_time_interval: TimeUnit::parse(memory.clear_time_interval.clone()),
                });
            }
            if let Some(ref redis) = cache.redis {
                caches.push(CacheType::Redis {
                    ip: redis.ip.clone(), 
                    port: redis.port.clone(),
                    pwd: redis.pwd.clone(), 
                });
            }
            caches
        } else {
            let mut caches = Vec::new();
            caches.push(CacheType::Memory { 
                clear_time_interval: TimeUnit::parse("60min".to_string()),
            });
            caches
        };
        let disable_upgrade_insecure_requests = if let Some(duir) = self.disable_upgrade_insecure_requests {
            duir.clone()
        } else {
            true
        };
        let ratelimiter = if let Some(ratelimiter_builder) = &self.ratelimiter {
            Some(match ratelimiter_builder.r#type {
                RatelimiterBuilderType::Service => {
                    let period = TimeUnit::parse(ratelimiter_builder.period.clone());
                    super::ratelimiter_builder::structure_ratelimiter(period, ratelimiter_builder.burst.clone(), RatelimiterType::Service)
                }
                RatelimiterBuilderType::Ip => {
                    let period = TimeUnit::parse(ratelimiter_builder.period.clone());
                    super::ratelimiter_builder::structure_ratelimiter(period, ratelimiter_builder.burst.clone(), RatelimiterType::IpService)
                }
                _ => {
                    return Err(gateway_err!(ConfigurationFailed, "Config service.ratelimiter failed", crate::error::ConfigError::new(crate::error::ConfigErrorKind::RATELIMITER)));
                }
            })
        } else {
            None
        };
        Ok(Service {
            external_ip: self.external_ip.clone(),
            disable_upgrade_insecure_requests,
            interfaces: interfaces_instance,
            tokio_type: tokio_type_instance,
            cache: caches,
            ratelimiter,
        })
    }
}