/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{borrow::Borrow, collections::HashMap, sync::Arc };

use crate::{
    config::parsers::{
        self, model::Builder, 
    }, context::GatewayContext, instance::{
        service::{AddressInterface, CacheType, TokioBindCpuType, TokioSettings, TokioType}, GatewayInstance
    }, modules::{cache::http_cache_cell::HttpCacheKey, upgrade::UpgradeProfile, CommonModule, ModuleType, Modules, PipeTask}, servers::GatewayServer 
};
use crate::error::{RResult, GatewayError};

pub struct RockGateway {
    gateway_instance: std::sync::RwLock<Arc<GatewayInstance>>,
    servers: std::sync::RwLock<HashMap<String, GatewayServer>>,
    // runtime: crate::util::j_unsafecell::JUnsafeCell<Option<std::sync::RwLock<tokio::runtime::Runtime>>>,
    // runtime: RefCell<Option<std::sync::RwLock<tokio::runtime::Runtime>>>,
    runtime: std::sync::RwLock<tokio::runtime::Runtime>,
    // modules: std::sync::RwLock<HashMap<&'static str, Box<dyn Module>>>,
    pub(crate) common_module: tokio::sync::RwLock<Option<CommonModule>>,
    pub(crate) modules: Modules,
}
impl std::fmt::Debug for RockGateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RockGateway {{ instance: {:?} }}", self.gateway_instance)
    }
}
impl Default for RockGateway {
    fn default() -> Self {
        Self {
            gateway_instance: std::sync::RwLock::default(),
            servers: std::sync::RwLock::default(),
            // runtime: crate::util::j_unsafecell::JUnsafeCell::new(None),
            // runtime: RefCell::new(None),
            runtime: std::sync::RwLock::new(tokio::runtime::Runtime::new().unwrap()),
            // modules: std::sync::RwLock::new(HashMap::new()),
            common_module: tokio::sync::RwLock::new(None),
            modules: Modules::register_module(),
        }
    }
}

