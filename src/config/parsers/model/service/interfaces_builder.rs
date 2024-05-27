/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use serde::Deserialize;

use crate::{error::{BuilderError, BuilderErrorKind, RResult, GatewayError}, instance::service::AddressInterface};


#[derive(Debug, Deserialize)]
pub(crate) struct InterfaceBuilder {
    pub(crate) address: String,
    pub(crate) ssl_cert: Option<String>,
    pub(crate) ssl_key: Option<String>,
}
impl InterfaceBuilder {
    pub(crate) fn make_address_interface(&self, protocol: &str) -> RResult<AddressInterface> {
        Ok(match protocol {
            "https" => AddressInterface::Https { 
                addr: self.address.parse().map_err(|e| {
                    gateway_err!(
                        ConfigurationFailed,
                        format!("Failed to parse the listener address {}", self.address),
                        e
                    )
                })?, 
                cert: self.ssl_cert.clone().unwrap(), 
                key: self.ssl_key.clone().unwrap(), 
            },
            "http" => AddressInterface::Http { 
                addr: self.address.parse().map_err(|e| {
                    gateway_err!(
                        ConfigurationFailed,
                        format!("Failed to parse the listener address {}", self.address),
                        e
                    )
                })?,
            },
            "tcp" => AddressInterface::Tcp { 
                addr: self.address.parse().map_err(|e| {
                    gateway_err!(
                        ConfigurationFailed,
                        format!("Failed to parse the listener address {}", self.address),
                        e
                    )
                })?, 
            },
            _ => {
                return Err(gateway_err!(
                    BuilderFailed, 
                    format!("Builder interface ERROR > Not found protocol:{:#?}", protocol).as_str(), 
                    BuilderError::new(BuilderErrorKind::INTERFACES))
                );
            }
        })
    }
}