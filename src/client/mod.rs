/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{cell::RefCell, rc::Rc, sync::Arc};

use http_body_util::{Full, BodyExt};
use hyper::{http::{Request, Response, HeaderMap, StatusCode, Version}, body::Bytes};
use hyper::body::Incoming;

use crate::{entitys::buf::DataBuf, gateway_err, util::j_unsafecell::JUnsafeCell};
use crate::error::{RResult, AsyncResult, BuilderErrorKind, BuilderError, GatewayError};

use self::{http::HttpClient, tcp::{sender::TcpSenderConnection, TcpClient}};
use self::https::HttpsClient;

pub mod https;
pub mod http;
pub mod tcp;

type HttpsRedirect = hyper_util::client::legacy::Client<hyper_tls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>, Full<Bytes>>;

#[derive(Debug, Clone)]
pub(crate) enum ClientProvider {
    Http {
        client_handler: HttpClient,
    },
    Https {
        client_handler: HttpsClient,
    },
    Tcp {
        client_handler: TcpClient,
    }
}
pub(crate) trait ClientHandlerInterface: std::fmt::Debug + Send + Sync + Clone {
    fn send(&self, request: RequestContent, timeout: std::time::Duration) -> AsyncResult<RResult<ResponseContent>>;
}
pub(crate) enum RequestContent {
    Http(Request<Full<Bytes>>),
    Tcp((
        std::net::SocketAddr, 
        DataBuf, 
        Arc<tokio::sync::mpsc::Sender<DataBuf>>,
        Arc<tokio::sync::mpsc::Sender<DataBuf>>,
        Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<DataBuf>>>,
    )),
}
pub(crate) enum ResponseContent {
    Http((StatusCode, Version, HeaderMap, Option<Bytes>)),
    Tcp(DataBuf)
}

impl ClientProvider {
    pub(crate) fn new(client_type: &str, buf_size: usize) -> RResult<Self> {
        match client_type {
            crate::util::r#const::HTTP => {
                Ok(Self::Http { client_handler: HttpClient{ redirect: ClientProvider::make_https_client() }, })
            }
            crate::util::r#const::HTTPS => {
                Ok(Self::Https { client_handler: HttpsClient{ redirect: ClientProvider::make_https_client() }, })
            }
            crate::util::r#const::TCP => {
                Ok(Self::Tcp { client_handler: TcpClient{ redirect: ClientProvider::make_tcp_client( buf_size), } })
            }
            _ => {
                return Err(gateway_err!(BuilderFailed, format!("Builder Client Error > not fount type:{:#?}", client_type).as_str(), BuilderError::new(BuilderErrorKind::CLIENT)));
            }
        }
    }
    pub(crate) fn send(&self, request: RequestContent, timeout: std::time::Duration) -> AsyncResult<RResult<ResponseContent>> {
        match self {
            ClientProvider::Http { client_handler } => {
                client_handler.send(request, timeout)
            }
            ClientProvider::Https { client_handler } => {
                client_handler.send(request, timeout)
            }
            ClientProvider::Tcp { client_handler } => {
                client_handler.send(request, timeout)
            }
        }
    }
    pub(crate) fn make_tcp_client(buf_size: usize) 
        -> Arc<TcpSenderConnection> {
        Arc::new(TcpSenderConnection::new(buf_size))
    }
    pub(crate) fn make_https_client() 
        -> hyper_util::client::legacy::Client<hyper_tls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>, Full<Bytes>> {
        let connection = hyper_tls::HttpsConnector::new();
        hyper_util::client::legacy::Client::builder(
            crate::servers::tokiort::TokioExecutor::new()
        ).build(connection)
    }
    pub(crate) async fn parse_response(response: Response<Incoming>) -> RResult<(StatusCode, Version, HeaderMap, Option<Bytes>)> {
        let (parts, mut incoming) = response.into_parts();
        let status = parts.status;
        let version = parts.version;
        let mut headers = parts.headers;
        let body_bytes;
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
        body_bytes = Some(Bytes::from(body_vec));
        Ok((status, version, headers, body_bytes))
    }
}