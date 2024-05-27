/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::time::Duration;
use std::sync::Arc;
use serde::Deserialize;

use crate::common::ratelimiter::RatelimiterCommon;
use crate::modules::ratelimiter::RatelimiterType;


#[derive(Debug, Deserialize)]
pub(crate) struct RatelimiterBuilder {
    pub(crate) r#type: RatelimiterBuilderType,
    pub(crate) period: String,
    pub(crate) burst: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) enum RatelimiterBuilderType {
    Service,
    Ip,
    Route
}
pub(crate) fn structure_ratelimiter(period: Duration, burst: u32, ratelimiter_type: RatelimiterType) -> Arc<RatelimiterCommon> {
    let quota = governor::Quota::with_period(period)
        .unwrap()
        .allow_burst(std::num::NonZeroU32::new(burst).unwrap());
    let lim: governor::RateLimiter<
        String,
        dashmap::DashMap<String, governor::state::InMemoryState>,
        governor::clock::QuantaClock,
        governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>,
    > = governor::RateLimiter::keyed(quota)
        .with_middleware::<governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>();
    Arc::new(RatelimiterCommon::new(
        Some(lim),
        ratelimiter_type,
    ))
}