impl RockGateway {
    pub fn start(config: &str) -> RResult<Arc<RockGateway>> {
        let start = std::time::Instant::now();
        let mut gateway = Arc::new(RockGateway::default());
        gateway.load(config)?;      // init config
        gateway.init_common_module()?;          // init common_module
        gateway.run(start)?;
        Ok(gateway)
    }
    pub(crate) fn get_modules(&self) -> Modules {
        self.modules.clone()
    }
    fn init_common_module(&self) -> RResult<()> {
        let instance = self.get_instance()?;
        let v: Arc<Vec<AddressInterface>> = Arc::new(instance.service.interfaces.iter().map(|i| {
            i.clone()
        }).collect());
        let upgrade_profile = UpgradeProfile { interfaces: v, external_ip: instance.service.external_ip.clone() };
        let upgrade_pipe_task = self.modules.make_pipe_task(
            ModuleType::UpgradeInsecureRequest, 
            crate::modules::PipeData::UpgradeInsecureRequestsData { profile: tokio::sync::RwLock::new(upgrade_profile) }
        );
        let route_pipe_task = self.modules.make_pipe_task(
            ModuleType::Route, 
            crate::modules::PipeData::RouteModuleData { profile: tokio::sync::RwLock::new(()) }
        );
        let return_pipe_task = self.modules.make_pipe_task(
            ModuleType::Return, 
            crate::modules::PipeData::ReturnModuleData { profile: tokio::sync::RwLock::new(()) }
        );
        self.runtime.read().unwrap().block_on(async {
            let mut common_module = self.common_module.write().await;
            *common_module = Some(CommonModule {
                upgrade_pipe_task,
                route_pipe_task,
                return_pipe_task,
            });
        });
        Ok(())
    }
    fn update_instance(&self, n_instance: GatewayInstance) -> RResult<()> {
        let mut instance = self.gateway_instance.write().unwrap();
        *instance = Arc::new(n_instance);
        Ok(())
    }
    pub(crate) fn get_instance(&self) -> RResult<Arc<GatewayInstance>> {
        let instance = self.gateway_instance.read().unwrap();
        Ok(instance.clone())
    }
    pub(crate) async fn execute_one_task(&self, ctx: GatewayContext, pipe_task: &PipeTask) -> RResult<GatewayContext> {
        self.modules.schedule(&pipe_task.types, ctx, &pipe_task.pipe_data).await
    }
    fn update_runtime(&self) -> RResult<()> {
        let service_lock = self.gateway_instance.read().unwrap();
        let mut runtime_builder = match service_lock.service.tokio_type {
            TokioType::MultiThread(ref multi_thread_tokio_settings) => {
                let runtime_builder = tokio::runtime::Builder::new_multi_thread();
                self.update_tokio_setting(runtime_builder, multi_thread_tokio_settings, service_lock.bind_cpu.clone())?
            }
            TokioType::CurrentThread(ref current_thread_tokio_settings) => {
                let runtime_builder = tokio::runtime::Builder::new_current_thread();
                self.update_tokio_setting(runtime_builder, current_thread_tokio_settings, service_lock.bind_cpu.clone())?
            }
            TokioType::None => {
                let mut runtime_builder = tokio::runtime::Builder::new_multi_thread();
                runtime_builder.enable_all();
                runtime_builder
            }
        };
        let runtime = runtime_builder.build().map_err(|e| {
            gateway_err!(
                ConfigurationFailed,
                "Tokio settings convert Error!",
                e
            )
        })?;
        // self.runtime.with_mut(|raw| {
        //     unsafe {
        //         *raw = Some(std::sync::RwLock::new(runtime))
        //     }
        // });
        // *self.runtime.borrow_mut() = Some(std::sync::RwLock::new(runtime));
        let mut runtime_lock = self.runtime.write().unwrap();
        *runtime_lock = runtime;
        Ok(())
    }
    fn update_tokio_setting(&self, mut tokio_builder: tokio::runtime::Builder, tokio_settings: &TokioSettings, bind_cpu: Arc<std::sync::Mutex<crate::instance::BindCpu>>) -> RResult<tokio::runtime::Builder> {
        let mut builder = &mut tokio_builder;
        builder = if let Some(core_threads) = tokio_settings.core_threads {
            builder.worker_threads(core_threads)
        } else { builder };
        builder = if let Some(event_interval) = tokio_settings.event_interval {
            builder.event_interval(event_interval)
        } else { builder };
        builder = if let Some(global_queue_interval) = tokio_settings.global_queue_interval {
            builder.global_queue_interval(global_queue_interval)
        } else { builder };
        builder = if let Some(nevents) = tokio_settings.nevents {
            builder.max_io_events_per_tick(nevents)
        } else { builder };
        builder = if let Some(ref thread_name) = tokio_settings.thread_name {
            builder.thread_name(thread_name.clone())
        } else { builder };
        builder = if let Some(thread_stack_size) = tokio_settings.thread_stack_size {
            builder.thread_stack_size(thread_stack_size)
        } else { builder };
        builder = if let Some(max_blocking_threads) = tokio_settings.max_blocking_threads {
            builder.max_blocking_threads(max_blocking_threads)
        } else { builder };
        builder = if let Some(bind_cpu_settings) = tokio_settings.bind_cpu {
            let bind_cpu_start = bind_cpu.clone();
            let bind_cpu_lock = bind_cpu.lock().unwrap();
            let cpus_length = bind_cpu_lock.core_ids.len();
            drop(bind_cpu_lock);
            builder.on_thread_start(move || {
                let mut lock = bind_cpu_start.lock().unwrap();
                let index = match bind_cpu_settings {
                    TokioBindCpuType::All => {
                        lock.index_step % cpus_length
                    }
                    TokioBindCpuType::Half => {
                        lock.index_step % (cpus_length / 2)
                    }
                    TokioBindCpuType::Num(num) => {
                        lock.index_step % num
                    }
                    TokioBindCpuType::Even => {
                        if lock.index_step.wrapping_add(1) % 2 == 0 {
                            lock.index_step % cpus_length
                        } else {
                            lock.index_step.wrapping_sub(1) % cpus_length
                        }
                    }
                    TokioBindCpuType::Odd => {
                        if lock.index_step.wrapping_add(1) % 2 == 0 {
                            lock.index_step.wrapping_add(1) % cpus_length
                        } else {
                            lock.index_step % cpus_length
                        }
                    }
                };
                core_affinity::set_for_current(lock.core_ids[index]);
                // log::info!("thread start:{:?}", lock);
                lock.index_step = lock.index_step + 1;
            });
            let bind_cpu_end = bind_cpu.clone();
            builder.on_thread_stop(move || {
                let mut lock = bind_cpu_end.lock().unwrap();
                // log::info!("thread stop:{:?}", lock);
                lock.index_step = lock.index_step - 1;
            });
            builder
        } else { builder };
        builder.enable_all();
        Ok(tokio_builder)
    }
    fn wait(&self) -> RResult<()> {
        let runtime_lock = self.runtime.read().unwrap();
        runtime_lock.block_on(async {
            crate::common::gateway_signal::signal_hook().await;
        });
        Ok(())
    }
    /*
    start memory_cache expire clear thread
     */
    fn start_memory_cache_clearthread(&self) -> RResult<()> {
        let instance = self.get_gateway_instance()?;
        for ct in &instance.service.cache {
            match ct {
                CacheType::Memory { clear_time_interval, .. } => {
                    if clear_time_interval.clone() != std::time::Duration::from_nanos(0) {
                        let mut caches = Vec::new();
                        let clear_time_interval_for_clear_thread = clear_time_interval.clone();
                        for (_, route) in &instance.routes {
                            caches.push(route.memory_cache_shared.clone());
                        }
                        let _ = std::thread::Builder::new().name("MEMORY_CACHE_CLEAR".to_string()).spawn(move || {
                            loop {
                                for cache in &caches {
                                    cache.lock().retain(|k, _| {
                                        if HttpCacheKey::is_expire(k) {
                                            return false;
                                        }
                                        true
                                    });
                                }
                                std::thread::sleep(clear_time_interval_for_clear_thread.clone());
                            }
                        }).unwrap();
                    }
                },
                _ => {},
            }
        }
        Ok(())
    }
    pub fn spawn<F: std::future::Future<Output=()> + Send + 'static>(&self, f: F) -> RResult<()> {
        // the first option
        // self.runtime.with(|raw| {
        //     unsafe {
        //         if let Some(runtime_lock) = &*raw {
        //             runtime_lock.read().unwrap().spawn(f);
        //         }
        //     }
        // });
        // the second option
        // if let Some(runtime_lock) = &*self.runtime.borrow() {
        //     let join = runtime_lock.read().unwrap().spawn(f);
        // }
        // the third option
        let join = self.runtime.write().unwrap().spawn(f);
        Ok(())
    }
    pub fn tokio_spawn<F: std::future::Future<Output = ()> + Send + 'static>(&self, f: F) -> RResult<()> {
        let join = tokio::task::spawn(f);
        Ok(())
    }
    pub fn test(&self) -> RResult<()> {
        Ok(())
    }
    fn get_gateway_instance(&self) -> RResult<Arc<GatewayInstance>> {
        let gateway_instance = self.gateway_instance.write().unwrap();
        Ok((*gateway_instance).clone())
    }
}
pub(crate) trait RockGatewayEngin {
    fn load(&self, config_file: &str) -> RResult<()>;
    fn run(&mut self, start: std::time::Instant) -> RResult<()>;
    fn run_service(&mut self) -> RResult<()>;
}
impl RockGatewayEngin for Arc<RockGateway> {
    fn load(&self, config_file: &str) -> RResult<()> {
        let config = parsers::parse_file(config_file)?;
        self.update_instance(config.build(self.clone())?)?;
        self.update_runtime()?;
        Ok(())
    }

    fn run(&mut self, start: std::time::Instant) -> RResult<()> {
        self.run_service()?;
        self.start_memory_cache_clearthread()?;
        println!("Service started:{:#?}ms", start.elapsed().as_millis());
        self.wait()?;
        log::info!("Over Gateway Server!!!");
        Ok(())
    }

    fn run_service(&mut self) -> RResult<()> {
        let gateway_instance = self.get_gateway_instance()?;
        for interface in gateway_instance.service.interfaces.iter() {
            let server = GatewayServer::create(interface)?;
            server.start(self)?;
            let server_name = server.name()?;
            let mut servers = self.servers.write().unwrap();
            (*servers).insert(server_name, server);
        }
        Ok(())
    }
}