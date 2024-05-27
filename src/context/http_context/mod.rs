/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{net::SocketAddr, sync::Arc};

use crate::RockGateway;

use self::{request_context::RequestContext, response_context::ResponseContext, return_context::ReturnContext};

pub(crate) mod request_context;
pub(crate) mod response_context;
pub(crate) mod return_context;

#[derive(Debug, Clone, Default)]
pub(crate) struct HttpContext {
    pub(crate) request_context: RequestContext,
    pub(crate) response_context: ResponseContext,
    pub(crate) return_context: ReturnContext,           
    pub(crate) cache_hit: bool,
}