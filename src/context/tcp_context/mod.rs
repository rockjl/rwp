/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::sync::Arc;

use crate::entitys::buf::DataBuf;

#[derive(Debug, Clone)]
pub(crate) struct TcpContext {
    pub(crate) sender: Arc<tokio::sync::mpsc::Sender<DataBuf>>,
    pub(crate) in_tx: Arc<tokio::sync::mpsc::Sender<DataBuf>>,
    pub(crate) in_rx: Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<DataBuf>>>,
    pub(crate) in_data_buf: Option<DataBuf>,
}