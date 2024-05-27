/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::Arc;

use uuid::Uuid;

use crate::{common::ratelimiter::RatelimiterCommon, modules::{PipeLineEngine, PipeModule}};

#[derive(Debug)]
pub(crate) struct PipeLine {
    pub(crate) id: Uuid,
    pub(crate) pipe_name: String,
    pub(crate) pipe_line_engine: PipeLineEngine,
}
