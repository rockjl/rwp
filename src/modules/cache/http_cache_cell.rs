/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[derive(Debug, Default)]
pub(crate) struct HttpCacheShared {
    pub(crate) cache: std::sync::Mutex<HashMap<HttpCacheKey, HttpCacheCell>>,
}
#[derive(Debug, Default)]
pub(crate) struct HttpCacheNoShared {
    pub(crate) cache: HashMap<HttpCacheKey, HttpCacheCell>,
}
#[derive(Debug, Default)]
pub(crate) struct HttpCacheCell {
    pub(crate) header_cache: HeaderMap,
    pub(crate) status_cache: StatusCode,
    pub(crate) version_cache: Version,
    pub(crate) body_cache: Bytes,
}
#[derive(std::cmp::Eq, Debug, Clone)]
pub(crate) struct HttpCacheKey {
    pub(crate) uri: Uri,
    pub(crate) save_point: std::cell::RefCell<Option<std::time::Instant>>,
    pub(crate) expire: std::cell::RefCell<Option<Duration>>,
    pub(crate) hit: std::cell::RefCell<Option<i32>>,
}
impl HttpCacheKey {
    pub fn is_expire(key: &HttpCacheKey) -> bool {
        if let Some(save_point) = *key.save_point.borrow() {
            if let Some(expire) = *key.expire.borrow() {
                if save_point.elapsed().as_millis() > expire.as_millis() {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}
impl std::cmp::PartialEq for HttpCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.uri.eq(&other.uri)
    }
}
impl std::hash::Hash for HttpCacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uri.hash(state);
    }
}
impl HttpCacheKey {
    pub(crate) fn for_key(uri: Uri) -> Self {
        Self {
            uri,
            save_point: std::cell::RefCell::new(None),
            expire: std::cell::RefCell::new(None),
            hit: std::cell::RefCell::new(None),
        }
    }
}

impl HttpCacheShared {
    pub(crate) fn lock(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<HttpCacheKey, HttpCacheCell>> {
        self.cache.lock().unwrap()
    }
    pub(crate) fn memory_cache_value(
        status_code: StatusCode,
        version: Version,
        headers: HeaderMap,
        body: Bytes,
    ) -> HttpCacheCell {
        HttpCacheCell {
            status_cache: status_code,
            version_cache: version,
            header_cache: headers,
            body_cache: body,
        }
    }
}
impl HttpCacheCell {
    pub(crate) fn clone_status(&self) -> StatusCode {
        self.status_cache.clone()
    }
    pub(crate) fn clone_version(&self) -> Version {
        self.version_cache.clone()
    }
    pub(crate) fn clone_headers(&self) -> HeaderMap {
        self.header_cache.clone()
    }
    pub(crate) fn clone_body(&self) -> Bytes {
        self.body_cache.clone()
    }
    pub(crate) fn to_origin(self) -> (HeaderMap, Version, StatusCode, Bytes) {
        (self.header_cache, self.version_cache, self.status_cache, self.body_cache)
    }
}
impl HttpCacheCell {
    pub(crate) fn headers_to_string(&self) -> String {
        let mut to_str = "{".to_string();
        for (k, v) in &self.header_cache {
            to_str.push('"');
            to_str += k.to_string().as_str();
            to_str.push('"');
            to_str += ":";
            to_str.push('"');
            to_str += v.to_str().unwrap();
            to_str.push('"');
            to_str += ",";
        }
        to_str = to_str[0..to_str.len() - 1].to_string();
        to_str += "}";
        to_str
    }
}
use std::collections::HashMap;
use std::time::Duration;

use bytes::Bytes;
use futures::AsyncReadExt;
use http::{HeaderMap, HeaderName, StatusCode, Uri, Version};
use serde::ser::{Serialize, SerializeMap, SerializeStruct, Serializer};
impl serde::ser::Serialize for HttpCacheCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut cell = serializer.serialize_struct("Cell", 4)?;
        cell.serialize_field("status", self.status_cache.as_str())?;
        let version = match self.version_cache {
            Version::HTTP_09 => "HTTP/0.9",
            Version::HTTP_10 => "HTTP/1.0",
            Version::HTTP_11 => "HTTP/1.1",
            Version::HTTP_2 => "HTTP/2.0",
            Version::HTTP_3 => "HTTP/3.0",
            _ => "",
        };
        cell.serialize_field("version", version)?;
        cell.serialize_field("header", self.headers_to_string().as_str())?;
        let body = self.body_cache.to_vec();
        let body_str = body.iter().map(|item| {
            item.to_string()
        }).collect::<Vec<String>>().join(",");
        cell.serialize_field("body", &body_str)?;
        cell.end()
    }
}
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
impl<'de> serde::de::Deserialize<'de> for HttpCacheCell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            StatusCache,
            VersionCache,
            HeaderCache,
            BodyCache,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`status_cache` or `version_cache` or `header_cache` or `body_cache`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "status" => Ok(Field::StatusCache),
                            "version" => Ok(Field::VersionCache),
                            "header" => Ok(Field::HeaderCache),
                            "body" => Ok(Field::BodyCache),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct HttpCacheCellVisitor;

        impl<'de> Visitor<'de> for HttpCacheCellVisitor {
            type Value = HttpCacheCell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_map<V>(self, mut map: V) -> Result<HttpCacheCell, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut status_code: Option<String> = None;
                let mut version: Option<String> = None;
                let mut headers: Option<String> = None;
                let mut body: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::StatusCache => {
                            if status_code.is_some() {
                                return Err(de::Error::duplicate_field("Status_cache"));
                            }
                            status_code = Some(map.next_value()?);
                        }
                        Field::VersionCache => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("Version_cache"));
                            }
                            version = Some(map.next_value()?);
                        }
                        Field::HeaderCache => {
                            if headers.is_some() {
                                return Err(de::Error::duplicate_field("Header_cache"));
                            }
                            headers = Some(map.next_value()?);
                        }
                        Field::BodyCache => {
                            if body.is_some() {
                                return Err(de::Error::duplicate_field("Body_cache"));
                            }
                            body = Some(map.next_value()?);
                        }
                    }
                }
                let status_code = status_code.ok_or_else(|| de::Error::missing_field("Status_cache"))?;
                let version = version.ok_or_else(|| de::Error::missing_field("Version_cache"))?;
                let headers = headers.ok_or_else(|| de::Error::missing_field("Header_cache"))?;
                let body = body.ok_or_else(|| de::Error::missing_field("Body_cache"))?;

                let status_code = StatusCode::from_bytes(status_code.as_bytes()).unwrap();
                let version = match version.as_str() {
                    "HTTP/0.9" => Version::HTTP_09,
                    "HTTP/1.0" => Version::HTTP_10,
                    "HTTP/1.1" => Version::HTTP_11,
                    "HTTP/2.0" => Version::HTTP_2,
                    "HTTP/3.0" => Version::HTTP_3,
                    _ => Version::HTTP_09,
                };
                let headers_map: HashMap<String, String> = serde_json::from_str(&headers).unwrap();
                let mut headers = HeaderMap::new();
                for (k, v) in headers_map {
                    headers.insert(HeaderName::from_bytes(k.as_bytes()).unwrap(), v.parse().unwrap());
                }
                let body = Bytes::from_iter(body.split(",").into_iter().map(|item|{
                    item.parse::<u8>().unwrap()
                }));
                Ok(HttpCacheShared::memory_cache_value(status_code, version, headers, body))
            }
        }

        const FIELDS: &'static [&'static str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, HttpCacheCellVisitor)
    }
}