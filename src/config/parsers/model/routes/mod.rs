/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub mod out_builder;
pub mod in_builder;
use serde::Deserialize;

use self::{in_builder::InBuilder, out_builder::OutBuilder};

use super::ratelimiter_builder::RatelimiterBuilder;

#[derive(Debug, Deserialize)]
pub(crate) struct RoutesBuilder {
    pub(crate) protocol: String,
    pub(crate) priority: Option<usize>,
    pub(crate) in_timeout: Option<String>,
    pub(crate) client_buf_size: Option<usize>,
    pub(crate) server_buf_size: Option<usize>,
    pub(crate) error: Option<String>,
    pub(crate) ratelimiter: Option<RatelimiterBuilder>,
    pub(crate) r#in: InBuilder,
    pub(crate) out: OutBuilder,
    pub(crate) pipe: String,
}