/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(crate) struct InBuilder {
    pub(crate) r#type: InType,
    pub(crate) pattern: Option<String>,
    pub(crate) method: Option<Vec<String>>,
    pub(crate) ranges: Option<Vec<String>>,
    pub(crate) file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum InType {
    Regex,
    Ip,
    FileIp,
}