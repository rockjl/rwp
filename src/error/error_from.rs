/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use deadpool_redis::CreatePoolError;
use http::header::{InvalidHeaderValue, InvalidHeaderName};

use crate::gateway_err;

use super::GatewayError;
use std::net::AddrParseError;



impl From<std::io::Error> for GatewayError {
    fn from(err: std::io::Error) -> Self {
        gateway_err!(IoError, "An IO Error occurred", err)
    }
}

impl From<regex::Error> for GatewayError {
    fn from(err: regex::Error) -> Self {
        gateway_err!(ConfigurationFailed, "Could not compile regex", err)
    }
}

impl From<http::method::InvalidMethod> for GatewayError {
    fn from(err: http::method::InvalidMethod) -> Self {
        gateway_err!(ConfigurationFailed, "Invalid HTTP method", err)
    }
}

impl From<config::ConfigError> for GatewayError {
    fn from(err: config::ConfigError) -> Self {
        gateway_err!(ConfigurationFailed, "Configuration error", err)
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(err: serde_json::Error) -> Self {
        gateway_err!(ConfigurationFailed, "Configuration json parse error", err)
    }
}

impl From<std::num::ParseIntError> for GatewayError {
    fn from(err: std::num::ParseIntError) -> Self {
        gateway_err!(ConfigurationFailed, "Invalid number format", err)
    }
}

impl From<http::uri::InvalidUri> for GatewayError {
    fn from(e: http::uri::InvalidUri) -> Self {
        gateway_err!(ConfigurationFailed, "Unable to parse URI", e)
    }
}

impl From<AddrParseError> for GatewayError {
    fn from(e: AddrParseError) -> Self {
        gateway_err!(ConfigurationFailed, "Unable to parse network address", e)
    }
}

impl From<hyper::Error> for GatewayError {
    fn from(e: hyper::Error) -> Self {
        gateway_err!(HyperError, "Hyper Error", e)
    }
}

impl From<hyper_util::client::legacy::Error> for GatewayError {
    fn from(e: hyper_util::client::legacy::Error) -> Self {
        gateway_err!(HyperError, "Hyper-util Error", e)
    }
}

impl From<super::BuilderError> for GatewayError {
    fn from(e: super::BuilderError) -> Self {
        gateway_err!(BuilderFailed, "Build instance Error", e)
    }
}

impl From<super::PipeError> for GatewayError {
    fn from(e: super::PipeError) -> Self {
        gateway_err!(BuilderFailed, "Execute pipes Error", e)
    }
}

impl From<InvalidHeaderName> for GatewayError {
    fn from(e: InvalidHeaderName) -> Self {
        gateway_err!(ConfigurationFailed, "Failed Config Header_Name", e)
    }
}

impl From<InvalidHeaderValue> for GatewayError {
    fn from(e: InvalidHeaderValue) -> Self {
        gateway_err!(ConfigurationFailed, "Failed Config Header_Value", e)
    }
}

impl From<&'static str> for GatewayError {
    fn from(err: &'static str) -> Self {
        gateway_err!(Other, err)
    }
}

impl From<String> for GatewayError {
    fn from(err: String) -> Self {
        gateway_err!(Other, err)
    }
}

impl From<CreatePoolError> for GatewayError {
    fn from(err: CreatePoolError) -> Self {
        gateway_err!(RedisFailed, "Failed to Connect Redis", err)
    }
}

impl From<deadpool_redis::redis::RedisError> for GatewayError {
    fn from(err: deadpool_redis::redis::RedisError) -> Self {
        gateway_err!(RedisFailed, "Redis Error", err)
    }
}

impl From<deadpool_redis::PoolError> for GatewayError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        gateway_err!(RedisFailed, "RedisPool Error", err)
    }
}