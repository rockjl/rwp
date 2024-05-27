/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub(crate) mod tcp_context;
pub(crate) mod http_context;
pub(crate) mod scheme;
pub(crate) mod redirect_context;

use std::{net::SocketAddr, sync::Arc};

use hyper::http::Request;
use hyper::body::{Bytes, Incoming};
use http_body_util::BodyExt;

use crate::entitys::buf::DataBuf;
use crate::gateway_err;
use crate::{RockGateway, instance::GatewayInstance, error::{RResult, GatewayError}};

use self::redirect_context::RedirectContext;
use self::http_context::request_context::RequestContext;
use self::http_context::response_context::ResponseContext;
use self::http_context::return_context::ReturnContext;
use self::http_context::HttpContext;
use self::scheme::SchemeContext;
use self::tcp_context::TcpContext;

#[derive(Debug)]
pub(crate) struct GatewayContext {
    pub(crate) remote_addr: SocketAddr,
    pub(crate) prompt_return: bool, //Immediately return the identifier. Terminate all pipe_module rows in the current pipe_line and return directly.
    pub(crate) gateway: Arc<RockGateway>,
    pub(crate) route: Option<String>,    
    pub(crate) redirect_context: RedirectContext,
    pub(crate) context_type: ContextType,
}
// impl Default for GatewayContext {
//     fn default() -> Self {
//         Self {
//             remote_addr: "0.0.0.0:0".parse().unwrap(),
//             gateway: Arc::default(),
//             route: None,
//             redirect_context: RedirectContext::default(),
//             context_type: ContextType::default(),
//         }
//     }
// }
#[derive(Debug)]
pub(crate) enum ContextType {
    HttpContext(HttpContext),
    TcpContext(TcpContext),
}
impl Default for ContextType {
    fn default() -> Self {
        ContextType::HttpContext(HttpContext::default())
    }
}

impl GatewayContext {
    pub(crate) async fn new_tcp_context(
        remote_addr: SocketAddr,
        gateway: Arc<RockGateway>,
        out_tx: Arc<tokio::sync::mpsc::Sender<DataBuf>>,
        in_tx: Arc<tokio::sync::mpsc::Sender<DataBuf>>,
        in_rx: Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<DataBuf>>>,
    ) -> RResult<Self> {
        Ok(Self {
            remote_addr,
            prompt_return: false,
            gateway,
            route: None,
            redirect_context: RedirectContext {
                host: None,
                port: None,
                timeout: None,
                previous_host: None,
                hosts: None,
                permanent_failure: None,
                err: None,
            },
            context_type: ContextType::TcpContext(TcpContext { 
                sender: out_tx,
                in_tx: in_tx,
                in_rx: in_rx,
                in_data_buf: None,
            }),
        })
    }
    pub(crate) async fn new_http_context(
        remote_addr: SocketAddr,
        request: Request<Incoming>,
        gateway: Arc<RockGateway>,
        scheme_str: &str,
    ) -> RResult<Self> {
        let uri = request.uri().clone();
        let query = uri.query();
        let uri_str = uri.to_string();
        let parameters = if let Some(param) = query {
            Some(param.to_string())
        } else {
            None
        };
        let scheme = SchemeContext::parse(uri.scheme_str().unwrap_or_else(||scheme_str));
        let method = request.method().clone();
        let (parts, mut incoming) = request.into_parts();
        let mut headers = parts.headers;
        let version = parts.version;
        let extensions = parts.extensions;
        let mut body_vec = Vec::new();
        while let Some(chunk) = incoming.frame().await {
            match chunk {
                Ok(body) => {
                    if body.is_data() {
                        let mut bb_v = body.into_data().unwrap().to_vec();
                        body_vec.append(&mut bb_v);
                    } else if body.is_trailers() {
                        let body_trailers = body.into_trailers().unwrap();
                        for (key, value) in body_trailers {
                            if let Some(k) = key {
                                headers.insert(k, value);
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(gateway_err!(ParseRequestError, "Parse Request Incoming ERROR", e));
                }
            }
        }
        let body_bytes = if body_vec.len() > 0 {
            Some(Bytes::from(body_vec))
        } else {
            None
        };
        
        let request_context = RequestContext {
            remote_addr,
            uri,
            parameters,
            scheme,
            extensions,
            version,
            method,
            headers,
            body: body_bytes.unwrap_or_else(|| { Bytes::new() } ),
        };
        let redirect_context = RedirectContext {
            host: None,
            port: None,
            timeout: None,
            previous_host: None,
            hosts: None,
            permanent_failure: None,
            err: None,
        };
        let response_context = ResponseContext::default();
        let return_context = ReturnContext::default();
        Ok(Self {
            remote_addr,
            prompt_return: false,
            gateway,
            route: None,
            redirect_context,
            context_type: ContextType::HttpContext(HttpContext {
                request_context,
                response_context,
                return_context,
                cache_hit: false,
            })
        })
    }
    pub(crate) fn get_gateway_instance(&self) -> RResult<Arc<GatewayInstance>> {
        self.gateway.get_instance()
    }
}