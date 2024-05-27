/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::{common::ratelimiter::RatelimiterCommon, error::{BuilderError, BuilderErrorKind, GatewayError, RResult}};

use self::ratelimiter::RatelimiterModule;

pub(crate) mod ratelimiter;

#[derive(Debug)]
pub(crate) struct RatelimiterProfile {
    pub(crate) ratelimiter: std::sync::Arc<RatelimiterCommon>,
}
#[derive(Debug)]
pub(crate) enum RatelimiterType {
    Service,
    IpRoute,
    IpService,
    Route(String),
}
impl RatelimiterType {
    pub(crate) fn new(ratelimiter_type_str: &str, route_name: String, pipe_name: String) -> RResult<Self> {
        match ratelimiter_type_str {
            "ip_route" => { Ok(RatelimiterType::IpRoute) }
            "ip_service" => { Ok(RatelimiterType::IpService) }
            "service" => { Ok(RatelimiterType::Service) }
            "route" => { Ok(RatelimiterType::Route(route_name)) }
            _ => { return Err(gateway_err!(BuilderFailed, "Build ratelimiter error", BuilderError::new(BuilderErrorKind::HEADER))); }
        }
    }
}