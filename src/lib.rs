/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#![allow(dead_code, unused_imports, unused_variables)]
#![feature(test)]
pub(crate) mod entitys;
pub(crate) mod common;
pub(crate) mod client;
pub(crate) mod instance;
pub(crate) mod context;
pub(crate) mod servers;
pub(crate) mod loggers;
pub(crate) mod util;
pub(crate) mod cli;
pub mod error;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod config;
pub(crate) mod modules;
pub mod app;


pub use app::RockGateway;