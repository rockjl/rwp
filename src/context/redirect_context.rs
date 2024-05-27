/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{collections::HashMap, sync::Arc};

use crate::{instance::errors::Errs, modules::balance::Host};

#[derive(Debug, Default, Clone)]
pub(crate) struct RedirectContext {
    pub(crate) host: Option<Arc<String>>,       //destination host
    pub(crate) port: Option<Arc<u16>>,          //destination port
    pub(crate) timeout: Option<std::time::Duration>, //request timeout for client use
    pub(crate) previous_host: Option<u16>,           //last selected host_index
    pub(crate) hosts: Option<Arc<tokio::sync::RwLock<HashMap<u16, Host>>>>,                 //current hosts collection
    pub(crate) permanent_failure: Option<Arc<tokio::sync::RwLock<HashMap<u16, Host>>>>,     //current permanent_failure hosts collection
    pub(crate) err: Option<Arc<Errs>>,               //selected error_handling_plan
}