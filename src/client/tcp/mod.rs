/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub(crate) mod sender;
use std::sync::Arc;

use crate::{entitys::buf::DataBuf, error::{AsyncResult, GatewayError, PipeError, PipeErrorKind, RResult}, gateway_err, util::j_unsafecell::JUnsafeCell};

use self::sender::TcpSenderConnection;

use super::{ClientHandlerInterface, RequestContent, ResponseContent};

#[derive(Debug, Clone)]
pub(crate) struct TcpClient {
    pub(crate) redirect: Arc<TcpSenderConnection>,
}
impl TcpClient {
    pub(crate) fn send(&self, request: RequestContent, timeout: std::time::Duration) -> AsyncResult<RResult<ResponseContent>> {
        let async_block = async move {
            let redirect = self.redirect.clone();
            if let RequestContent::Tcp((
                addr, 
                data_buf, 
                sender, 
                in_tx, 
                in_rx)) = request {
                if !redirect.is_connected().await {
                    redirect.connect(addr).await?;
                    let read_loop = self.redirect.clone();
                    tokio::task::spawn(async move {
                        match read_loop.start_read_loop(sender, in_rx, timeout).await {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("TcpSender read_loop Error:{:#?}", e);
                            }
                        }
                    });
                }
                match in_tx.send(data_buf).await {
                    Ok(_) => {
                        return Ok::<ResponseContent, GatewayError>(ResponseContent::Tcp(DataBuf::default()));
                    }
                    Err(e) => {
                        return Err(gateway_err!(PipeExecuteError, "Execute Pipe Error: in_tx.send called failed ERROR!", PipeError::new(PipeErrorKind::DISPATCHE)));
                    }
                }
            }
            Err(gateway_err!(PipeExecuteError, "Execute Pipe Error: RequestContent type ERROR!", PipeError::new(PipeErrorKind::DISPATCHE)))
        };
        Box::pin(async_block)
    }
}