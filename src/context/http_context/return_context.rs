/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use http_body_util::Full;
use hyper::{http::Response, body::Bytes};

#[derive(Debug, Default, Clone)]
pub(crate) struct ReturnContext {
    pub(crate) response: Option<Response<Full<Bytes>>>,
}