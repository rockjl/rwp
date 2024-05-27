/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct BuilderError {
    kind: BuilderErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BuilderErrorKind {
    BALANCE,        //Exception in constructing load_balance
    INTERFACES,     //Exception in constructing listening interface
    MEMORYCACHE,   //Construct memory cache exception
    HEADER,         //Exception in constructing header modifier
    DISPATCHE,      //Construct a dispatch exception
    CLIENT,         //Constructing client exceptions
    RETURN,         //Construct return exception
    UPGRADE,        //Constructing HTTP UPGRADE exception
    BAWLIST,       //Abnormal black and white list construction
}
impl std::error::Error for BuilderError {}
impl BuilderError {
    fn to_str(&self) -> &str {
        match self.kind {
            BuilderErrorKind::BALANCE => "cannot build load balance",
            BuilderErrorKind::INTERFACES => "cannot build interfaces",
            BuilderErrorKind::MEMORYCACHE => "cannot build memory_cache",
            BuilderErrorKind::HEADER => "cannot build header",
            BuilderErrorKind::DISPATCHE => "cannot build dispatche",
            BuilderErrorKind::CLIENT => "cannot build client redirect",
            BuilderErrorKind::RETURN => "cannot build return",
            BuilderErrorKind::UPGRADE => "cannot build http upgrade",
            BuilderErrorKind::BAWLIST => "cannot build black an white list",
        }
    }
    pub(crate) fn new(kind: BuilderErrorKind) -> Self {
        Self { kind }
    }
}
impl std::fmt::Debug for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[warn(deprecated)]
        self.to_str().fmt(f)
    }
}
impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_str().fmt(f)
    }
}

//--------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
pub struct ConfigError {
    kind: ConfigErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigErrorKind {
    CACHE,              //Cache - Configuration Error
    PIPES,              //Channel - Configuration Error
    PIPESMEMORY,        //Channel - Cache Configuration Error
    PIPESHEADER,        //Channel - Header Configuration Error
    PIPESDISPATCHE,     //Channel - Dispatche Configuration Error
    ROUTETCPIPRANGE,    //Routing - TCP IP range Configuration Error
    RATELIMITER,        //Ratelimiter - Configuration Error
    BAWTCPIP,           //Black and white list - Ip Configuration Error
    BLACKANDWHITE,      //Black and white list - Configuration Error
    ROUTEIN,            //Gateway In - Configuration Error
    ROUTEOUT,           //Gateway out - Configuration Error
    ROUTEPROTOCOL,      //Gateway protocol - Configuration Error
    TOKIO,              //Tokio Settings - Configuration Error
}
impl std::error::Error for ConfigError {}
impl ConfigError {
    fn to_str(&self) -> &str {
        match self.kind {
            ConfigErrorKind::CACHE => "error set service.cache",
            ConfigErrorKind::PIPES => "error set pipes",
            ConfigErrorKind::PIPESMEMORY => "error set pipes.memory",
            ConfigErrorKind::PIPESHEADER => "error set pipes.header",
            ConfigErrorKind::PIPESDISPATCHE => "error set pipes.dispatche",
            ConfigErrorKind::ROUTETCPIPRANGE => "error set routes.xx.in.pattern",
            ConfigErrorKind::RATELIMITER => "error set ??.xx.ratelimiter",
            ConfigErrorKind::BAWTCPIP => "error set black and white list tcp ip",
            ConfigErrorKind::BLACKANDWHITE => "error black and white list",
            ConfigErrorKind::ROUTEIN => "error routes.xx.in",
            ConfigErrorKind::ROUTEOUT => "error routes.xx.out",
            ConfigErrorKind::ROUTEPROTOCOL => "error routes.xx.protocol",
            ConfigErrorKind::TOKIO => "error service.multi_thread/current_thread  .bind_cpu",
        }
    }
    pub fn new(kind: ConfigErrorKind) -> Self {
        Self { kind }
    }
}
impl std::fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[warn(deprecated)]
        self.to_str().fmt(f)
    }
}
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_str().fmt(f)
    }
}

//----------------------------------------------
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct PipeError {
    kind: PipeErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PipeErrorKind {
    DISPATCHE,              //dispatche exceptio
    RETURN,                 //return exception
    ROUTE,                  //route exception
    NOTFOUNDROUTE,         //not found route
    DISPATCHETCPSENDER,   //TCP sender exception dispatche
    TCPSENDERTIMEOUT,     //TCP send timeout
    HTTPSENDERTIMEOUT,    //HTTP send timeout
    REDIS,                  //Redis cache excepton
    RATELIMITER,            //ratelimiter
    CACHEHIT,
    DIRECTLYRETURN,
    NOAVAILABLEHOSTS,      //No available hosts
}
impl std::error::Error for PipeError {}
impl PipeError {
    fn to_str(&self) -> &str {
        match self.kind {
            PipeErrorKind::DISPATCHE => "error dispatche",
            PipeErrorKind::RETURN => "error return",
            PipeErrorKind::ROUTE => "error route",
            PipeErrorKind::NOTFOUNDROUTE => "error not found route",
            PipeErrorKind::DISPATCHETCPSENDER => "error dispatche tcp sender",
            PipeErrorKind::TCPSENDERTIMEOUT => "error tcp sender timeout",
            PipeErrorKind::HTTPSENDERTIMEOUT => "error http sender timeout",
            PipeErrorKind::REDIS => "error redis",
            PipeErrorKind::RATELIMITER => "error ratelimiter",
            PipeErrorKind::CACHEHIT => "error cache hit",
            PipeErrorKind::DIRECTLYRETURN => "directly return",
            PipeErrorKind::NOAVAILABLEHOSTS => "No available hosts",
        }
    }
    pub(crate) fn new(kind: PipeErrorKind) -> Self {
        Self { kind }
    }
}
impl std::fmt::Debug for PipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[warn(deprecated)]
        self.to_str().fmt(f)
    }
}
impl std::fmt::Display for PipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_str().fmt(f)
    }
}