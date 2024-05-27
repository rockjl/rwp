/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

pub(crate) mod errors;
pub(crate) mod hosts;
pub(crate) mod routes;
pub(crate) mod service;
pub(crate) mod ratelimiter_builder;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    client::ClientProvider, common::{content_type::ContentTypeAndExtension, file_system::FileSystem, ip_range::IpRange, ratelimiter::RatelimiterCommon, redis::Redis}, error::{
        ConfigError, ConfigErrorKind, GatewayError, RResult
    }, instance::{
        errors::{Err, ErrModule, ErrTypes, Errs, ReturnTypes}, hosts::Hosts, pipes::PipeLine, routes::{
            r#in::In, out::Out, Route
        }, GatewayInstance
    }, modules::{
        balance::{Host, LoadBalanceProfile}, 
        blackandwhitelist::black_and_white_list::{BawFileOrMemory, BlackAndWhiteListInitData, BlackAndWhiteListProfile}, 
        cache::{http_cache_cell::HttpCacheShared, CacheProfile}, 
        dispatche::DispatcheProfile, header::{HeaderActionKey, HeaderProfile}, 
        ratelimiter::{RatelimiterProfile, RatelimiterType}, 
        ModuleType, Modules, PipeData, PipeLineEngine, PipeModule, PipeTask
    }, util::{time_unit::TimeUnit, string_to_number}, RockGateway
};

use self::{errors::{ErrBuilder, ErrType}, hosts::HostsBuilder, ratelimiter_builder::RatelimiterBuilderType, routes::{in_builder::{InBuilder, InType}, out_builder::{OutBuilder, OutType}, RoutesBuilder}, service::ServiceBuilder};

