/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use bytes::Bytes;
use http::{header, HeaderMap, HeaderName, Response, StatusCode, Version};
use http_body_util::Full;
use package_info::PackageInfo;

use crate::{error::RResult, util::gateway_info::CargoPackageInfo};

use super::{content_type::ContentTypeAndExtension, file_system::{self, FileSystem}};

pub(crate) struct HttpFile {
    pub(crate) file_content: Vec<u8>,
    pub(crate) file_extension: Option<String>,
    pub(crate) http_headers: HeaderMap,
    pub(crate) status: StatusCode,
}
impl HttpFile {
    pub(crate) async fn to_http_response(self) -> (StatusCode, HeaderMap, Bytes) {
        (self.status, self.http_headers, self.file_content.into())
    }
    pub(crate) async fn file_response_structure(
        file_path: &str, 
        file_system: &FileSystem, 
        ctae: &ContentTypeAndExtension, 
        request_header: &HeaderMap) -> RResult<Self> {
        let mut file_content = file_system.read_file(file_path).await?;
        let status = if file_content.is_some() { StatusCode::OK } else { StatusCode::NOT_FOUND };
        let file_extension = if file_content.is_some() {
            let path = match std::path::Path::new(file_path).extension().map(|o| {
                let tmp = o.to_str().unwrap();
                let tmp = tmp.trim();
                if tmp != "" {
                    let tmp = tmp.split("?").into_iter().collect::<Vec<&str>>();
                    let tmp = tmp.get(0).unwrap();
                    return *tmp;
                } else {
                    return "";
                }
            }) {
                Some(extension) => { 
                    Some(extension.to_string()) 
                }
                None => { None }
            };
            path
        } else { None };
        let file_name = match std::path::Path::new(file_path).file_name().map(|o| {
            let tmp = o.to_str().unwrap();
            let tmp = tmp.trim();
            if tmp != "" {
                let tmp = tmp.split("?").into_iter().collect::<Vec<&str>>();
                let tmp = tmp.get(0).unwrap();
                return *tmp;
            } else {
                return "";
            }
        }) {
            Some(f_n) => { f_n.to_string() }
            None => { "rock_waypoint".to_string() }
        };
        /* start building the header */
        let mut http_headers = HeaderMap::new();
        if let Some(fc) = &file_content {
            if request_header.get(header::CONNECTION).is_some_and(|v| {
                v.to_str().unwrap().trim() == "keep-alive"
            }) {
                http_headers.insert(header::CONNECTION, "keep-alive".to_string().parse().unwrap());
            }
            http_headers.insert(HeaderName::from_static("server"), CargoPackageInfo::name().unwrap().to_string().parse().unwrap());
            http_headers.insert(header::CONTENT_LENGTH, fc.len().to_string().parse().unwrap());
            if let Some(ext) = &file_extension {
                http_headers.insert(header::CONTENT_TYPE, 
                    ctae.take_content_type((".".to_string() + ext).as_str())
                            .unwrap_or_else(||mime_guess::mime::APPLICATION_OCTET_STREAM.to_string()).parse().unwrap()
                );
                if ext.eq_ignore_ascii_case("png") ||
                    ext.eq_ignore_ascii_case("gif") ||
                    ext.eq_ignore_ascii_case("svg") ||
                    ext.eq_ignore_ascii_case("jpeg") ||
                    ext.eq_ignore_ascii_case("bmp") ||
                    ext.eq_ignore_ascii_case("jpg") ||
                    ext.eq_ignore_ascii_case("woff") ||
                    ext.eq_ignore_ascii_case("ttf")
                {
                    http_headers.insert(header::ACCEPT_RANGES, "bytes".to_string().parse().unwrap());
                } 
                if ext.eq_ignore_ascii_case("js") || ext.eq_ignore_ascii_case("css") {
                    if request_header.get(header::ACCEPT_ENCODING).is_some_and(|v| {
                        v.to_str().unwrap().contains("gzip")
                    }){
                        http_headers.insert(header::ACCEPT_RANGES, "bytes".to_string().parse().unwrap());
                        http_headers.insert(header::CONTENT_ENCODING, "identity".to_string().parse().unwrap());
                    } else if request_header.get(header::ACCEPT_ENCODING).is_some_and(|v| {
                        v.to_str().unwrap().contains("br")
                    }) {
                        http_headers.insert(header::ACCEPT_RANGES, "bytes".to_string().parse().unwrap());
                        http_headers.insert(header::CONTENT_ENCODING, "identity".to_string().parse().unwrap());
                    } else {
                        http_headers.insert(header::CONTENT_ENCODING, "identity".to_string().parse().unwrap());
                    }
                } 
            } else {
                if request_header.get(header::ACCEPT).is_some_and(|v| {
                    v.to_str().unwrap().contains("text/html")
                }) {
                    http_headers.insert(header::CONTENT_TYPE, mime_guess::mime::TEXT_HTML_UTF_8.to_string().parse().unwrap());
                    http_headers.insert(header::CONTENT_ENCODING, "identity".to_string().parse().unwrap());
                } else if request_header.get(header::ACCEPT).is_some_and(|v| {
                    let accept = v.to_str().unwrap().split(",").into_iter().collect::<Vec<&str>>();
                    for a in accept {
                        if a.starts_with("image") { return true; }
                    }
                    false
                }) {
                    http_headers.insert(header::ACCEPT_RANGES, "bytes".to_string().parse().unwrap());
                    http_headers.insert(header::CONTENT_TYPE, "image/webp".to_string().parse().unwrap());
                } else {
                    http_headers.insert(header::ACCEPT_RANGES, "bytes".to_string().parse().unwrap());
                    http_headers.insert(header::CONTENT_TYPE, mime_guess::mime::APPLICATION_OCTET_STREAM.to_string().parse().unwrap());
                    http_headers.insert(header::CONTENT_DISPOSITION, format!("attachment:filename={:?}", file_name).parse().unwrap());
                }
            }

        } else { // 404
            let body_str = format!("<html>
            <head><title>404 Not Found</title></head>
            <body>
            <center><h1>404 Not Found</h1></center>
            <hr><center>{:?}/{:?}</center>
            </body>
            </html>", CargoPackageInfo::name().unwrap(), CargoPackageInfo::version().unwrap());
            file_content = Some(body_str.as_bytes().to_vec());
            http_headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());
            http_headers.insert(header::CONTENT_TYPE, mime_guess::mime::TEXT_HTML.to_string().parse().unwrap());
            http_headers.insert(header::CONTENT_LENGTH, body_str.len().to_string().parse().unwrap());
        }
        Ok(Self {
            file_content: file_content.unwrap_or_else(|| { Vec::new() }),
            file_extension,
            http_headers,
            status,
        })
    }
}