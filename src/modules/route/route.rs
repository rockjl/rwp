/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use hyper::http::Method;

use crate::{context::ContextType, error::{
    BuilderError, BuilderErrorKind, GatewayError, PipeError, PipeErrorKind, RResult
}, instance::routes::r#in::In, modules::{ModuleType, PipeData}};

use crate::modules::PipeModule;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RouteModule {}
impl RouteModule {
    fn match_method(&self, method: &Method, methods: &Option<Vec<String>>) -> bool {
        match methods {
            Some(ms) => {
                for m in ms {
                    if m.to_uppercase().eq(method.as_str()) {
                        return true;
                    }
                }
                false
            }
            None => {
                true
            }
        }
    }
}
impl PipeModule for RouteModule {
    fn name(&self) -> ModuleType {
        ModuleType::Route
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::RouteModuleData { profile } = pipe_data {
            match &mut ctx.context_type {
                ContextType::HttpContext(http_context) => {
                    let request_uri = http_context.request_context.uri.clone();
                    let path = request_uri.path();
                    let request_method = &http_context.request_context.method;
                    for (index, protocol, route_name, r#in) in &ctx.gateway.get_instance()?.ins {
                        if http_context.request_context.scheme.matchs(protocol.as_str()) {     //first, match protocol
                            match r#in {
                                In::Regex { pattern, method } => {      
                                    if self.match_method(request_method, method)
                                        && pattern.is_match(path) {                         //second, match request method and match reqeust path
                                        ctx.route = Some(route_name.clone());
                                        return Ok(ctx);
                                    }
                                }
                                In::Ip { ranges, method } => {
                                    let ip_str = ctx.remote_addr.ip().to_string();
                                    if self.match_method(request_method, method)
                                        &&ranges.check_whitelist(&ip_str) {
                                        ctx.route = Some(route_name.clone());
                                            return Ok(ctx);
                                    }
                                }
                                In::IpFile { ranges, method } => {
                                    let ip_str = ctx.remote_addr.ip().to_string();
                                    if self.match_method(request_method, method)
                                        && ranges.check_whitelist(&ip_str) {
                                        ctx.route = Some(route_name.clone());
                                            return Ok(ctx);
                                    }
                                }
                            };
                        }
                    }
                    return Err(gateway_err!(NotFoundRouteError, format!("NotFoundRouteError not match http route>uri:{:#?}, scheme:{:#?}, method:{:#?}", request_uri, http_context.request_context.scheme.as_str(), request_method.to_string()).as_str(), PipeError::new(PipeErrorKind::NOTFOUNDROUTE)));
                }
                ContextType::TcpContext(_) => {
                    for (route_name, route) in &ctx.gateway.get_instance()?.routes {
                        if route.protocol.as_str().eq_ignore_ascii_case(crate::util::r#const::TCP) {
                            match &route.r#in {
                                In::Regex { .. } => {
                                    return Err(gateway_err!(PipeExecuteError, "PipeExecuteError Regex not support Tcp", PipeError::new(PipeErrorKind::ROUTE)));
                                }
                                In::Ip { ranges, method } => {
                                    let ip_str = ctx.remote_addr.ip().to_string();
                                    if ranges.check_whitelist(&ip_str) {
                                        ctx.route = Some(route_name.clone());
                                        return Ok(ctx);
                                    }
                                }
                                In::IpFile { ranges, method} => {
                                    let ip_str = ctx.remote_addr.ip().to_string();
                                    if ranges.check_whitelist(&ip_str) {
                                        ctx.route = Some(route_name.clone());
                                        return Ok(ctx);
                                    }
                                }
                            }
                        }
                    }
                    return Err(gateway_err!(NotFoundRouteError, format!("NotFoundRouteError not match tcp route>remote_addr:{:#?}, ", ctx.remote_addr).as_str(), PipeError::new(PipeErrorKind::NOTFOUNDROUTE)));
                }
            }
        }
        unreachable!()
    }
}