/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::{context::ContextType, error::RResult, modules::{ModuleType, PipeData, PipeModule}};

use super::{HeaderProfile, HeaderActionKey};

#[derive(Debug, Clone, Copy)]
pub(crate) struct HeaderResponse {}
impl PipeModule for HeaderResponse {
    fn name(&self) -> ModuleType {
        ModuleType::HeaderResponse
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::HeaderResponseData { profile } = pipe_data {
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                let profile_read_lock = profile.read().await;
                if let Some(value) = profile_read_lock.action.get(&HeaderActionKey::DEL) {
                    for (header_name, header_value) in value {
                        http_context.request_context.headers.insert(header_name, header_value.clone());
                    }
                }
                if let Some(value) = profile_read_lock.action.get(&HeaderActionKey::ADD) {
                    for (header_name, header_value) in value {
                        http_context.request_context.headers.insert(header_name, header_value.clone());
                    }
                }
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}