/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(crate) struct OutBuilder {
    pub(crate) r#type: OutType,
    pub(crate) path: Option<String>,
    pub(crate) out_host: Option<String>,
    pub(crate) root_path: Option<String>,
    pub(crate) index_file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum OutType {
    Network,
    File,
}