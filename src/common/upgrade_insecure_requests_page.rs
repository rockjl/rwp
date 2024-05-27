/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use hyper::http::{header, HeaderMap, Response, StatusCode, Version, Uri};
use http_body_util::Full;
use hyper::body::Bytes;

pub fn upgrade_insecure_request(uri: Uri) -> Response<Full<Bytes>> {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, mime_guess::mime::TEXT_HTML.to_string().parse().unwrap());
    headers.insert(header::CONTENT_LENGTH, "0".parse().unwrap());
    headers.insert(header::LOCATION, uri.to_string().parse().unwrap());
    let mut response = Response::new(Full::new(Bytes::new()));
    *response.headers_mut() = headers;
    *response.status_mut() = StatusCode::FOUND;
    *response.version_mut() = Version::HTTP_11;

    response
}