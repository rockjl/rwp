/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use hyper::http::{HeaderMap, Uri, StatusCode, Version};
use hyper::body::Bytes;

#[derive(Debug)]
pub(crate) struct ResponseContext {
    pub(crate) status: StatusCode,
    pub(crate) version: Version, 
    pub(crate) headers: HeaderMap,
    pub(crate) body: Bytes,
    pub(crate) is_new: bool,
}
impl ResponseContext {
    pub(crate) fn refresh(&mut self) {
        self.is_new = true;
    }
    pub(crate) fn set_old(&mut self) {
        self.is_new = false;
    }
    pub(crate) fn is_new(&self) -> bool {
        self.is_new
    }
}
impl Default for ResponseContext {
    fn default() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            is_new: false,
        }
    }
}
impl Clone for ResponseContext {
    fn clone(&self) -> Self {
        Self { 
            status: self.status.clone(),
            version: self.version.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
            is_new: self.is_new.clone(),
        }
    }
}