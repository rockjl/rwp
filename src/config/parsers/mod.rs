/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

mod parser;
pub(crate) mod model;
use std::{path::Path, fs::File, io::prelude::*, ffi::OsStr, };

use crate::error::RResult;

use self::{parser::{Format, Parser}, model::ConfigModel};


pub(crate) fn parse_file(file_path: &str) -> RResult<ConfigModel> {
    let path = Path::new(file_path);
    let open_result = File::open(&path);
    match open_result {
        Ok(f) => {
            log::info!("config file: {:#?}", file_path);
        }
        Err(e) => {
            log::error!("config file not found:{:#?}", e);
        }
    }
    let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap();
    let file_ext = path.extension().and_then(OsStr::to_str).unwrap();
    let parent = path.parent().and_then(|p| p.to_str()).unwrap_or("");
    let config_name = if parent.is_empty() {
        file_stem.to_string()
    } else {
        format!("{}/{}", parent, file_stem)
    };
    let ret: ConfigModel = Parser::from_str(&config_name, Format::ext(file_ext)).unwrap();
    Ok(ret)
}