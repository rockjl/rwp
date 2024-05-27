/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{net::SocketAddr, sync::Arc};

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;
use hyper::body::Incoming;

use crate::error::RResult;

use crate::servers::{GatewayServerInterface, tokiort::TokioIo};


pub(crate) mod http_service;

pub(crate) struct HttpServer {
    pub(crate) addr: SocketAddr,
}
impl GatewayServerInterface for HttpServer {
    fn start(&self, gateway: &mut Arc<crate::RockGateway>) -> RResult<()> {
        let addr = self.addr;
        let gateway_clone = gateway.clone();
        let self_name = self.name().unwrap_or_else(|_|{"Http".to_string()});
        gateway.spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await.expect(format!("Error ip_bind failed:{:#?}", addr).as_str());
            log::info!("{}>accept:{:#?}", self_name, addr);
            loop {
                let (stream, remote_addr) = listener.accept().await.expect(format!("Error ip_accept failed:{:#?}", addr).as_str());
                let io = TokioIo::new(stream);
                let gateway_clone = gateway_clone.clone();
                let instance = gateway_clone.get_instance();
                tokio::task::spawn(async move {
                    /* Whenever the browser is closed or not operated for a long time, a result of conn will be returned here. */
                    let conn_ret = hyper::server::conn::http1::Builder::new()
                        .serve_connection(io, hyper::service::service_fn(|request| {
                            http_service::http1(remote_addr, request, gateway_clone.clone())
                        })).await;
                    match conn_ret {
                        Ok(()) => {}
                        Err(e) => {
                            println!("Error http serving connection: {:?}", e);
                        }
                    }
                });
            }
        })
    }
    fn name(&self) -> RResult<String> {
        Ok(crate::util::r#const::HTTP.to_owned())
    }
}