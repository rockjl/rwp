/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{net::SocketAddr, sync::Arc};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{tcp::{ReadHalf, WriteHalf}, TcpStream}};

use crate::{entitys::buf::DataBuf, error::{PipeError, PipeErrorKind, RResult, GatewayError}, gateway_err};


#[derive(Debug)]
pub struct TcpSenderConnection {
    pub(crate) dst_addr: tokio::sync::RwLock<Option<SocketAddr>>,
    pub(crate) stream: tokio::sync::RwLock<Option<TcpStream>>,
    pub(crate) buf_size: usize,
}
unsafe impl Send for TcpSenderConnection {}
// unsafe impl Sync for TcpSenderConnection {}
impl TcpSenderConnection {
    pub(crate) fn new(buf_size: usize) -> Self {
        Self { dst_addr: tokio::sync::RwLock::new(None), stream: tokio::sync::RwLock::new(None), buf_size}
    }
    pub(crate) async fn is_connected(&self) -> bool {
        self.stream.read().await.is_some()
    }
    pub(crate) async fn disconnect(&self) {
        let mut self_stream = self.stream.write().await;
        let mut self_dst_addr = self.dst_addr.write().await;
        if let Some(ref mut stream) = *self_stream {
            match stream.shutdown().await {
                Ok(_) => { }
                Err(e) => {
                    log::error!("TcpSenderConnection >> TcpStream shutdown Error:{:#?}", e);
                }
            }
        }
        *self_stream = None;
        *self_dst_addr = None;
    }
    pub(crate) async fn connect(&self, dst_addr: SocketAddr) -> RResult<()> {
        let mut self_stream = self.stream.write().await;
        let mut self_dst_addr = self.dst_addr.write().await;
        *self_dst_addr = Some(dst_addr);
        log::info!("TcpStream connect: dst_addr: {:?}", self_dst_addr.unwrap());
        match tokio::time::timeout(std::time::Duration::from_millis(1000 * 5), TcpStream::connect(self_dst_addr.unwrap())).await {
            Ok(connect_ret) => {
                match connect_ret {
                    Ok(stream) => {
                        *self_stream = Some(stream);
                    },
                    Err(e) => {
                        log::error!("TcpStream::connect failed: {:#?}", e);
                        *self_stream = None;
                        return Err(e.into());
                    }
                }
            }
            Err(e) => {
                log::error!("TcpStream::connect timeout:{:#?} - {:#?}", e, self_dst_addr);
            }
        }
        Ok(())
    }
    pub(crate) async fn start_read_loop(&self, 
        sender: Arc<tokio::sync::mpsc::Sender<DataBuf>>,
        in_rx: Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<DataBuf>>>,
        timeout: std::time::Duration,
    ) -> RResult<()> {
        loop {
            let mut data_buf = DataBuf::new(self.buf_size, Some(self.buf_size));
            // take stream_lock begin
            let mut sw: tokio::sync::RwLockWriteGuard<'_, Option<TcpStream>> = self.stream.write().await;
            if sw.is_none() {
                return Err(gateway_err!(TcpSenderError, format!("TcpSender Error: TcpStream not ready!").as_str(), PipeError::new(PipeErrorKind::TCPSENDERTIMEOUT)));
            }
            let stream = sw.as_mut().unwrap();
            // take stream_lock end
            let mut in_rx_write = in_rx.write().await;
            tokio::select! {
                read_ret = stream.read_buf(&mut data_buf) => {
                    drop(in_rx_write);
                    drop(sw);
                    match read_ret {
                        Ok(read_size) => {
                            log::info!("read sucdess:{:#?}", read_size);
                            if read_size == 0 {
                                break;
                            }
                            println!("out_tx.send:{:#?}", data_buf.buf_to_string());
                            sender.send(data_buf).await.expect("out_tx send called failed");
                        }
                        Err(e) => {
                            log::error!("read error:{:#?}", e);
                        }
                    }
                }
                in_recv = in_rx_write.recv() => {
                    drop(in_rx_write);
                    match in_recv {
                        Some(mut d_b) => {
                            drop(sw);
                            // take stream_lock begin
                            let mut sw: tokio::sync::RwLockWriteGuard<'_, Option<TcpStream>> = self.stream.write().await;
                            if sw.is_none() {
                                return Err(gateway_err!(TcpSenderError, format!("TcpSender Error: TcpStream not ready!").as_str(), PipeError::new(PipeErrorKind::TCPSENDERTIMEOUT)));
                            }
                            let stream = sw.as_mut().unwrap();
                            // take stream_lock end
                            match tokio::time::timeout(timeout, stream.write_buf(&mut d_b)).await {
                                Ok(write_ret) => {
                                    match write_ret {
                                        Ok(_) => {
                                            log::info!("write success:{:#?}", d_b.buf_to_string());
                                        }
                                        Err(e) => {
                                            log::error!("write Error:{:#?}", e);
                                            self.disconnect().await;
                                            return Err(gateway_err!(TcpSenderError, format!("TcpSender Error: TcpStream write error:{:#?}", e).as_str(), PipeError::new(PipeErrorKind::TCPSENDERTIMEOUT)));
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("write timeout:{:#?} - {:#?}", e, self.dst_addr);
                                    self.disconnect().await;
                                    break;
                                }
                            }
                            drop(sw);
                        }
                        None => {
                            log::error!("in_rx None");
                            self.disconnect().await;
                            break;
                        }
                    }
                    
                }
            }
        }
        Ok(())
    }
    // pub(crate) async fn send(&self, data_buf: &mut DataBuf) -> RResult<usize> {
    //     println!("TcpSenderConnection::send:{:#?}", data_buf.buf_to_string());
    //     let mut sw = self.stream.write().await;
    //     if sw.is_none() {
    //         return Err(gateway_err!(TcpSenderError, format!("TcpSender Error: TcpStream not ready!").as_str(), PipeError::new(PipeErrorKind::TCPSENDERTIMEOUT)));
    //     }
    //     let stream = sw.as_mut().unwrap();
    //     match tokio::time::timeout(self.timeout, stream.write_buf(data_buf)).await {
    //         Ok(write_ret) => {
    //             match write_ret {
    //                 Ok(write_size) => {
    //                     return Ok(write_size);
    //                 }
    //                 Err(e) => {
    //                     self.disconnect().await;
    //                     log::error!("TcpSender Error: TcpStream write:{:#?}", e);
    //                     return Err(gateway_err!(TcpSenderError, format!("TcpSender Error: TcpStream write:{:#?}", e).as_str(), PipeError::new(PipeErrorKind::DISPATCHETCPSENDER)));
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             self.disconnect().await;
    //             log::error!("TcpSender Error: TcpSender Timeout elapsed:{:#?}", e);
    //             return Err(gateway_err!(TcpSenderError, format!("TcpSender Timeout: elapsed:{:#?}", e).as_str(), PipeError::new(PipeErrorKind::TCPSENDERTIMEOUT)));
    //         }
    //     }
    // }
}


