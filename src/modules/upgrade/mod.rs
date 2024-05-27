/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::Arc;

use crate::{error::{BuilderError, BuilderErrorKind, GatewayError}, instance::service::AddressInterface};

use self::upgrade_insecure_requests::UpgradeInsecureRequests;

pub(crate) mod upgrade_insecure_requests;

#[derive(Debug)]
pub(crate) struct UpgradeProfile {
    pub(crate) interfaces: Arc<Vec<AddressInterface>>,
    pub(crate) external_ip: Option<String>,
}