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
    // let file_name = path.file_name().and_then(OsStr::to_str).unwrap();
    let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap();
    let file_ext = path.extension().and_then(OsStr::to_str).unwrap();
    let ret: ConfigModel = Parser::from_str(file_stem, Format::ext(file_ext)).unwrap();
    Ok(ret)
}