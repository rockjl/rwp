/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{error::Error, net::SocketAddr};

use hyper::{http::Request, body::Bytes};
use http_body_util::Full;
use hyper::http::Uri;

use crate::{
    client::{ClientProvider, RequestContent, ResponseContent}, common::http_file::HttpFile, context::{tcp_context, ContextType}, error::{GatewayError, PipeError, PipeErrorKind, RResult}, instance::errors::{ErrModule, ErrTypes, ReturnTypes}, modules::{ModuleType, PipeData, PipeModule}, util::uri_util
};

use super::DispatcheProfile;


#[derive(Debug, Clone)]
pub(crate) struct NetworkDispatche {}

impl PipeModule for NetworkDispatche {
    fn name(&self) -> ModuleType {
        ModuleType::DispatchNetwork
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::NetworkDispatcheData { profile } = pipe_data {
            let profile_read_lock = profile.read().await;
            /* first: match whether it is network or file */
            if let DispatcheProfile::Network { path, out_host, client} = &*profile_read_lock {
                let gateway_instance = ctx.get_gateway_instance()?;
                /* second: check if the specified host has been found */
                match gateway_instance.hosts.get(out_host) {
                    Some(hosts) => {
                        /* If the error_handling_plan of hosts exists, choose the error_handling_plan of hosts; otherwise, choose the error_handling_plan at the route level. */
                        let error_handling_plan = if let Some(err) = &hosts.hosts_error {
                            err.clone()
                        } else {
                            let route = gateway_instance.routes.get(&ctx.route.clone().unwrap()).unwrap();
                            route.routes_error.clone()
                        };
                        loop {
                            /* third: start routing and selecting the target host through the specified host */
                            ctx = crate::modules::dispatche::load_balance(ctx, hosts, hosts.lb_task.as_ref()).await?;
                            if ctx.redirect_context.host.is_none() {// If no available hosts are found, start the error_handling_plan process.
                                if let ErrModule::HTTP(http_error_handling_plan) = &error_handling_plan.inner {
                                    match http_error_handling_plan.r#return {
                                        ReturnTypes::Origin => { return Err(gateway_err!(NoAvailableHostsError, "no available host", PipeError::new(PipeErrorKind::NOAVAILABLEHOSTS))); }
                                        ReturnTypes::Hsc(sc) => {
                                            match ctx.context_type {
                                                ContextType::HttpContext(ref mut http_context) => {
                                                    let (status, version, headers, body) = crate::common::four_and_four_page::response_page(sc);
                                                    http_context.response_context.status = status;
                                                    http_context.response_context.version = version;
                                                    http_context.response_context.headers = headers;
                                                    http_context.response_context.body = body;
                                                    http_context.response_context.refresh();
                                                    return Ok(ctx);
                                                }
                                                ContextType::TcpContext(_) => {
                                                    unreachable!()
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    return Err(gateway_err!(NoAvailableHostsError, "no available host", PipeError::new(PipeErrorKind::NOAVAILABLEHOSTS)));
                                }
                            }
                            let timeout = ctx.redirect_context.timeout.unwrap_or_else(|| crate::util::r#const::DEFAULT_OUT_TIMEOUT);
                            match ctx.context_type {
                                ContextType::HttpContext(ref mut http_context) => {
                                    match *&client {
                                        ClientProvider::Http { client_handler } => {
                                            let scheme = "http";
                                            let host = ctx.redirect_context.host.clone().expect("not found host");
                                            let host = (*host).clone();
                                            let port = ctx.redirect_context.port.clone().expect("not found port");
                                            let port = (*port).clone();
                                            let uri_path = http_context.request_context.uri.path();
                                            let uri: Uri = uri_util::assemble_uri(scheme, host.clone(), port, path.clone().unwrap_or("".to_string()), uri_path, http_context.request_context.parameters.clone());
                                            let extensions = http_context.request_context.extensions.clone();
                                            let version = http_context.request_context.version.clone();
                                            let method = http_context.request_context.method.clone();
                                            let mut headers = http_context.request_context.headers.clone();
                                            headers.insert(hyper::header::HOST, host.parse().unwrap());
                                            let body = http_context.request_context.body.clone();
                                            let mut request = Request::new(Full::new(body));
                                            *request.headers_mut() = headers;
                                            *request.uri_mut() = uri;
                                            *request.extensions_mut() = extensions;
                                            *request.version_mut() = version;
                                            *request.method_mut() = method;
                                            match client_handler.send(RequestContent::Http(request), timeout).await {
                                                Ok(ResponseContent::Http((status, version, headers, body_bytes))) => {
                                                    http_context.response_context.status = status;
                                                    http_context.response_context.version = version;
                                                    http_context.response_context.headers = headers;
                                                    if let Some(body_bytes) = body_bytes {
                                                        http_context.response_context.body = body_bytes;
                                                    } else {
                                                        http_context.response_context.body = Bytes::new();
                                                    }
                                                    http_context.response_context.refresh();
                                                }
                                                Ok(_) => {
                                                    unreachable!("http response error");
                                                }
                                                Err(e) => {
                                                    let hosts = ctx.redirect_context.hosts.clone().unwrap();
                                                    let permanent_failure = ctx.redirect_context.permanent_failure.clone().unwrap();
                                                    let ret: Option<(std::sync::Arc<String>, std::sync::Arc<u16>, Option<std::time::Duration>, u16)> 
                                                        = crate::modules::dispatche::host_point_exception_handing(
                                                            &hosts, 
                                                            &permanent_failure, 
                                                            ctx.redirect_context.previous_host,
                                                            &e,
                                                        ).await?;
                                                    if let ErrModule::HTTP(http_error_handling_plan) = &error_handling_plan.inner {
                                                        if http_error_handling_plan.pass_next {
                                                            match e {
                                                                GatewayError::PipeExecuteTimeoutError { .. } => {
                                                                    log::error!("PipeExecuteError timeout continue:{:#?}", e);
                                                                    continue;
                                                                }
                                                                _ => {
                                                                    log::error!("PipeExecuteError other continue:{:#?}", e);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    return Err(e);
                                                }
                                            }
                                            if let ErrModule::HTTP(http_error_handling_plan) = &error_handling_plan.inner {
                                                for et in &http_error_handling_plan.error_list {
                                                    match et {
                                                        ErrTypes::Hsc(status_code) => {
                                                            if status_code == &http_context.response_context.status {   // error detected
                                                                if http_error_handling_plan.pass_next {
                                                                    continue;
                                                                } else {
                                                                    match http_error_handling_plan.r#return {
                                                                        ReturnTypes::Origin => {}
                                                                        ReturnTypes::Hsc(sc) => {
                                                                            let (status, version, headers, body) = crate::common::four_and_four_page::response_page(sc);
                                                                            http_context.response_context.status = status;
                                                                            http_context.response_context.version = version;
                                                                            http_context.response_context.headers = headers;
                                                                            http_context.response_context.body = body;
                                                                            http_context.response_context.refresh();
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            return Ok(ctx);
                                        }
                                        ClientProvider::Https { client_handler } => {
                                            let scheme = "https";
                                            let host = ctx.redirect_context.host.clone().expect("not found host");
                                            let host = (*host).clone();
                                            let port = ctx.redirect_context.port.clone().expect("not found port");
                                            let port = (*port).clone();
                                            let uri_path = http_context.request_context.uri.path();
                                            let uri: Uri = uri_util::assemble_uri(scheme, host.clone(), port, path.clone().unwrap_or("".to_string()), uri_path, http_context.request_context.parameters.clone());
                                            let mut headers = http_context.request_context.headers.clone();
                                            headers.insert(hyper::header::HOST, host.parse().unwrap()); 
                                            let extensions = http_context.request_context.extensions.clone();
                                            let version = http_context.request_context.version.clone();
                                            let method = http_context.request_context.method.clone();
                                            let body = http_context.request_context.body.clone();
                                            let mut request = Request::new(Full::new(body));
                                            *request.headers_mut() = headers;
                                            *request.uri_mut() = uri;
                                            *request.extensions_mut() = extensions;
                                            *request.version_mut() = version;
                                            *request.method_mut() = method;
                                            match client_handler.send(RequestContent::Http(request), timeout).await {
                                                Ok(ResponseContent::Http((status, version, headers, body_bytes))) => {
                                                    http_context.response_context.status = status;
                                                    http_context.response_context.version = version;
                                                    http_context.response_context.headers = headers;
                                                    if let Some(body_bytes) = body_bytes {
                                                        http_context.response_context.body = body_bytes;
                                                    } else {
                                                        http_context.response_context.body = Bytes::new();
                                                    }
                                                    http_context.response_context.refresh();
                                                }
                                                Ok(_) => {
                                                    unreachable!("http response error");
                                                }
                                                Err(e) => {
                                                    let hosts = ctx.redirect_context.hosts.clone().unwrap();
                                                    let permanent_failure = ctx.redirect_context.permanent_failure.clone().unwrap();
                                                    let ret: Option<(std::sync::Arc<String>, std::sync::Arc<u16>, Option<std::time::Duration>, u16)> 
                                                        = crate::modules::dispatche::host_point_exception_handing(
                                                            &hosts, 
                                                            &permanent_failure, 
                                                            ctx.redirect_context.previous_host,
                                                            &e,
                                                        ).await?;
                                                    if let ErrModule::HTTP(http_error_handling_plan) = &error_handling_plan.inner {
                                                        if http_error_handling_plan.pass_next {
                                                            match e {
                                                                GatewayError::PipeExecuteTimeoutError { .. } => {
                                                                    log::error!("PipeExecuteError timeout continue:{:#?}", e);
                                                                    continue;
                                                                }
                                                                _ => {
                                                                    log::error!("PipeExecuteError other continue:{:#?}", e);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    return Err(e);
                                                }
                                            }
                                            if let ErrModule::HTTP(http_error_handling_plan) = &error_handling_plan.inner {
                                                for et in &http_error_handling_plan.error_list {
                                                    match et {
                                                        ErrTypes::Hsc(status_code) => {
                                                            if status_code == &http_context.response_context.status {   // error detected
                                                                if http_error_handling_plan.pass_next {
                                                                    continue;
                                                                } else {
                                                                    match http_error_handling_plan.r#return {
                                                                        ReturnTypes::Origin => {}
                                                                        ReturnTypes::Hsc(sc) => {
                                                                            let (status, version, headers, body) = crate::common::four_and_four_page::response_page(sc);
                                                                            http_context.response_context.status = status;
                                                                            http_context.response_context.version = version;
                                                                            http_context.response_context.headers = headers;
                                                                            http_context.response_context.body = body;
                                                                            http_context.response_context.refresh();
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            return Ok(ctx);
                                        }
                                        ClientProvider::Tcp { .. } => {
                                            unreachable!()
                                        }
                                    }
                                }
                                ContextType::TcpContext(ref mut tcp_context) => {
                                    match *&client {
                                        ClientProvider::Http { .. } => {
                                            unreachable!()
                                        }
                                        ClientProvider::Https { .. } => {
                                            unreachable!()
                                        }
                                        ClientProvider::Tcp { client_handler } => {
                                            let host = ctx.redirect_context.host.clone().expect("not found host");
                                            let host = (*host).clone();
                                            let port = ctx.redirect_context.port.clone().expect("not found port");
                                            let port = (*port).clone();
                                            let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
                                            let data_buf = match tcp_context.in_data_buf.take() {
                                                Some(d_b) => {
                                                    d_b
                                                }
                                                None => {
                                                    return Err(gateway_err!(PipeExecuteError, "Pipe Execute Error Dispatche Network Error > not receive Tcp DataBuf.", PipeError::new(PipeErrorKind::DISPATCHE)));
                                                }
                                            };
                                            let sender = tcp_context.sender.clone();
                                            let in_tx = tcp_context.in_tx.clone();
                                            let in_rx = tcp_context.in_rx.clone();
                                            let _ = client_handler.send(RequestContent::Tcp((addr, data_buf, sender, in_tx, in_rx)), timeout).await?;
                                            return Ok(ctx);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        return Err(gateway_err!(PipeExecuteError, "Pipe Execute Error Dispatche Network Error > not found load balance dest host.", PipeError::new(PipeErrorKind::DISPATCHE)));
                    }
                }
            } else {
                return Err(gateway_err!(PipeExecuteError, "Pipe Execute Error Dispatche Network Error", PipeError::new(PipeErrorKind::DISPATCHE)));
            }
        }
        unreachable!()
    }
}