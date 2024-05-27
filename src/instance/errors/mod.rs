/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use http::StatusCode;

#[derive(Debug)]
pub(crate) struct Errs {
    pub(crate) inner: ErrModule,
}
#[derive(Debug)]
pub(crate) enum ErrModule {
    HTTP(Err),
    TCP,
}
#[derive(Debug)]
pub(crate) struct Err {
    pub(crate) error_list: Vec<ErrTypes>,
    pub(crate) pass_next: bool,
    pub(crate) r#return: ReturnTypes,
}
#[derive(Debug)]
pub(crate) enum ErrTypes {
    Hsc(StatusCode)
}
#[derive(Debug)]
pub(crate) enum ReturnTypes {
    Origin,
    Hsc(StatusCode)
}
impl Default for Errs {
    fn default() -> Self {
        Self { inner: ErrModule::HTTP(Err { error_list: Vec::new(), pass_next: true, r#return: ReturnTypes::Origin }) }
    }
}