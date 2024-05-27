/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

/* 
    year- y,
    month - m,
    week - w,
    day - d,
    hour - h,
    minute - min,
    second - s,
    millisecond - ms,
    microsecond - us,
    nanosecond - ns,
    picosecond - ps
*/
pub(crate) enum TimeUnit {
    YEAR(String), MONTH(String), WEEK(String), DAY(String), HOUR(String), MINUTE(String), SECOND(String), MILLISECOND(String), MICROSECOND(String), NANOSECOND(String), PICOSECOND(String), NONE
}
impl TimeUnit {
    pub(crate) fn parse(time_str: String) -> std::time::Duration {
        match TimeUnit::analysis_unit(time_str) {
            Self::PICOSECOND(time) => {
                std::time::Duration::from_nanos(time.parse::<u64>().unwrap_or_else(|_| { 0 }) / 1000)
            }
            Self::NANOSECOND(time) => {
                std::time::Duration::from_nanos(time.parse::<u64>().unwrap_or_else(|_| { 0 }))
            }
            Self::MICROSECOND(time) => {
                std::time::Duration::from_micros(time.parse::<u64>().unwrap_or_else(|_| { 0 }))
            }
            Self::MILLISECOND(time) => {
                std::time::Duration::from_millis(time.parse::<u64>().unwrap_or_else(|_| { 0 }))
            }
            Self::SECOND(time) => {
                std::time::Duration::from_secs(time.parse::<u64>().unwrap_or_else(|_| { 0 }))
            }
            Self::MINUTE(time) => {
                std::time::Duration::from_secs(time.parse::<u64>().unwrap_or_else(|_| { 0 }) * 60)
            }
            Self::HOUR(time) => {
                std::time::Duration::from_secs(time.parse::<u64>().unwrap_or_else(|_| { 0 }) * 60 * 60)
            }
            Self::DAY(time) => {
                std::time::Duration::from_secs(time.parse::<u64>().unwrap_or_else(|_| { 0 }) * 24 * 60 * 60)
            }
            Self::WEEK(time) => {
                std::time::Duration::from_secs(time.parse::<u64>().unwrap_or_else(|_| { 0 }) * 7 * 24 * 60 * 60)
            }
            Self::MONTH(time) => {
                std::time::Duration::from_secs(time.parse::<u64>().unwrap_or_else(|_| { 0 }) * 30 * 24 * 60 * 60)
            }
            Self::YEAR(time) => {
                std::time::Duration::from_secs(time.parse::<u64>().unwrap_or_else(|_| { 0 }) * 365 * 24 * 60 * 60)
            }
            _ => {
                std::time::Duration::from_nanos(0)
            }
        }
    }
    fn analysis_unit(time_str: String) -> Self {
        if time_str.contains("ps") {
            return Self::PICOSECOND(time_str.replace("ps", ""));
        }
        if time_str.contains("ns") {
            return Self::NANOSECOND(time_str.replace("ns", ""));
        }
        if time_str.contains("us") {
            return Self::MICROSECOND(time_str.replace("us", ""));
        }
        if time_str.contains("ms") {
            return Self::MILLISECOND(time_str.replace("ms", ""));
        }
        if time_str.contains("s") {
            return Self::SECOND(time_str.replace("s", ""));
        }
        if time_str.contains("min") {
            return Self::MINUTE(time_str.replace("min", ""));
        }
        if time_str.contains("h") {
            return Self::HOUR(time_str.replace("h", ""));
        }
        if time_str.contains("d") {
            return Self::DAY(time_str.replace("d", ""));
        }
        if time_str.contains("w") {
            return Self::WEEK(time_str.replace("w", ""));
        }
        if time_str.contains("m") {
            return Self::MONTH(time_str.replace("m", ""));
        }
        if time_str.contains("y") {
            return Self::YEAR(time_str.replace("y", ""));
        }
        Self::NONE
    }
}