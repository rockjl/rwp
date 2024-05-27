/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::net::SocketAddr;

use hyper::http::{Extensions, Version};
use hyper::http::{HeaderMap, Uri, Method};
use hyper::body::Bytes;

use super::super::scheme::SchemeContext;

#[derive(Debug)]
pub(crate) struct RequestContext {
    pub(crate) remote_addr: SocketAddr,
    pub(crate) uri: Uri,
    pub(crate) parameters: Option<String>,
    pub(crate) scheme: SchemeContext,
    pub(crate) extensions: Extensions,
    pub(crate) version: Version,
    pub(crate) method: Method,
    pub(crate) headers: HeaderMap,
    pub(crate) body: Bytes,
}
impl Default for RequestContext {
    fn default() -> Self {
        Self {
            remote_addr: "0.0.0.0:0".parse().unwrap(),
            uri: "/".parse().unwrap(),
            parameters: None,
            scheme: SchemeContext::HTTP,
            extensions: Extensions::new(),
            version: Version::HTTP_11,
            method: Method::GET,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }
}
impl Clone for RequestContext {
    fn clone(&self) -> Self {
        Self { 
            remote_addr: self.remote_addr.clone(), 
            uri: self.uri.clone(),
            parameters: self.parameters.clone(),
            scheme: self.scheme.clone(),
            extensions: self.extensions.clone(),
            version: self.version.clone(),
            method: self.method.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }
}