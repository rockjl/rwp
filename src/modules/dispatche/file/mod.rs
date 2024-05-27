/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::path::PathBuf;

use futures_util::TryFutureExt;
use http::Version;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::common::http_file::HttpFile;
use crate::context::ContextType;
use crate::error::{PipeError, PipeErrorKind};
use crate::modules::{ModuleType, PipeData, PipeModule};
use crate::{
    context::GatewayContext,
    error::{GatewayError, RResult},
};

use super::DispatcheProfile;

#[derive(Debug, Clone, Copy)]
pub(crate) struct FileServerDispatche {}

impl PipeModule for FileServerDispatche {
    fn name(&self) -> ModuleType {
        ModuleType::DispatchFile
    }
    
    async fn execute(&self, mut ctx: GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<GatewayContext>  {
        if let PipeData::FileServerDispatcheData { profile } = pipe_data {
            let profile_read_lock = profile.read().await;
            if let DispatcheProfile::File { file_system, content_type, } = &*profile_read_lock {
                let base_path = file_system.base_path().to_string();
                match ctx.context_type {
                    ContextType::HttpContext(ref mut http_context) => {
                        let mut file_path = base_path + http_context.request_context.uri.path();
                        if file_path.ends_with("/") {
                            file_path = file_path[0..file_path.len()-1].to_string();
                        } 
                        let http_file = HttpFile::file_response_structure(
                            &file_path,
                            &file_system,
                            content_type,
                            &http_context.request_context.headers,
                        ).await?;
                        let (status_code, headers, bytes) = http_file.to_http_response().await;
                        http_context.response_context.status = status_code;
                        http_context.response_context.version = Version::HTTP_11;
                        http_context.response_context.headers = headers;
                        http_context.response_context.body = bytes;
                        http_context.response_context.refresh();
                    }
                    ContextType::TcpContext(ref mut tcp_context) => {
                        unreachable!()
                    }
                }
                return Ok(ctx);
            } else {
                return Err(gateway_err!(
                    PipeExecuteError,
                    "Pipe Execute Error Dispatche File Error",
                    PipeError::new(PipeErrorKind::DISPATCHE)
                ));
            }
        }
        unreachable!()
    }
}