/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;

use crate::{servers::{pipeline, TcpResult}, RockGateway};

pub(crate) fn tcp(remote_addr: SocketAddr, stream: TcpStream, gateway: Arc<RockGateway>) -> TcpResult {
    Box::pin(pipeline::tcp::tcp_run(remote_addr, stream, gateway))
}