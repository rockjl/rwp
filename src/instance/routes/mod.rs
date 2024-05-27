/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub mod out;
pub mod r#in;

use std::sync::Arc;
use crate::{common::ratelimiter::RatelimiterCommon, modules::cache::http_cache_cell::HttpCacheShared};
use self::{out::Out, r#in::In};
use super::{errors::Errs, pipes::PipeLine};

#[derive(Debug)]
pub(crate) struct Route {
    pub(crate) protocol: String,
    pub(crate) priority: usize,
    pub(crate) in_timeout: std::time::Duration,
    pub(crate) client_buf_size: usize,
    pub(crate) server_buf_size: usize,
    pub(crate) r#in: In,
    pub(crate) out: Out,
    pub(crate) pipe_line: PipeLine,
    pub(crate) memory_cache_shared: Arc<HttpCacheShared>,
    pub(crate) ratelimiter: 
        Option<std::sync::Arc<
                RatelimiterCommon
        >>,
    pub(crate) routes_error: Arc<Errs>,
}