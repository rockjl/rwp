/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

#[cfg(unix)]
pub(crate) async fn signal_hook() {
    let mut stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).expect("hahaha hohoho");
    loop {
        stream.recv().await;
        break;
    }
}

#[cfg(windows)]
pub(crate) async fn signal_hook() {
    tokio::signal::ctrl_c().await.expect("hahaha hohoho");
}