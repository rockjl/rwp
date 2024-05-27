/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub mod error;
mod result;
mod error_from;

pub use result::*;
pub use error::*;

use derive_more::Display;

pub type RResult<T> = std::result::Result<T, GatewayError>;

pub trait SourceError: std::error::Error + std::fmt::Display + Send + Sync + 'static {}
impl<T: std::error::Error + std::fmt::Display + Send + Sync + 'static> SourceError for T {}

type Source = Option<Box<dyn SourceError + 'static>>;

fn add_source(source: &Source) -> String {
    if let Some(s) = source {
        format!("\nCaused by: {}", s)
    } else {
        "".into()
    }
}

#[derive(Debug, Display)]
pub enum GatewayError {
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    ConfigurationFailed {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    DirectlyReturn {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    CacheHit {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    RedisFailed {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    BuilderFailed {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    RatelimiterArrival {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    BlackAndWhiteListError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    NotFoundRouteError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    ParseRequestError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    PipeExecuteError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    PipeExecuteTimeoutError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    NoAvailableHostsError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    TcpSenderError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message,"add_source(source)")]
    HyperError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    Critical {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] Component {}: {}{}",module_path, line,col,name,message,"add_source(source)"
    )]
    RequiredComponent {
        #[doc(hidden)]
        name: String,
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    IoError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    Other {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
}
impl std::error::Error for GatewayError {}