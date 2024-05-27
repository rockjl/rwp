/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub mod errors;
pub(crate) mod routes;
pub(crate) mod hosts;
pub(crate) mod service;
pub(crate) mod pipes;
use std::{collections::HashMap, sync::Arc};

use self::{errors::Errs, hosts::Hosts, routes::{r#in::In, Route}, service::Service};

#[derive(Debug, Default)]
pub(crate) struct GatewayInstance {
    pub(crate) service: Service,
    pub(crate) hosts: HashMap<String, Hosts>,
    pub(crate) routes: HashMap<String, Route>,
    pub(crate) ins: Vec<(usize, String, String, In)>, // index, protocol, pipe_name, In
    pub(crate) errors: Arc<HashMap<String, Arc<Errs>>>,
    pub(crate) bind_cpu: Arc<std::sync::Mutex<BindCpu>>,
}
unsafe impl Send for GatewayInstance{}
unsafe impl Sync for GatewayInstance{}

#[derive(Debug)]
pub(crate) struct BindCpu {
    pub(crate) index_step: usize,
    pub(crate) core_ids: Vec<core_affinity::CoreId>,
}
impl Default for BindCpu {
    fn default() -> Self {
        let core_ids = core_affinity::get_core_ids().unwrap();
        Self {
            index_step: 0,
            core_ids
        }
    }
}