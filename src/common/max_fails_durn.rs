/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use governor::{clock::QuantaInstant, NotUntil};

#[derive(Debug)]
pub(crate) struct MaxFailsDurn {
    pub(crate) inner: 
        governor::RateLimiter<&'static str, dashmap::DashMap<&'static str, governor::state::InMemoryState>, governor::clock::QuantaClock, governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>
    ,
}
impl MaxFailsDurn {
    pub(crate) fn new(
        v: governor::RateLimiter<&'static str, dashmap::DashMap<&'static str, governor::state::InMemoryState>, governor::clock::QuantaClock, governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>,
    ) -> Self {
        Self { inner: v }
    }
    pub(crate) fn check_key(&self, key: &&'static str) -> Result<(), NotUntil<QuantaInstant>> {
        self.inner.check_key(key)
    }
}
pub(crate) fn structure_max_fails_durn(period: std::time::Duration, burst: u32) -> MaxFailsDurn {
    // let period = TimeUnit::parse(crate::util::unstructured_to_string::unstructured_to_string(
    //     c["period"].clone(),
    // ));
    // let burst: u32 = unstructured_to_number::unstructured_to_number(c["burst"].clone())?;
    let quota = governor::Quota::with_period(period)
        .unwrap()
        .allow_burst(std::num::NonZeroU32::new(burst).unwrap());
    let lim: governor::RateLimiter<
        &'static str,
        dashmap::DashMap<&'static str, governor::state::InMemoryState>,
        governor::clock::QuantaClock,
        governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>,
    > = governor::RateLimiter::keyed(quota)
        .with_middleware::<governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>>();
    MaxFailsDurn::new(
        lim,
    )
}