/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::{client::ClientProvider, error::{AsyncResult, PipeError, PipeErrorKind, RResult, GatewayError}, gateway_err};

use super::{ClientHandlerInterface, HttpsRedirect, RequestContent, ResponseContent};


#[derive(Debug, Clone)]
pub(crate) struct HttpsClient {
    pub(crate) redirect: HttpsRedirect,
}
impl HttpsClient {
    pub(crate) fn send(&self, request: RequestContent, timeout: std::time::Duration) -> AsyncResult<RResult<ResponseContent>> {
        let async_block = async move {
            if let RequestContent::Http(req) = request {
                // println!("HttpsClient:request:{:#?}", req);
                // println!("request:url:{:#?}", req.uri().to_string());
                match tokio::time::timeout(timeout, self.redirect.request(req)).await {
                    Ok(ret) => {
                        let response = ret?;
                        // println!("HttpsClient:response:{:#?}", response);
                        let response_tuple = ClientProvider::parse_response(response).await?;
                        // println!("response:body:{:?}", response_tuple.3.as_slice());
                        return Ok(ResponseContent::Http(response_tuple));
                    }
                    Err(e) => {
                        return Err(gateway_err!(PipeExecuteTimeoutError, format!("Execute Pipe HttpsSender timeout:{:#?}", e).as_str(), PipeError::new(PipeErrorKind::HTTPSENDERTIMEOUT)));
                    }
                };
            }
            Err(gateway_err!(PipeExecuteError, "Execute Pipe Error: RequestContent type ERROR!", PipeError::new(PipeErrorKind::DISPATCHE)))
        };
        Box::pin(async_block)
    }
}