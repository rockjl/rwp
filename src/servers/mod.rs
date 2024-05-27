/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub(crate) mod websocket;
pub(crate) mod pipeline;
pub(crate) mod http;
pub(crate) mod https;
pub(crate) mod tcp;
pub(crate) mod tokiort;

use std::pin::Pin;
use std::sync::Arc;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::Response;

use crate::error::RResult;
use crate::instance::service::AddressInterface;

pub(crate) type HyperResult = Pin<Box<dyn std::future::Future<Output=RResult<Response<Full<Bytes>>>> + Send>>;
// pub(crate) type HyperResult = RResult<Response<Full<Bytes>>>;
pub(crate) type TcpResult = Pin<Box<dyn std::future::Future<Output=RResult<()>> + Send>>;

pub(crate) enum GatewayServer {
    Tcp(tcp::TcpServer),
    Http(http::HttpServer),
    Https(https::HttpsServer),
}
pub(crate) trait GatewayServerInterface {
    fn start(&self, gateway: &mut Arc<crate::RockGateway>) -> RResult<()>;
    fn name(&self) -> RResult<String>;
}
impl GatewayServer {
    pub(crate) fn create(iface: &AddressInterface) -> RResult<GatewayServer> {
        Ok(match iface {
            AddressInterface::Http { addr } => {
                GatewayServer::Http(http::HttpServer {
                    addr: *addr
                })
            }
            AddressInterface::Https { addr, cert, key } => {
                GatewayServer::Https(https::HttpsServer {
                    addr: *addr,
                    cert: cert.to_owned(),
                    key: key.to_owned(),
                })
            }
            AddressInterface::Tcp { addr } => {
                GatewayServer::Tcp(tcp::TcpServer {
                    addr: *addr,
                })
            }
        })
    }
    pub(crate) fn start(&self, gateway: &mut Arc<crate::RockGateway>) -> RResult<()> {
        match self {
            GatewayServer::Http(http) => {
                http.start(gateway)?;
            }
            GatewayServer::Https(https) => {
                https.start(gateway)?;
            }
            GatewayServer::Tcp(tcp) => {
                tcp.start(gateway)?;
            }
        }
        Ok(())
    }
    pub(crate) fn name(&self) -> RResult<String> {
        Ok(match self {
            GatewayServer::Http(http) => { http.name()? }
            GatewayServer::Https(https) => { https.name()? }
            GatewayServer::Tcp(tcp) => { tcp.name()? }
        })
    }
}