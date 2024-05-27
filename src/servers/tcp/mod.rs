/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub(crate) mod tcp_service;

use std::{sync::Arc, net::SocketAddr};

use crate::error::RResult;

use super::GatewayServerInterface;


pub(crate) struct TcpServer {
    pub(crate) addr: SocketAddr,
}
impl GatewayServerInterface for TcpServer {
    fn start(&self, gateway: &mut Arc<crate::RockGateway>) -> RResult<()> {
        let addr = self.addr;
        let self_name = self.name().unwrap_or_else(|_|{"tcp".to_string()});
        let gateway_clone = gateway.clone();
        gateway.spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            log::info!("{}>accept:{:#?}", self_name, addr);
            loop {
                let (stream, remote_addr) = listener.accept().await.unwrap();
                let gateway_clone = gateway_clone.clone();
                tokio::spawn(async move {
                    match tcp_service::tcp(remote_addr, stream, gateway_clone).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Tcp_Service ERROR:{:#?}", e);
                        }
                    }
                });
            }
        })
    }
    fn name(&self) -> RResult<String> {
        Ok(crate::util::r#const::TCP.to_owned())
    }
}