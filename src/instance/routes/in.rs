/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::{common::ip_range::IpRange, modules::blackandwhitelist::black_and_white_list::BlackAndWhiteListProfile};


#[derive(Debug, Clone)]
pub(crate) enum In {
    Regex {
        pattern: regex::Regex,
        method: Option<Vec<String>>,
    },
    Ip {
        ranges: BlackAndWhiteListProfile,
        method: Option<Vec<String>>,
    },
    IpFile {
        ranges: BlackAndWhiteListProfile,
        method: Option<Vec<String>>,
    },
}
impl Default for In {
    fn default() -> Self {
        Self::Regex { pattern: regex::Regex::new("/").unwrap(), method: None }
    }
}