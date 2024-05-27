/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::net::SocketAddr;

use super::GatewayServerInterface;


pub(crate) struct WebsocketServer {
    pub(crate) addr: SocketAddr,
}
impl GatewayServerInterface for WebsocketServer {
    fn start(&self, gateway: &mut std::sync::Arc<crate::RockGateway>) -> crate::error::RResult<()> {
        
        Ok(())
    }

    fn name(&self) -> crate::error::RResult<String> {
        Ok(crate::util::r#const::WEBSOCKET.to_owned())
    }
}