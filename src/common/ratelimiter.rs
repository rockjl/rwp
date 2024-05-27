/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use governor::{clock::QuantaInstant, NotUntil};

use crate::modules::ratelimiter::RatelimiterType;


#[derive(Debug)]
pub(crate) struct RatelimiterCommon {
    pub(crate) ratelimiter: 
        Option<
            governor::RateLimiter<String, dashmap::DashMap<String, governor::state::InMemoryState>, governor::clock::QuantaClock, governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>
        >
    ,
    pub(crate) ratelimiter_type: RatelimiterType,
}
impl Default for RatelimiterCommon {
    fn default() -> Self {
        Self {
            ratelimiter: None,
            ratelimiter_type: RatelimiterType::Service,
        }
    }
}
impl RatelimiterCommon {
    pub(crate) fn new(
        ratelimiter: Option<
            governor::RateLimiter<String, dashmap::DashMap<String, governor::state::InMemoryState>, governor::clock::QuantaClock, governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>
        >,
        ratelimiter_type: RatelimiterType,
    ) -> Self {
        Self { ratelimiter, ratelimiter_type }
    }
    pub(crate) fn check_key(&self, key: &String) -> Result<(), NotUntil<QuantaInstant>> {
        if let Some(lim) = &self.ratelimiter {
            return lim.check_key(key);
        } else {
            unreachable!("ratelimitercommon");
        }
    }
}