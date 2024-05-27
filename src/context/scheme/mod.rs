/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[derive(Debug, Clone)]
pub(crate) enum SchemeContext { HTTP,HTTPS,NONE, }
impl Default for SchemeContext {
    fn default() -> Self {
        SchemeContext::NONE
    }
}
impl SchemeContext {
    pub(crate) fn parse(http_scheme: &str) -> Self {
        match http_scheme {
            "http" => { Self::HTTP }
            "https" => { Self::HTTPS }
            _ => { Self::NONE }
        }
    }
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::HTTP => {
                return "http";
            }
            Self::HTTPS => {
                return "https"
            }
            Self::NONE => {
                return "none";
            }
        }
    }
    pub(crate) fn matchs(&self, protocol: &str) -> bool {
        match self {
            Self::HTTP => {
                if protocol == "http" {
                    return true;
                }
            }
            Self::HTTPS => {
                if protocol == "https" {
                    return true;
                }
            }
            Self::NONE => {
                return false;
            }
        }
        return false;
    }
    pub(crate) fn check_self(self, protocol: &str) -> Self {
        match self {
            Self::HTTP => { self }
            Self::HTTPS => { self }
            _ => {
                SchemeContext::parse(protocol)
            }
        }
    }
}