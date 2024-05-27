/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{net::SocketAddr, sync::Arc};

use futures_util::TryFutureExt;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{tcp, TcpStream}};

use crate::{context::{ContextType, GatewayContext}, entitys::buf::DataBuf, error::{GatewayError, PipeError, PipeErrorKind, RResult}, gateway_err, RockGateway};

pub(crate) async fn tcp_run(
    remote_addr: SocketAddr,
    stream: TcpStream,
    gateway: Arc<RockGateway>,
) -> RResult<()> {
    let instance = gateway.get_instance()?;
    let common_module_lock = gateway.common_module.read().await;
    let common_module = common_module_lock.as_ref().unwrap();
    let route_pipe_task = common_module.route_pipe_task.as_ref();
    let stream_write = tokio::sync::RwLock::new(stream);
    let (in_tx, in_rx) = tokio::sync::mpsc::channel::<DataBuf>(128);
    let (out_tx, mut out_rx) = tokio::sync::mpsc::channel::<DataBuf>(128);
    let out_tx = Arc::new(out_tx);
    let in_tx = Arc::new(in_tx);
    let in_rx = Arc::new(tokio::sync::RwLock::new(in_rx));
    'main: loop {
        let pipe_fut = futures_util::future::ok::<GatewayContext, GatewayError>(
            GatewayContext::new_tcp_context(remote_addr, gateway.clone(), out_tx.clone(), in_tx.clone(), in_rx.clone()).await?
        );
        let mut context = pipe_fut.and_then(|ctx| {
            /* first execute route */
            gateway.execute_one_task(ctx, route_pipe_task)
        }).await?;

        let mut stream_write_lock;
        match &context.route {
            Some(route_name) => {
                stream_write_lock = stream_write.write().await;
                let route = instance.routes.get(route_name).unwrap();
                let mut data_buf = DataBuf::new(route.client_buf_size, Some(route.client_buf_size));
                let in_timeout = route.in_timeout;
                tokio::select! {
                    read_ret = stream_write_lock.read_buf(&mut data_buf) => {
                        match read_ret {
                            Ok(read_size) => {
                                if let ContextType::TcpContext(ref mut tcp_context) = context.context_type {
                                    tcp_context.in_data_buf = Some(data_buf);
                                } else {
                                    unreachable!()
                                }
                                /* second execute pipe line */
                                context = match route.pipe_line.pipe_line_engine.execute(context).await {
                                    Ok(ctx) => { ctx }
                                    Err(e) => { 
                                        log::error!("PIPE_LINE_ERROR>{:#?}", e);
                                        stream_write_lock.write_all("not found route".as_bytes()).await.unwrap();
                                        stream_write_lock.shutdown().await.unwrap();
                                        return Ok(());
                                    }
                                };
                                let _ = context;

                                if read_size == 0 {
                                    break 'main;
                                }
                                drop(stream_write_lock);
                            }
                            Err(e) => {
                                log::error!("Tcp Connect Client Error:{:#?}", e);
                                drop(stream_write_lock);
                                break;
                            }
                        }
                    }
                    recv = out_rx.recv() => {
                        drop(stream_write_lock);
                        match recv {
                            Some(mut d_b) => {
                                // println!("back to browser:{:#?}", d_b.buf_to_string());
                                stream_write_lock = stream_write.write().await;
                                match tokio::time::timeout(in_timeout, stream_write_lock.write_buf(&mut d_b)).await {
                                    Ok(write_ret) => {
                                        match write_ret {
                                            Ok(_) => {
                                                log::info!("write_success:{:#?}", d_b.buf_to_string());
                                            }
                                            Err(e) => {
                                                log::error!("write Error:{:#?}", e);
                                                return Err(gateway_err!(TcpSenderError, format!("TcpSender Error: TcpStream write error:{:#?}", e).as_str(), PipeError::new(PipeErrorKind::TCPSENDERTIMEOUT)));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("write timeout:{:#?}", e);
                                        break;
                                    }
                                }
                                let _ = stream_write_lock.flush().await;
                                drop(stream_write_lock);
                            }
                            None => {
                                log::error!("Tcp Connect Server Error:");
                            }
                        }
                    }
                }
            }
            None => {
                log::error!("PIPE_LINE_ERROR>not found route. <routes> may not be configured.");
                stream_write_lock = stream_write.write().await;
                stream_write_lock.write_all("not found route".as_bytes()).await.unwrap();
                stream_write_lock.shutdown().await.unwrap();
                drop(stream_write_lock);
                return Ok(());
            }
        }
    }
    Ok(())
}