pub(crate) trait Builder<T> {
    fn build(&self, engine: Arc<RockGateway>) -> RResult<T>;
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConfigModel {
    pub(crate) service: ServiceBuilder,
    pub(crate) hosts: HashMap<String, HostsBuilder>,
    pub(crate) pipes: HashMap<String, Vec<config::Map<String, config::Value>>>,
    pub(crate) routes: HashMap<String, RoutesBuilder>,
    pub(crate) errors: Option<HashMap<String, ErrBuilder>>,
}
impl Builder<GatewayInstance> for ConfigModel {
    fn build(&self, engine: std::sync::Arc<RockGateway>) -> crate::error::RResult<GatewayInstance> {
        /*
        initial service
         */
        let mut service = self.service.build(engine.clone())?;
        /*
        initial errors
         */
        let errors = Arc::new(initial_errors(&self.errors)?);
        /*
        initial hosts
         */
        let hosts = initial_home(&self.hosts, engine.get_modules(), &errors)?;
        /*
        initial routes
         */
        let (routes, ins) = initial_routes(&self.routes, &self.pipes, &mut service, engine.get_modules(), &errors)?;

       
        let gateway_instance = GatewayInstance { 
            service,
            hosts,
            routes,
            ins,
            errors,
            bind_cpu: Arc::new(std::sync::Mutex::new(crate::instance::BindCpu::default())),
        };
        Ok(gateway_instance)
    }
}
fn initial_routes(
    routes_builder: &HashMap<String, RoutesBuilder>,
    pipes_setting: &HashMap<String, Vec<config::Map<String, config::Value>>>, 
    service: &mut crate::instance::service::Service,
    modules: Modules,
    errors: &Arc<HashMap<String, Arc<Errs>>>,
) -> crate::error::RResult<(HashMap<String, Route>, Vec<(usize, String, String, In)>)> {
    let mut routes = HashMap::new();
    let mut ins = Vec::new();
    for (key, value) in routes_builder {
        match value.protocol.as_str() {
            "http" => {}
            "https" => {}
            "tcp" => {}
            _ => {
                return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.protocol failed", ConfigError::new(ConfigErrorKind::ROUTEIN)));
            }
        }
        let r#in = match &value.r#in.r#type {
            InType::Regex => {
                let pattern = match &value.r#in.pattern {
                    Some(pattern) => {
                        pattern.clone()
                    }
                    None => { return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.in.pattern failed", ConfigError::new(ConfigErrorKind::ROUTEIN))); }
                };
                In::Regex { pattern: regex::Regex::new(&pattern).unwrap(), method: value.r#in.method.clone() }
            }
            InType::Ip => {
                let ranges = match &value.r#in.ranges {
                    Some(ranges) => {
                        ranges.clone()
                    }
                    None => { return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.in.ranges failed", ConfigError::new(ConfigErrorKind::ROUTEIN))); }
                };
                let baw_type = BlackAndWhiteListInitData { blacklist: BawFileOrMemory::None, whitelist: BawFileOrMemory::Memory(ranges), };
                In::Ip { ranges: BlackAndWhiteListProfile::new(baw_type)?, method: value.r#in.method.clone() }
            }
            InType::FileIp => {
                let file = match &value.r#in.file {
                    Some(file) => {
                        file.clone()
                    }
                    None => { return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.in.file failed", ConfigError::new(ConfigErrorKind::ROUTEIN))); }
                };
                let baw_type = BlackAndWhiteListInitData { blacklist: BawFileOrMemory::None, whitelist: BawFileOrMemory::File(file), };
                In::IpFile { ranges: BlackAndWhiteListProfile::new(baw_type)?, method: value.r#in.method.clone() } 
            }
        };
        let out: Out = match &value.out.r#type {
            OutType::Network => {
                let out_host = match value.out.out_host {
                    Some(ref out_host) => { out_host.clone() }
                    None => { return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.out.out_host failed", ConfigError::new(ConfigErrorKind::ROUTEOUT))); }
                };
                Out::Network { path: value.out.path.clone(), out_host }
            },
            OutType::File => {
                let root_path = match value.out.root_path {
                    Some(ref root_path) => { root_path.clone() }
                    None => { return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.out.root_path failed", ConfigError::new(ConfigErrorKind::ROUTEOUT))); }
                };
                let index_file = match value.out.index_file {
                    Some(ref index_file) => { index_file.clone() }
                    None => { return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.out.index_file failed", ConfigError::new(ConfigErrorKind::ROUTEOUT))); }
                };
                Out::File { file_system: FileSystem::new(&root_path, &index_file)? }
            }
        };
        /* The timeout for listening to incoming data */
        let in_timeout = match &value.in_timeout {
            Some(t) => {
                TimeUnit::parse(t.to_string())
            }
            None => {
                std::time::Duration::from_millis(0)
            }
        };
        /* The buffer size of the router for client reading data */
        let client_buf_size = match &value.client_buf_size {
            Some(t) => {
                t.clone()
            }
            None => {
                crate::util::r#const::BUF_SIZE
            }
        };
        /* The buffer size of the router for reading data from the target address */
        let server_buf_size = match &value.server_buf_size {
            Some(t) => {
                t.clone()
            }
            None => {
                crate::util::r#const::BUF_SIZE
            }
        };
        /* Incorrect settings at the route level */
        let routes_error = if let Some(err_set_name) = &value.error {
            match errors.get(err_set_name) {
                Some(e) => { e.clone() }
                None => { return Err("Parse routes.xx.error_set not found error_set ERROR".into()); }
            }
        } else {
            Arc::new(Errs {
                inner: ErrModule::HTTP(Err {
                    error_list: Vec::new(),
                    pass_next: false,
                    r#return: ReturnTypes::Origin,
                }),
            })
        };
        /* route ratelimiter */
        let route_ratelimiter = if let Some(ratelimiter_builder) = &value.ratelimiter {
            Some(match ratelimiter_builder.r#type {
                RatelimiterBuilderType::Route => {
                    let period = TimeUnit::parse(ratelimiter_builder.period.clone());
                    ratelimiter_builder::structure_ratelimiter(period, ratelimiter_builder.burst.clone(), RatelimiterType::Route(key.clone()))
                }
                RatelimiterBuilderType::Ip => {
                    let period = TimeUnit::parse(ratelimiter_builder.period.clone());
                    ratelimiter_builder::structure_ratelimiter(period, ratelimiter_builder.burst.clone(), RatelimiterType::IpRoute)
                }
                _ => {
                    return Err(gateway_err!(ConfigurationFailed, "Config routes.xx.ratelimiter failed", ConfigError::new(ConfigErrorKind::RATELIMITER)));
                }
            })
        } else {
            None
        };
         /*
        initail pipes
         */
        let (pipe_line, memory_cache_shared) = initial_pipe_line(
            pipes_setting, 
            service, 
            key.clone(),
            value.pipe.clone(), 
            out.clone(), 
            value.protocol.clone(),
            server_buf_size,
            &route_ratelimiter,
            modules.clone(),
        )?;

        let priority = value.priority.unwrap_or_else(|| 1);
        let route = Route {
            protocol: value.protocol.clone(),
            priority,
            in_timeout,
            client_buf_size,
            server_buf_size,
            r#in: r#in.clone(),
            out: out,
            pipe_line,
            memory_cache_shared,
            ratelimiter: route_ratelimiter,
            routes_error,
        };
        ins.push((priority, value.protocol.clone(), key.clone(), r#in));
        routes.insert(key.clone(), route);
    }
    ins.sort_by(|x, y| {
        y.0.cmp(&x.0)
    });
    Ok((routes, ins))
}
fn initial_pipe_line(
    pipes_setting: &HashMap<String, Vec<config::Map<String, config::Value>>>, 
    service: &mut crate::instance::service::Service, 
    route_name: String,
    pipe_builder_name: String,
    out: Out,
    protocol: String,
    server_buf_size: usize,
    route_ratelimiter: 
        &Option<std::sync::Arc<
                RatelimiterCommon
        >>,
    modules: Modules,
) -> crate::error::RResult<(PipeLine, Arc<HttpCacheShared>)> {
    for (pipe_name, pipes) in pipes_setting {
        if pipe_builder_name.eq(pipe_name) {
            let memory_cache = Arc::new(HttpCacheShared{
                cache: std::sync::Mutex::new(HashMap::new()),
            });
            let mut cur_pipe_task: Option<Box<PipeTask>> = None;
            let mut task_ref: Option<&mut PipeTask> = None;
            for pipe_b in pipes {
                let p_t = initial_pipe_module(
                    route_name.clone(),
                    pipe_name.clone(),
                    out.clone(), 
                    protocol.clone(),
                    server_buf_size, 
                    pipe_b, 
                    service, 
                    memory_cache.clone(),
                    route_ratelimiter,
                    modules.clone(),
                )?;
                if let Some(t_r) = task_ref {
                    t_r.next_task = Some(p_t);
                    task_ref = Some(t_r.next_task.as_mut().unwrap());
                } else {
                    cur_pipe_task = Some(p_t);
                    task_ref = Some(cur_pipe_task.as_mut().unwrap());
                }
            }
            if let None = cur_pipe_task {
                return Err(gateway_err!(ConfigurationFailed, format!("Failed to parse pipes ERROR > not found pipe_module. pipe_line:{:#?}", pipe_builder_name.clone()).as_str(), ConfigError::new(ConfigErrorKind::PIPES)));            
            }
            let pipe_task = cur_pipe_task.expect("System Error cur_pipe_task");
            let pipe_line_engine = PipeLineEngine {
                task: pipe_task,
                module_scheduling: modules.clone(),
            };
            return Ok((PipeLine { id: Uuid::new_v4(), pipe_name: pipe_name.clone(), pipe_line_engine, }, memory_cache,));
        }
    }
    return Err(gateway_err!(ConfigurationFailed, format!("Failed to parse pipes ERROR > not found pipe_line:{:#?}", pipe_builder_name.clone()).as_str(), ConfigError::new(ConfigErrorKind::PIPES)));
}
fn initial_pipe_module(
    route_name: String,
    pipe_name: String,
    out: Out,
    protocol: String,
    server_buf_size: usize,
    pipe_b: &config::Map<String, config::Value>, 
    service: &mut crate::instance::service::Service,
    memory_cache: Arc<HttpCacheShared>,
    route_ratelimiter: 
        &Option<std::sync::Arc<
                RatelimiterCommon
        >>,
    modules: Modules,
) -> crate::error::RResult<Box<PipeTask>> {
    if pipe_b.contains_key(&crate::util::r#const::BLACK_AND_WHITE_LIST.to_string()) {
        match pipe_b.get(&crate::util::r#const::BLACK_AND_WHITE_LIST.to_string()) {
            Some(v) => {
                if let config::ValueKind::Table(m0) = &v.kind {
                    let bl_fam = if let Some(black_list_org) = m0.get("black_list") {
                        if let config::ValueKind::Table(blo) = &black_list_org.kind {
                            if let Some(blo_m) = blo.get("memory") {
                                if let config::ValueKind::Array(b_m) = &blo_m.kind {
                                    let mut ret = Vec::new();
                                    for v0 in b_m {
                                        let ip_str = if let config::ValueKind::String(ip) = &v0.kind {
                                            ip.clone()
                                        } else {
                                            return Err(gateway_err!(ConfigurationFailed, "ERROR black_list.memory.xx set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));        
                                        };
                                        ret.push(ip_str);
                                    }
                                    BawFileOrMemory::Memory(ret)
                                } else {
                                    return Err(gateway_err!(ConfigurationFailed, "ERROR black_list.memory set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));        
                                }
                            } else if let Some(blo_r) = blo.get("file") {
                                let file_name = if let config::ValueKind::String(file_name) = &blo_r.kind {
                                    file_name.clone()
                                } else {
                                    return Err(gateway_err!(ConfigurationFailed, "ERROR black_list.file.xx set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));        
                                };
                                BawFileOrMemory::File(file_name)
                            } else {
                                BawFileOrMemory::None
                            }
                        } else {
                            return Err(gateway_err!(ConfigurationFailed, "ERROR black_list set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
                        }
                    } else {
                        BawFileOrMemory::None
                    };
                    let wl_fam = if let Some(white_list_org) = m0.get("white_list") {
                        if let config::ValueKind::Table(wlo) = &white_list_org.kind {
                            if let Some(wlo_m) = wlo.get("memory") {
                                if let config::ValueKind::Array(w_m) = &wlo_m.kind {
                                    let mut ret = Vec::new();
                                    for v0 in w_m {
                                        let ip_str = if let config::ValueKind::String(ip) = &v0.kind {
                                            ip.clone()
                                        } else {
                                            return Err(gateway_err!(ConfigurationFailed, "ERROR white_list.memory.xx set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));        
                                        };
                                        ret.push(ip_str);
                                    }
                                    BawFileOrMemory::Memory(ret)
                                } else {
                                    return Err(gateway_err!(ConfigurationFailed, "ERROR white_list.memory set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));        
                                }
                            } else if let Some(wlo_r) = wlo.get("file") {
                                let file_name = if let config::ValueKind::String(file_name) = &wlo_r.kind {
                                    file_name.clone()
                                } else {
                                    return Err(gateway_err!(ConfigurationFailed, "ERROR white_list.file.xx set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));        
                                };
                                BawFileOrMemory::File(file_name)
                            } else {
                                BawFileOrMemory::None
                            }
                        } else {
                            return Err(gateway_err!(ConfigurationFailed, "ERROR white_list set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
                        }
                    } else {
                        BawFileOrMemory::None  
                    };
                    let baw_init_data = BlackAndWhiteListInitData { blacklist: bl_fam, whitelist: wl_fam };
                    let profile = BlackAndWhiteListProfile::new(baw_init_data)?;
                    
                    return Ok(modules.make_pipe_task(ModuleType::BlackAndWhiteList, PipeData::BlackAndWhiteListData { profile: tokio::sync::RwLock::new(profile) }));
                } else {
                    return Err(gateway_err!(ConfigurationFailed, "ERROR black and white list set failed", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
                }
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "ERROR not found black and white list", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::RATELIMITER.to_string()) {
        match pipe_b.get(&crate::util::r#const::RATELIMITER.to_string()) {
            Some(v) => {
                if let config::ValueKind::Table(ratelimiter_sets) = &v.kind {
                    let ratelimiter_str = match ratelimiter_sets.get("type") {
                        Some(rs) => {
                            rs.to_string()
                        }
                        None => { return Err(gateway_err!(ConfigurationFailed, "ERROR ratelimiter >type< not found", ConfigError::new(ConfigErrorKind::CACHE))); }
                    };
                    let lim = match ratelimiter_str.as_str() {
                        "service" => {
                            if let Some(service_lim) = &service.ratelimiter {
                                service_lim.clone()
                            } else {
                                return Err(gateway_err!(ConfigurationFailed, "ERROR not found service.ratelimiter settings.", ConfigError::new(ConfigErrorKind::CACHE)));
                            }
                        }
                        "route" => {
                            if let Some(route_lim) = &route_ratelimiter {
                                route_lim.clone()
                            } else {
                                return Err(gateway_err!(ConfigurationFailed, "ERROR not found routes.xx.ratelimiter settings.", ConfigError::new(ConfigErrorKind::CACHE)));
                            }
                        }
                        _ => {
                            return Err(gateway_err!(ConfigurationFailed, "ERROR pipes.xx.ratelimiter.type set error.", ConfigError::new(ConfigErrorKind::CACHE)));
                        }
                    };
                    let profile = RatelimiterProfile { 
                        ratelimiter: lim 
                    };
                    return Ok(modules.make_pipe_task(ModuleType::RateLimiter, PipeData::RatelimiterModuleData { profile: tokio::sync::RwLock::new(profile) }));
                } else {
                    return Err(gateway_err!(ConfigurationFailed, "ERROR ratelimiter set_error", ConfigError::new(ConfigErrorKind::CACHE)));
                }
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "ERROR not found ratelimiter", ConfigError::new(ConfigErrorKind::CACHE)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::MEMORY_CACHE_GET.to_string()) {
        if !service.has_cache("memory") {
            return Err(gateway_err!(ConfigurationFailed, "ERROR not found service.cache.memory.", ConfigError::new(ConfigErrorKind::CACHE)));
        }
        match pipe_b.get(&crate::util::r#const::MEMORY_CACHE_GET.to_string()) {
            Some(v) => {
                if let config::ValueKind::Table(mcg) = &v.kind {
                    let expire = TimeUnit::parse(match mcg.get("expire") {
                        Some(expire_value) => {
                            expire_value.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    });
                    let hit_str = match mcg.get("hit") {
                        Some(hit_str) => {
                            hit_str.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    };
                    let hit = if hit_str == "" { -1 } else { hit_str.parse::<i32>()? };
                    let back_str = match mcg.get("back") {
                        Some(back_str) => {
                            back_str.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    };
                    let back = if back_str.is_empty() { false } else { back_str.parse().unwrap() };
                    let profile = CacheProfile::new(memory_cache.clone(), expire, hit, None, back);
                    return Ok(modules.make_pipe_task(ModuleType::MemoryGet, PipeData::MemoryCacheGetData { profile: tokio::sync::RwLock::new(profile) }));
                } else {
                    let profile = CacheProfile::new(memory_cache.clone(), std::time::Duration::from_millis(0), -1, None, false);
                    return Ok(modules.make_pipe_task(ModuleType::MemoryGet, PipeData::MemoryCacheGetData { profile: tokio::sync::RwLock::new(profile) }));
                }
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "ERROR not found service.cache.memory_cache_get", ConfigError::new(ConfigErrorKind::CACHE)));
            }
        }    
    } else if pipe_b.contains_key(&crate::util::r#const::MEMORY_CACHE_SET.to_string()) {
        if !service.has_cache("memory") {
            return Err(gateway_err!(ConfigurationFailed, "ERROR not found service.cache.memory.", ConfigError::new(ConfigErrorKind::CACHE)));
        }
        match pipe_b.get(&crate::util::r#const::MEMORY_CACHE_SET.to_string()) {
            Some(v) => {
                if let config::ValueKind::Table(mcg) = &v.kind {
                    let expire = TimeUnit::parse(match mcg.get("expire") {
                        Some(expire_value) => {
                            expire_value.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    });
                    let hit_str = match mcg.get("hit") {
                        Some(hit_str) => {
                            hit_str.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    };
                    let hit = if hit_str == "" { -1 } else { hit_str.parse::<i32>()? };
                    let profile = CacheProfile::new(memory_cache.clone(), expire, hit, None, false);
                    return Ok(modules.make_pipe_task(ModuleType::MemorySet, PipeData::MemoryCacheSetData { profile: tokio::sync::RwLock::new(profile) }));
                } else {
                    let profile = CacheProfile::new(memory_cache.clone(), std::time::Duration::from_millis(0), -1, None, false);
                    return Ok(modules.make_pipe_task(ModuleType::MemorySet, PipeData::MemoryCacheSetData { profile: tokio::sync::RwLock::new(profile) }));
                }
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.memory_cache_set ERROR", ConfigError::new(ConfigErrorKind::PIPES)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::REDIS_CACHE_GET.to_string()) {
        if !service.has_cache("redis") {
            return Err(gateway_err!(ConfigurationFailed, "ERROR not found service.cache.redis.", ConfigError::new(ConfigErrorKind::CACHE)));
        }
        match pipe_b.get(&crate::util::r#const::REDIS_CACHE_GET.to_string()) {
            Some(v) => {
                if let config::ValueKind::Table(rcg) = &v.kind {
                    let expire = TimeUnit::parse(match rcg.get("expire") {
                        Some(expire_value) => {
                            expire_value.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    });
                    let hit_str = match rcg.get("hit") {
                        Some(hit_str) => {
                            hit_str.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    };
                    let hit = if hit_str == "" { -1 } else { hit_str.parse::<i32>()? };
                    let redis = service.crate_redis("redis_prefix_".to_string())?;
                    let back_str = match rcg.get("back") {
                        Some(back_str) => {
                            back_str.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    };
                    let back = if back_str.is_empty() { false } else { back_str.parse().unwrap() };
                    let profile = CacheProfile::new(memory_cache.clone(), expire, hit, Some(redis), back);
                    return Ok(modules.make_pipe_task(ModuleType::RedisGet, PipeData::RedisCacheGetData { profile: tokio::sync::RwLock::new(profile) }));
                } else {
                    let redis = service.crate_redis("redis_prefix_".to_string())?;
                    let profile = CacheProfile::new(memory_cache.clone(), std::time::Duration::from_millis(0), -1, Some(redis), false);
                    return Ok(modules.make_pipe_task(ModuleType::RedisGet, PipeData::RedisCacheGetData { profile: tokio::sync::RwLock::new(profile) }));
                }
            },
            None => {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.redis_cache_get ERROR", ConfigError::new(ConfigErrorKind::PIPES)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::REDIS_CACHE_SET.to_string()) {
        if !service.has_cache("redis") {
            return Err(gateway_err!(ConfigurationFailed, "ERROR not found service.cache.redis.", ConfigError::new(ConfigErrorKind::CACHE)));
        }
        match pipe_b.get(&crate::util::r#const::REDIS_CACHE_SET.to_string()) {
            Some(v) => {
                if let config::ValueKind::Table(rcg) = &v.kind {
                    let expire = TimeUnit::parse(match rcg.get("expire") {
                        Some(expire_value) => {
                            expire_value.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    });
                    let hit_str = match rcg.get("hit") {
                        Some(hit_str) => {
                            hit_str.to_string()
                        }
                        None => {
                            "".to_string()
                        }
                    };
                    let hit = if hit_str == "" { -1 } else { hit_str.parse::<i32>()? };
                    let redis = service.crate_redis(crate::util::r#const::REDIS_PREFIX.to_string())?;
                    let profile = CacheProfile::new(memory_cache.clone(), expire, hit, Some(redis), false);
                    return Ok(modules.make_pipe_task(ModuleType::RedisSet, PipeData::RedisCacheSetData { profile: tokio::sync::RwLock::new(profile) }));
                } else {
                    let redis = service.crate_redis(crate::util::r#const::REDIS_PREFIX.to_string())?;
                    let profile = CacheProfile::new(memory_cache.clone(), std::time::Duration::from_millis(0), -1, Some(redis), false);
                    return Ok(modules.make_pipe_task(ModuleType::RedisSet, PipeData::RedisCacheSetData { profile: tokio::sync::RwLock::new(profile) }));
                }
            },
            None => {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.redis_cache_set ERROR", ConfigError::new(ConfigErrorKind::PIPES)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::HEADER_REQUEST.to_string()) {
        match pipe_b.get(&crate::util::r#const::HEADER_REQUEST.to_string()) {
            Some(header_request) => {
                let map = parse_header_profile(header_request)?;
                return Ok(modules.make_pipe_task(ModuleType::HeaderRequest, PipeData::HeaderRequestData { profile: tokio::sync::RwLock::new(HeaderProfile { action: map }) }));
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.header_request ERROR", ConfigError::new(ConfigErrorKind::PIPESHEADER)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::HEADER_RESPONSE.to_string()) {
        match pipe_b.get(&crate::util::r#const::HEADER_RESPONSE.to_string()) {
            Some(header_response) => {
                let map = parse_header_profile(header_response)?;
                return Ok(modules.make_pipe_task(ModuleType::HeaderResponse, PipeData::HeaderResponseData { profile: tokio::sync::RwLock::new(HeaderProfile { action: map }) }));
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.header_response ERROR", ConfigError::new(ConfigErrorKind::PIPESHEADER)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::DISPATCHE.to_string()) {
        match pipe_b.get(&crate::util::r#const::DISPATCHE.to_string()) {
            Some(dispatch) => {
                let client = ClientProvider::new(protocol.as_str(), server_buf_size)?;
                return parse_dispatche(out, client, dispatch, modules);
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.dispatche ERROR", ConfigError::new(ConfigErrorKind::PIPESDISPATCHE)));
            }
        }
    } else if pipe_b.contains_key(&crate::util::r#const::RETURN.to_string()) {
        match pipe_b.get(&crate::util::r#const::RETURN.to_string()) {
            Some(_) => {
                return Ok(modules.make_pipe_task(ModuleType::Return, PipeData::ReturnModuleData { profile: tokio::sync::RwLock::new(()) }));
            }
            None => {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.dispatche ERROR", ConfigError::new(ConfigErrorKind::PIPESDISPATCHE)));
            }
        }
    } 
    else {
        return Err(gateway_err!(ConfigurationFailed, format!("Failed to parse pipes ERROR > not found pipes:{:#?}", pipe_b).as_str(), ConfigError::new(ConfigErrorKind::PIPES)));
    }
}
fn parse_dispatche(
    out: Out,
    client: ClientProvider,
    _: &config::Value, 
    modules: Modules,
) -> RResult<Box<PipeTask>> {
    match out {
        Out::File { file_system } => {
            return Ok(modules.make_pipe_task(ModuleType::DispatchFile, PipeData::FileServerDispatcheData { profile: tokio::sync::RwLock::new(DispatcheProfile::File { file_system, content_type: ContentTypeAndExtension::new(), }) }));
        },
        Out::Network { path, out_host } => {
            return Ok(modules.make_pipe_task(ModuleType::DispatchNetwork, PipeData::NetworkDispatcheData { profile: tokio::sync::RwLock::new(DispatcheProfile::Network { path, out_host, client }) }));
        }
    }
}
fn parse_header_profile(headers_document: &config::Value) -> RResult<HashMap<HeaderActionKey, HeaderMap>> {
    let mut map = HashMap::new();
    if let config::ValueKind::Table(headers_config) = &headers_document.kind {
        if let Some(add_arr_v) = headers_config.get(crate::util::r#const::ADD) {
            if let config::ValueKind::Array(add_arr) = &add_arr_v.kind {
                let mut header_map = HeaderMap::new();
                for header_line in add_arr {
                    if let config::ValueKind::Table(hm) = &header_line.kind {
                        for (k, v) in hm {
                            let header_name_str = k.clone();
                            let header_value_str = v.to_string();
                            let header_name = parse_header_name(header_name_str)?;
                            let header_value = parse_header_value(header_value_str)?;
                            if header_map.contains_key(header_name.clone()) {
                                header_map.append(header_name, header_value);
                            } else {
                                header_map.insert(header_name, header_value);
                            }
                        }
                    } else {
                        return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.headers_request or pipes.hreaders_response add ERROR", ConfigError::new(ConfigErrorKind::PIPESHEADER)));                
                    }
                }
                map.insert(HeaderActionKey::ADD, header_map);
            } else {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.headers_request or pipes.hreaders_response add ERROR", ConfigError::new(ConfigErrorKind::PIPESHEADER)));        
            }
        }

        if let Some(add_arr_v) = headers_config.get(crate::util::r#const::DEL) {
            if let config::ValueKind::Array(add_arr) = &add_arr_v.kind {
                let mut header_map = HeaderMap::new();
                for header_line in add_arr {
                    if let config::ValueKind::String(hv) = &header_line.kind {
                        let header_name_str = hv.clone();
                        let header_name = parse_header_name(header_name_str)?;
                        let header_value = parse_header_value("".to_string())?;
                        if header_map.contains_key(header_name.clone()) {
                            header_map.append(header_name, header_value);
                        } else {
                            header_map.insert(header_name, header_value);
                        }
                    } else {
                        return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.headers_request or pipes.hreaders_response add ERROR", ConfigError::new(ConfigErrorKind::PIPESHEADER)));                
                    }
                }
                map.insert(HeaderActionKey::DEL, header_map);
            } else {
                return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.headers_request or pipes.hreaders_response add ERROR", ConfigError::new(ConfigErrorKind::PIPESHEADER)));        
            }
        }
    } else {
        return Err(gateway_err!(ConfigurationFailed, "Failed to parse pipes.headers_request or pipes.hreaders_response add ERROR", ConfigError::new(ConfigErrorKind::PIPESHEADER)));
    }
    Ok(map)
}
fn parse_header_name(name: String) -> RResult<HeaderName> {
    match http::header::HeaderName::try_from(name.clone()) {
        Ok(header_name) => {
            Ok(header_name)
        }
        Err(e) => {
            return Err(gateway_err!(ConfigurationFailed, format!("Failed to parse header.header_name:{:?}", &name).as_str(), e));
        }
    }
}
fn parse_header_value(value: String) -> RResult<HeaderValue> {
    match http::header::HeaderValue::try_from(value.clone()) {
        Ok(header_value) => {
            Ok(header_value)
        }
        Err(e) => {
            return Err(gateway_err!(ConfigurationFailed, format!("Failed to parse header.header_value:{:?}", &value).as_str(), e));
        }
    }
}
fn parse_host_vec(hwmf: Vec<&str>, errors: &Arc<HashMap<String, Arc<Errs>>>,) -> RResult<(String, u16, Option<u32>, Option<u32>, Option<std::time::Duration>, Option<std::time::Duration>, std::time::Duration)> {
    let mut host = String::new();
    let mut port = 0;
    let mut weight = None;
    let mut max_fails = None;
    let mut fail_timeout = None;
    let mut timeout = None;
    let mut max_fails_durn = crate::util::r#const::HOST_POINT_MAX_FAILS_DURN;
    for item in hwmf {
        if item.contains("weight=") {
            weight = Some(item.replace("weight=", "").parse::<u32>().map_err(|e| {
                return gateway_err!(ConfigurationFailed, "Failed to parse weight ERROR", e);
            })?);
            continue;
        }
        if item.contains("max_fails_durn=") {
            max_fails_durn = TimeUnit::parse(item.replace("max_fails_durn=", ""));
            continue;
        }
        if item.contains("max_fails=") {
            max_fails = Some(item.replace("max_fails=", "").parse::<u32>().map_err(|e| {
                return gateway_err!(ConfigurationFailed, "Failed to parse max_fails ERROR", e);
            })?);
            continue;
        }
        if item.contains("fail_timeout=") {
            fail_timeout = Some(TimeUnit::parse(item.replace("fail_timeout=", "")));
            continue;
        }
        if item.contains("timeout=") {
            timeout = Some(TimeUnit::parse(item.replace("timeout=", "")));
            continue;
        }
        let host_port = item.split(":").collect::<Vec<&str>>();
        let new_host = match host_port.len() {
            1 => {
                host = host_port.get(0).unwrap().to_string();
                port = 80;
            }
            2 => {
                host = host_port.get(0).unwrap().to_string();
                port = host_port.get(1).unwrap().parse::<u16>().map_err(|e| {
                    return gateway_err!(ConfigurationFailed, "Failed to parse host_port ERROR", e);
                })?;
            }
            _ => {
                return Err("Parse host_port ERROR".into());
            }
        };
    }
    Ok((host, port, weight, max_fails, fail_timeout, timeout, max_fails_durn))
}
fn initial_errors(errors_setting: &Option<HashMap<String, ErrBuilder>>) -> crate::error::RResult<HashMap<String, Arc<Errs>>> {
    let mut err_ret = HashMap::new();
    if let Some(errors_map) = errors_setting {
        for (key, err_builder) in errors_map {
            let errors = err_builder.error_list.clone().unwrap_or_else(|| "".to_string());
            let pass_next = err_builder.pass_next.clone().unwrap_or_else(|| false);
            let return_str = err_builder.r#return.clone().unwrap_or_else(|| "".to_string());
            let r#return = if &return_str == "" {
                ReturnTypes::Origin
            } else if &return_str == "origin" {
                ReturnTypes::Origin
            } else {
                ReturnTypes::Hsc(parse_hsc(&return_str)?)
            };
            let module_type = &err_builder.r#type;
            let error_type = match module_type {
                ErrType::Http => {
                    let errors = errors.split_whitespace().collect::<Vec<&str>>();
                    let err_v = parse_errors(errors)?;
                    let err = Err {
                        error_list: err_v,
                        pass_next,
                        r#return,
                    };
                    ErrModule::HTTP(err)
                }
                ErrType::Tcp => {
                    ErrModule::TCP
                }
            };
            err_ret.insert(key.clone(), Arc::new(Errs { inner: error_type }));
        }
    }
    Ok(err_ret)
}
fn parse_errors(errors: Vec<&str>) -> RResult<Vec<ErrTypes>> {
    let mut ret = Vec::new();
    for e_str in errors {
        let e = match e_str {
            sc if sc.starts_with("hsc_") => {
                let status_code = parse_hsc(e_str)?;
                ErrTypes::Hsc(status_code)
            }
            _ => { return Err(format!("Parse errors ERROR:{{{:?}}}", e_str).into()); }
        };
        ret.push(e);
    }
    Ok(ret)
}
fn parse_hsc(str: &str) -> RResult<StatusCode> {
    if !str.starts_with("hsc_") { return Err(format!("Parse error_list ERROR:{{{:?}}}", str).into()) }
    let nsc = match str.replace("hsc_", "").parse::<u16>(){
        Ok(nsc) => { nsc }
        Err(_) => { return Err(format!("Parse error_list parse::<u16> ERROR:{{{:?}}}", str).into()); }
    };
    let status_code = match http::StatusCode::from_u16(nsc) {
        Ok(status_code) => { status_code }
        Err(_) => { return Err(format!("Parse error_list from_u16 ERROR:{{{:?}}}", str).into()); }
    };
    Ok(status_code)
}
fn initial_home(
    hosts_setting: &HashMap<String, HostsBuilder>, 
    modules: Modules,
    errors: &Arc<HashMap<String, Arc<Errs>>>,
) -> crate::error::RResult<HashMap<String, Hosts>> {
    let mut hosts = HashMap::new();
    for (key, hosts_builder) in hosts_setting {
        let hosts_vec: Vec<String> = hosts_builder.servers.clone();
        let error_name = hosts_builder.error.clone().unwrap_or_else(|| "".to_string());
        let mut value_arg = HashMap::new();
        let mut index = 0;
        for new_host in hosts_vec {
            let hp_w_m_ft = new_host.split_whitespace().collect::<Vec<&str>>();
            let (host, port , weight, max_fails, fail_timeout, timeout, max_fails_durn) = parse_host_vec(hp_w_m_ft, errors)?;
            let host_item = Host {
                host: Arc::new(host), 
                port: Arc::new(port), 
                weight, 
                cur_weight:1, 
                max_fails, 
                fail_timeout, 
                timeout, 
                max_fails_durn: crate::common::max_fails_durn::structure_max_fails_durn(max_fails_durn, max_fails.unwrap_or_else(|| 1)), 
            };
            value_arg.insert(index, host_item);
            index = index + 1;
        }
        let value_arg = Arc::new(tokio::sync::RwLock::new(value_arg));
        let hosts_error = match errors.get(&error_name) {
            Some(e) => { Some(e.clone()) }
            None => { None }
        };
        let module_name = hosts_builder.r#type.clone();
        let pipe_data = match module_name.as_str() {
            "ip_round_robin" => {
                PipeData::IpRoundRobinBalancerData { profile: tokio::sync::RwLock::new(LoadBalanceProfile { 
                    hosts: value_arg, 
                    permanent_failure: Arc::new(tokio::sync::RwLock::new(HashMap::new())), 
                    host_index: 0, 
                    ip_matchs: HashMap::new(), 
                    hosts_error: hosts_error.clone() 
                }) }
            }
            "random" => {
                PipeData::RandomBalancerData { profile: tokio::sync::RwLock::new(LoadBalanceProfile { 
                    hosts: value_arg, 
                    permanent_failure: Arc::new(tokio::sync::RwLock::new(HashMap::new())), 
                    host_index: 0, 
                    ip_matchs: HashMap::new(), 
                    hosts_error: hosts_error.clone() 
                }) }
            }
            "round_robin" => {
                PipeData::RoundRobinBalancerData { profile: tokio::sync::RwLock::new(LoadBalanceProfile { 
                    hosts: value_arg, 
                    permanent_failure: Arc::new(tokio::sync::RwLock::new(HashMap::new())), 
                    host_index: 0, 
                    ip_matchs: HashMap::new(), 
                    hosts_error: hosts_error.clone() 
                }) }
            }
            _ => { return Err("Parse host pipe_data ERROR".into()); }
        };
        let lb_task = modules.make_pipe_task(ModuleType::from(module_name.as_str()), pipe_data);
        hosts.insert(key.clone(), Hosts {
            hosts_error,
            lb_task,
            modules: modules.clone(),
        });
    }
    Ok(hosts)
}



