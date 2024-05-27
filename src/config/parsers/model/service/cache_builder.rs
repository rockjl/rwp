/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(crate) struct CacheBuilder {
    pub(crate) memory: Option<CacheMemoryBuilder>,
    pub(crate) redis: Option<CacheRedisBuilder>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CacheMemoryBuilder {
    pub(crate) clear_time_interval: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CacheRedisBuilder {
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) pwd: String,
}