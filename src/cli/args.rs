/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use log::LevelFilter;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "Rock_Waypoint")]
pub struct EnvArgs {
    /// Config file
    #[structopt(short, long, help = "Config file", default_value = "rock.yaml")]
    pub config: String,

    /// Filter to apply to input files
    #[structopt(short, long, help = "Logging level to use", default_value = "info")]
    pub log_level: LevelFilter,
}

impl EnvArgs {
    pub fn new() -> Self {
        EnvArgs::from_args()
    }
}