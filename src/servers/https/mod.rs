/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{fs::File, io::BufReader, net::SocketAddr, pin::pin, sync::Arc};

use tokio_rustls::{rustls::ServerConfig, TlsAcceptor};

use crate::{error::{GatewayError, RResult}, gateway_err, servers::tokiort::TokioTimer};

use super::{tokiort::TokioIo, GatewayServerInterface};

pub(crate) mod https_service;


pub(crate) struct HttpsServer {
    pub(crate) addr: SocketAddr,
    pub(crate) cert: String,
    pub(crate) key: String,
}
impl HttpsServer {
    fn load_certs(&self, filename: &str) -> RResult<Vec<tokio_rustls::rustls::pki_types::CertificateDer<'static>>> {
        let certfile = File::open(filename).expect(format!("cannot open certificate file:{:?}", filename).as_str());
        let mut reader = BufReader::new(certfile);
        Ok(rustls_pemfile::certs(&mut reader)
            .into_iter()
            .map(|v| {
                v.unwrap()
            })
            .collect())
    }
    
    fn load_private_key(&self, filename: &str) -> RResult<tokio_rustls::rustls::pki_types::PrivateKeyDer<'static>> {
        // rustls_pemfile::rsa_private_keys(&mut BufReader::new(File::open(filename)?))
        //     .next()
        //     .unwrap()
        //     .map(Into::into).map_err(|e| {
        //         e.into()
        //     })
        let keyfile = File::open(filename).expect(format!("cannot open private key file:{:?}", filename).as_str());
        let mut reader = BufReader::new(keyfile);
        loop {
            let private_key  = match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key.pem file") {
                Some(rustls_pemfile::Item::Sec1Key(key)) => {
                    tokio_rustls::rustls::pki_types::PrivateKeyDer::Sec1(key)
                }
                Some(rustls_pemfile::Item::Pkcs1Key(key)) => {
                    tokio_rustls::rustls::pki_types::PrivateKeyDer::Pkcs1(key)
                }
                Some(rustls_pemfile::Item::Pkcs8Key(key)) => {
                    tokio_rustls::rustls::pki_types::PrivateKeyDer::Pkcs8(key)
                }
                None => break,
                _ => {break}
            };
            return Ok(private_key);
        }
        Err(format!("no keys found in {:?} (encrypted keys not supported)",filename).into())
    }
    fn server_config(&self, cert_file: &str, key_file: &str) -> RResult<Arc<ServerConfig>> {
        let cert = self.load_certs(cert_file)?;
        let private_key = self.load_private_key(key_file)?;
        let server_config = tokio_rustls::rustls::ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(cert.clone(), private_key.clone_key())
                .map_err(|e| {
                    gateway_err!(
                        ConfigurationFailed,
                        "Https bad certificates/private_key Error!",
                        e
                    )
                })?;
        let tls_cfg = Arc::new(server_config);
        Ok(tls_cfg)
    }
}
impl GatewayServerInterface for HttpsServer {
    fn start(&self, gateway: &mut Arc<crate::RockGateway>) -> RResult<()> {
        let addr = self.addr;
        let tls_cfg = self.server_config(&self.cert, &self.key)?;
        let tls_acceptor = TlsAcceptor::from(tls_cfg);
        let gateway_clone = gateway.clone();
        let self_name = self.name().unwrap_or_else(|_|{"https".to_string()});
        gateway.spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await.expect(format!("Error ip_bind failed:{:#?}", addr).as_str());
            log::info!("{}>accept:{:#?}", self_name, addr);
            loop {
                let (stream, remote_addr) = listener.accept().await.expect(format!("Error ip_accept failed:{:#?}", addr).as_str());
                let acceptor = tls_acceptor.clone();
                let tls_stream = match acceptor.accept(stream).await {
                    Ok(stream) => {
                        stream
                    }
                    Err(e) => {
                        log::error!("GatewayError::{:#?}", e);
                        continue;
                    }
                };
                let io: TokioIo<tokio_rustls::server::TlsStream<tokio::net::TcpStream>> = TokioIo::new(tls_stream);
                let gateway_clone = gateway_clone.clone();
                tokio::task::spawn(async move {
                    /* Whenever the browser is closed or not operated for a long time, a result of conn will be returned here. */
                    let conn_ret = hyper::server::conn::http1::Builder::new()
                        .serve_connection(io, hyper::service::service_fn(
                            |request| {
                                https_service::https(remote_addr, request, gateway_clone.clone())
                            }
                        )).await;
                    match conn_ret {
                        Ok(()) => {
                            
                        }
                        Err(e) => {
                            println!("Error https serving connection: {:?}", e);
                        }
                    }
                });
            }
        })
    }
    fn name(&self) -> RResult<String> {
        Ok(crate::util::r#const::HTTPS.to_owned())
    }
}

// pub(crate) struct IOTypeNotSend {
//     _marker: PhantomData<*const ()>,
//     stream: TokioIo<TcpStream>,
// }

// impl IOTypeNotSend {
//     pub(crate) fn new(stream: TokioIo<TcpStream>) -> Self {
//         Self {
//             _marker: PhantomData,
//             stream,
//         }
//     }
// }

// impl hyper::rt::Write for IOTypeNotSend {
//     fn poll_write(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         buf: &[u8],
//     ) -> Poll<Result<usize, std::io::Error>> {
//         Pin::new(&mut self.stream).poll_write(cx, buf)
//     }

//     fn poll_flush(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//     ) -> Poll<Result<(), std::io::Error>> {
//         Pin::new(&mut self.stream).poll_flush(cx)
//     }

//     fn poll_shutdown(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//     ) -> Poll<Result<(), std::io::Error>> {
//         Pin::new(&mut self.stream).poll_shutdown(cx)
//     }
// }

// impl hyper::rt::Read for IOTypeNotSend {
//     fn poll_read(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         buf: hyper::rt::ReadBufCursor<'_>,
//     ) -> Poll<std::io::Result<()>> {
//         Pin::new(&mut self.stream).poll_read(cx, buf)
//     }
// }