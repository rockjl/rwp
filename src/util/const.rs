/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub(crate) const ROCKJIANG_SERVICE: &'static str = "rockjiang_service";
pub(crate) const BUF_SIZE: usize = 1024;

pub(crate) const BLACK_AND_WHITE_LIST: &'static str = "black_white_list";
pub(crate) const RATELIMITER: &'static str = "ratelimiter";
pub(crate) const MEMORY_CACHE_GET: &'static str = "memory_cache_get";
pub(crate) const MEMORY_CACHE_SET: &'static str = "memory_cache_set";
pub(crate) const REDIS_CACHE_GET: &'static str = "redis_cache_get";
pub(crate) const REDIS_CACHE_SET: &'static str = "redis_cache_set";
pub(crate) const HEADER_REQUEST: &'static str = "header_request";
pub(crate) const HEADER_RESPONSE: &'static str = "header_response";
pub(crate) const DISPATCHE: &'static str = "dispatche";
pub(crate) const DISPATCHE_NETWORK: &'static str = "dispatche_network";
pub(crate) const DISPATCHE_FILE: &'static str = "dispatche_file";
pub(crate) const RETURN: &'static str = "return";
pub(crate) const LEAST_CONNECTION: &'static str = "least_connection";
pub(crate) const RANDOM: &'static str = "random";
pub(crate) const ROUND_ROBIN: &'static str = "round_robin";
pub(crate) const IP_ROUND_ROBIN: &'static str = "ip_round_robin";
pub(crate) const ROUTE: &'static str = "route";
pub(crate) const UPGRADE_INSECURE_REQUESTS: &'static str = "upgrade_insecure_requests";

pub(crate) const ADD: &'static str = "add";
pub(crate) const DEL: &'static str = "del";

pub(crate) const TCP: &'static str = "tcp";
pub(crate) const HTTP: &'static str = "http";
pub(crate) const HTTPS: &'static str = "https";
pub(crate) const WEBSOCKET: &'static str = "websocket";

pub(crate) const REDIS_PREFIX: &'static str = "redis_prefix_";

pub(crate) const DEFAULT_OUT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(20);
pub(crate) const HOST_POINT_MAX_FAILS_DURN: std::time::Duration = std::time::Duration::from_secs(60 * 60);
pub(crate) const MAX_FAILS_DURN_CHECK_KEY: &'static str = "MAX_FAILS_DURN_CHECK_KEY";