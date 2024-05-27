/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{sync::Arc, net::SocketAddr};

use hyper::{Request, body::Incoming, };
use crate::{RockGateway, servers::HyperResult};

pub(crate) fn https(remote_addr: SocketAddr, request: Request<Incoming>, gateway: Arc<RockGateway>) -> HyperResult {
    Box::pin(crate::servers::pipeline::https::https_run(remote_addr, request, gateway))
}
