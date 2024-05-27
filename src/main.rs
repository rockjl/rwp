/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#![allow(dead_code, unused_imports, unused_variables)]
use cli::args::EnvArgs;
use rock_waypoint::RockGateway;
use std::io::Write;

mod cli;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let args = EnvArgs::new();
    start_log_env(&args);
    let _ = RockGateway::start(&args.config).map_err(| e | {
        log::error!("Gateway ERROR>{:#?}", e);
    });
}

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
