/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{sync::Arc, net::SocketAddr};

use http_body_util::Full;
use hyper::{Request, body::Incoming};
use crate::{servers::HyperResult, RockGateway};

pub(crate) fn http1(remote_addr: SocketAddr, request: Request<Incoming>, gateway: Arc<RockGateway>) -> HyperResult {
    Box::pin(crate::servers::pipeline::http::http_run(remote_addr, request, gateway))
}