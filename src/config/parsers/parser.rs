/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::error::{RResult, GatewayError};

pub(crate) enum Format {
    Default,
    Json,
    Yaml,
    Ron,
    Toml,
}
impl Format {
    pub(crate) fn ext(ext: &str) -> config::FileFormat {
        match ext {
            "yml" | "yaml" => config::FileFormat::Yaml,
            "json" => config::FileFormat::Json,
            "ron" => config::FileFormat::Ron,
            "toml" => config::FileFormat::Toml,
            _ => config::FileFormat::Yaml,
        }
    }
}
pub(crate) struct Parser;

impl Parser {
    pub(crate) fn from_str<T: serde::de::DeserializeOwned>(ser: &str, f: config::FileFormat) -> RResult<T> {
        let config = config::Config::builder().add_source(config::File::new(ser, f)).build()?;
        Ok(config.try_deserialize()?)
    }
}