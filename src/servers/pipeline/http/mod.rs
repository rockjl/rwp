/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{net::SocketAddr, sync::Arc};

use futures_util::TryFutureExt;
use http::{header, Request, Response };
use http_body_util::Full;
use hyper::body::{Incoming, Bytes};

use crate::{
    context::{ContextType, GatewayContext}, error::{ GatewayError, RResult }, modules::PipeData, RockGateway
};


pub(crate) async fn http_run(
    remote_addr: SocketAddr,
    request: Request<Incoming>,
    gateway: Arc<RockGateway>,
) -> RResult<Response<Full<Bytes>>> {
    let instance = gateway.get_instance()?;
    let common_module_lock = gateway.common_module.read().await;
    let common_module = common_module_lock.as_ref().unwrap();
    let pipe_fut = futures_util::future::ok::<GatewayContext, GatewayError>(
        GatewayContext::new_http_context(remote_addr, request, gateway.clone(), crate::util::r#const::HTTP).await?
    );
    let upgrade_pipe_task = common_module.upgrade_pipe_task.as_ref();
    let route_pipe_task = common_module.route_pipe_task.as_ref();
    let return_pipe_task = common_module.return_pipe_task.as_ref();
    let mut context = if instance.service.disable_upgrade_insecure_requests {
        pipe_fut.and_then(|ctx| {
            /* first execute route */
            gateway.execute_one_task(ctx, route_pipe_task)
        }).await?
    } else {
        pipe_fut.and_then(|ctx| {
            /* first execute upgrade */
            gateway.execute_one_task(ctx, upgrade_pipe_task)
        }).and_then(|ctx| {
            /* second execute route */
            gateway.execute_one_task(ctx, route_pipe_task)
        }).await?
    };
    drop(common_module_lock);

    match &context.route {
        Some(route_name) => {
            let route = instance.routes.get(route_name).unwrap();
            /* third execute pipe line */
            context = match route.pipe_line.pipe_line_engine.execute(context).await {
                Ok(ctx) => {
                    ctx
                }
                Err(e) => {
                    match &e {
                        GatewayError::RatelimiterArrival { .. } => {
                            log::error!("RatelimiterArrival>{:#?}", e);
                            return Ok(crate::common::four_and_four_page::page404());
                        }
                        GatewayError::NotFoundRouteError { .. } => {
                            log::error!("NotFoundRouteError>{:#?}", e);
                            return Ok(crate::common::four_and_four_page::page404());
                        }
                        GatewayError::BlackAndWhiteListError { .. } => {
                            log::error!("BlackAndWhiteListError>{:#?}", e);
                            return Ok(crate::common::four_and_four_page::page404());
                        }
                        _ => {}
                    }
                    log::error!("PIPE_LINE_ERROR>{:#?}", e);
                    return Ok(crate::common::four_and_four_page::page404());
                }
            };
            if let ContextType::HttpContext(http_context) = &context.context_type {
                if http_context.return_context.response.is_none() {
                    println!("Immediately return and reorganize the data");
                    // if executed at this point, it is possible that the context was immediately returned. So here, try to reorganize the data and return it.
                    context = gateway.modules.Return(context, &mut PipeData::ReturnModuleData { profile: tokio::sync::RwLock::new(()) }).await?;
                }
            }
            if let ContextType::HttpContext(mut http_context) = context.context_type {
                match http_context.return_context.response.take() {
                    Some(response) => {
                        return Ok(response);    //return back success
                    }
                    None => {
                        log::error!("PIPE_LINE_ERROR>not found return_context. <return> may not be configured.");
                        return Ok(crate::common::four_and_four_page::page404());
                    }
                }
            }
        }
        None => {
            log::error!("PIPE_LINE_ERROR>not found route. <routes> may not be configured.");
            return Ok(crate::common::four_and_four_page::page404());
        }
    }
    
    unreachable!("Spell ERROR")
}