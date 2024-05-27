/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use http::{header, Uri};

use crate::{common::upgrade_insecure_requests_page, context::ContextType, error::RResult, instance::service::AddressInterface, modules::{ModuleType, PipeData, PipeModule}, util::uri_util};


#[derive(Debug, Clone, Copy)]
pub(crate) struct UpgradeInsecureRequests {}
impl PipeModule for UpgradeInsecureRequests {
    fn name(&self) -> ModuleType {
        ModuleType::UpgradeInsecureRequest
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::UpgradeInsecureRequestsData { profile } = pipe_data {
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                if http_context.request_context.headers.contains_key(header::UPGRADE_INSECURE_REQUESTS) {
                    let profile_read_lock = profile.read().await;
                    let header_value = http_context.request_context.headers.get(header::UPGRADE_INSECURE_REQUESTS).unwrap();
                    if let Ok(h_v) = header_value.to_str() {
                        if h_v == "1" {
                            let mut address_opt = None;
                            for interface in profile_read_lock.interfaces.iter() {
                                if let AddressInterface::Https { addr, cert, key } = interface {
                                    address_opt = Some(addr);
                                    break;
                                }
                            };
                            if let Some(addr) = address_opt {
                                /*
                                if external_ip is null do nothing in the current pipe_module.
                                 */
                                let host = match profile_read_lock.external_ip {
                                    Some(ref host) => {
                                        host.clone()
                                    },
                                    None => { return Ok(ctx); }
                                };
                                let port = addr.port();
                                let uri_path = http_context.request_context.uri.path();
                                let uri: Uri = uri_util::assemble_uri(crate::util::r#const::HTTPS, host, port, "".to_string(), uri_path, http_context.request_context.parameters.clone());
                                let response = upgrade_insecure_requests_page::upgrade_insecure_request(uri.clone());
                                http_context.return_context.response = Some(response);
                            } else {
                                log::error!("Gateway not found https support!");
                            }
                        }
                    }
                }
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}