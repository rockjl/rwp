/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::collections::HashMap;

use http::HeaderMap;

use crate::error::{GatewayError, BuilderError, BuilderErrorKind};

use self::{header_request::HeaderRequest, header_response::HeaderResponse};

pub(crate) mod header_response;
pub(crate) mod header_request;

#[derive(Debug, Default)]
pub(crate) struct HeaderProfile {
    pub(crate) action: HashMap<HeaderActionKey, HeaderMap>,
}
impl HeaderProfile {
    pub(crate) fn new(action: HashMap<HeaderActionKey, HeaderMap>) -> Self {
        Self { action }
    }
}
#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum HeaderActionKey {
    ADD,
    DEL,
}
impl Default for HeaderActionKey {
    fn default() -> Self {
        Self::ADD
    }
}