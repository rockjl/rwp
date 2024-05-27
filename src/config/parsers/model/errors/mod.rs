/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ErrBuilder {
    pub(crate) r#type: ErrType,
    pub(crate) error_list: Option<String>,
    pub(crate) pass_next: Option<bool>,
    pub(crate) r#return: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) enum ErrType {
    Http,
    Tcp,
}
