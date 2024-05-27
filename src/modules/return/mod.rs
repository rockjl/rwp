/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use http_body_util::Full;
use hyper::http::Response;

use crate::{context::{http_context, ContextType}, error::{
    BuilderError, BuilderErrorKind, GatewayError, RResult
}};

use super::{ModuleType, PipeData, PipeModule};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ReturnModule {}

impl PipeModule for ReturnModule {
    fn name(&self) -> ModuleType {
        ModuleType::Return
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &super::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::ReturnModuleData { profile } = pipe_data {
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                let mut response = Response::new(Full::new(http_context.response_context.body.clone()));
                *response.headers_mut() = http_context.response_context.headers.clone();
                *response.status_mut() = http_context.response_context.status.clone();
                *response.version_mut() = http_context.response_context.version.clone();
                // println!("return::response:{:#?}", response);
                http_context.return_context.response = Some(response);
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}