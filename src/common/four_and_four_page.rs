/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use http::{header, HeaderMap, Response, StatusCode, Version};
use http_body_util::Full;
use hyper::body::Bytes;
use package_info::PackageInfo;

use crate::{instance::errors::ReturnTypes, util::gateway_info::CargoPackageInfo};


pub fn page404() -> Response<Full<Bytes>> {
    let body_str = format!("<html>
    <head><title>404 Not Found</title></head>
    <body>
    <center><h1>404 Not Found</h1></center>
    <hr><center>{:?}/{:?}</center>
    </body>
    </html>", CargoPackageInfo::name().unwrap(), CargoPackageInfo::version().unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, mime_guess::mime::TEXT_HTML.to_string().parse().unwrap());
    headers.insert(header::CONTENT_LENGTH, body_str.len().to_string().parse().unwrap());
    let mut response = Response::new(Full::new(Bytes::from(body_str)));
    *response.headers_mut() = headers;
    *response.status_mut() = StatusCode::NOT_FOUND;
    *response.version_mut() = Version::HTTP_11;

    response
}

pub fn response_page(status_code: StatusCode) -> (StatusCode, Version, HeaderMap, Bytes) {
    let body_str = format!("<html>
    <head><title>{:#?}</title></head>
    <body>
    <center><h1>{:#?}</h1></center>
    <hr><center>{:?}/{:?}</center>
    </body>
    </html>", status_code, status_code, CargoPackageInfo::name().unwrap(), CargoPackageInfo::version().unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, mime_guess::mime::TEXT_HTML.to_string().parse().unwrap());
    headers.insert(header::CONTENT_LENGTH, body_str.len().to_string().parse().unwrap());
    (status_code, Version::HTTP_11, headers, Bytes::from(body_str))
}