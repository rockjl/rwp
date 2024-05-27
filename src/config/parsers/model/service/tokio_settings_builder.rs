/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use serde::Deserialize;

use crate::{error::{ ConfigError, ConfigErrorKind, GatewayError, RResult }, instance::service::{TokioBindCpuType, TokioSettings}};


#[derive(Debug, Deserialize)]
pub(crate) struct TokioSettingsBuilder {
    pub(crate) event_interval: Option<u32>,
    pub(crate) thread_name: Option<String>,
    pub(crate) thread_stack_size: Option<usize>,
    pub(crate) global_queue_interval: Option<u32>,
    pub(crate) max_blocking_threads: Option<usize>,
    pub(crate) nevents: Option<usize>,
    pub(crate) core_threads: Option<usize>,
    pub(crate) bind_cpu: Option<String>,
}

impl TokioSettingsBuilder {
    pub(crate) fn clone_to_tokio_settings(&self) -> RResult<TokioSettings> {
        let bind_cpu = match &self.bind_cpu {
            Some(t) => {
                Some(match t.as_str() {
                    "all" => { TokioBindCpuType::All }
                    "half" => { TokioBindCpuType::Half }
                    other => {
                        let num = match other.parse::<usize>() {
                            Ok(ret) => { ret }
                            Err(e) => {
                                return Err(gateway_err!(ConfigurationFailed, "tokio setting error bind_cpu", ConfigError::new(ConfigErrorKind::TOKIO))); 
                            }
                        };
                        TokioBindCpuType::Num(num)
                    }
                })
            }
            None => { None }
        };
        Ok(TokioSettings {
            event_interval: self.event_interval.clone(),
            thread_name: self.thread_name.clone(),
            thread_stack_size: self.thread_stack_size.clone(),
            global_queue_interval: self.global_queue_interval.clone(),
            max_blocking_threads: self.max_blocking_threads.clone(),
            nevents: self.nevents.clone(),
            core_threads: self.core_threads.clone(),
            bind_cpu,
        }) 
    }
}