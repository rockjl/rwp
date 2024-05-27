/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub(crate) fn assemble_uri(scheme: &str, host: String, port: u16, path: String, uri_path: &str, parameters: Option<String>) -> hyper::http::Uri {
    let mut uri_str = if (path.is_empty() || path.eq("/")) && (!uri_path.is_empty() && !uri_path.eq("/")) {
        let uri_path = if uri_path.starts_with("/") {
            let mut chars = uri_path.chars();
            chars.next();
            chars.as_str()
        } else {
            uri_path
        };
        format!("{}://{}:{}/{}", scheme, host, port, uri_path)
    } else if (!path.is_empty() && !path.eq("/")) && (uri_path.is_empty() || uri_path.eq("/")) {
        format!("{}://{}:{}/{}", scheme, host, port, path)
    } else if (path.is_empty() || path.eq("/")) && (uri_path.is_empty() || uri_path.eq("/")) {
        format!("{}://{}:{}", scheme, host, port)
    } else {
        format!("{}://{}:{}/{}/{}", scheme, host, port, path, uri_path)
    };
    uri_str = if let Some(param) = parameters {
        uri_str + "?" + param.as_str()
    } else {
        uri_str
    };
    uri_str.parse().unwrap()
}