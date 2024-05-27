/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::Arc;

use crate::modules::{Modules, PipeTask};

use super::errors::Errs;

#[derive(Debug)]
pub(crate) struct Hosts {
    pub(crate) hosts_error: Option<Arc<Errs>>,
    pub(crate) lb_task: Box<PipeTask>,
    pub(crate) modules: Modules,
}