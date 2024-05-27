/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::common::file_system::FileSystem;



#[derive(Debug, Clone)]
pub(crate) enum Out {
    Network {
        path: Option<String>,
        out_host: String,
    },
    File {
        file_system: FileSystem,
    }
}
impl Default for Out {
    fn default() -> Self {
        Self::File { file_system: FileSystem::default() }
    }
}