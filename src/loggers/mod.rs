/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::io::Write;

use crate::cli::args::EnvArgs;
pub(crate) fn start_log_env(args: &EnvArgs) {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} THREAD:{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                std::thread::current().name().unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, args.log_level)
        .init();
}