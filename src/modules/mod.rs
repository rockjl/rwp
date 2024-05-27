/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#![allow(non_snake_case)]

pub(crate) mod blackandwhitelist;
pub(crate) mod ratelimiter;
pub(crate) mod upgrade;
pub(crate) mod route;
pub(crate) mod r#return;
pub(crate) mod header;
pub(crate) mod dispatche;
pub(crate) mod cache;
pub(crate) mod balance;

use std::sync::Arc;

use crate::{context::{ContextType, GatewayContext}, error::{AsyncResult, RResult} };

use self::{
    balance::{
        ip_match_round_robin_balance::IpRoundRobinBalancer, 
        random_balance::RandomBalancer, 
        round_robin_balance::RoundRobinBalancer, LoadBalanceProfile, 
    }, 
    blackandwhitelist::black_and_white_list::{
        BlackAndWhiteList, 
        BlackAndWhiteListProfile
    }, 
    cache::{
        memory::{
            memory_get::MemoryCacheGet, 
            memory_set::MemoryCacheSet
        }, 
        redis::{
            redis_get::RedisCacheGet, 
            redis_set::RedisCacheSet
        }, 
        CacheProfile, 
    }, 
    dispatche::{
        file::FileServerDispatche, 
        network::NetworkDispatche, 
        DispatcheProfile
    }, 
    header::{
        header_request::HeaderRequest, 
        header_response::HeaderResponse, 
        HeaderProfile
    }, 
    ratelimiter::{
        ratelimiter::RatelimiterModule, 
        RatelimiterProfile
    }, 
    r#return::ReturnModule, 
    route::route::RouteModule, 
    upgrade::{
        upgrade_insecure_requests::UpgradeInsecureRequests,
        UpgradeProfile
    }
};
macro_rules! module_initial{
    ($($module_type_name:ident($module_type_str:expr)($pipe_data_name:ident)($module_name:ident) -> ($pipe_data_profile:ty))+) => {
        #[derive(Debug, Clone)]
        pub(crate) enum ModuleType {
            $(
                $module_type_name ,
            )*
        }
        impl ModuleType {
            pub(crate) fn from(module_name: &str) -> Self {
                match module_name {
                    $(
                        $module_type_str => { Self::$module_type_name }
                    )*
                    _ => {
                        unreachable!("init ModuleType failed");
                    }
                }
            }
        }
        #[derive(Debug)]
        pub(crate) enum PipeData {
            $(
                $pipe_data_name {
                    profile: tokio::sync::RwLock<$pipe_data_profile>
                },
            )*
        }
        #[derive(Debug, Clone)]
        pub(crate) struct Modules {
            $( 
                $module_name : $module_name ,
            )*
        }
        impl Modules {
            pub(crate) fn register_module() -> Self {
                Self {
                    $( 
                        $module_name : $module_name {},
                    )*
                }
            }
            #[inline(always)]
            pub(crate) async fn schedule(&self, module_type: &ModuleType, ctx: GatewayContext, pipe_data: &PipeData) -> RResult<GatewayContext> {
                match module_type {
                    $( 
                        ModuleType::$module_type_name => {
                            return self.$module_name .execute(ctx, pipe_data).await;
                        },
                    )*
                }
            }
            $(
                pub(crate) async fn $module_type_name (&self, ctx: GatewayContext, pipe_data: &PipeData) -> RResult<GatewayContext> {
                    self.$module_name .execute(ctx, pipe_data).await
                }
            )*
            pub(crate) fn make_pipe_task(&self, module_type: ModuleType, pipe_data: PipeData) -> Box<PipeTask> {
                match module_type {
                    $( 
                        ModuleType::$module_type_name => {
                            return Box::new(self.$module_name .make_pipe_task(None, pipe_data));
                        },
                    )*
                };
            }
        }
    };
}
module_initial! {
    UpgradeInsecureRequest("upgrade_insecure_requests")(UpgradeInsecureRequestsData)(UpgradeInsecureRequests) -> (UpgradeProfile)
    IpMatchRoundRobinLB("ip_round_robin")(IpRoundRobinBalancerData)(IpRoundRobinBalancer) -> (LoadBalanceProfile)
    RandomLB("random")(RandomBalancerData)(RandomBalancer) -> (LoadBalanceProfile)
    RoundBorinLB("round_robin")(RoundRobinBalancerData)(RoundRobinBalancer) -> (LoadBalanceProfile)
    BlackAndWhiteList("black_white_list")(BlackAndWhiteListData)(BlackAndWhiteList) -> (BlackAndWhiteListProfile)
    MemoryGet("memory_cache_get")(MemoryCacheGetData)(MemoryCacheGet) -> (CacheProfile)
    MemorySet("memory_cache_set")(MemoryCacheSetData)(MemoryCacheSet) -> (CacheProfile)
    RedisGet("redis_cache_get")(RedisCacheGetData)(RedisCacheGet) -> (CacheProfile)
    RedisSet("redis_cache_set")(RedisCacheSetData)(RedisCacheSet) -> (CacheProfile)
    DispatchFile("dispatche_file")(FileServerDispatcheData)(FileServerDispatche) -> (DispatcheProfile)
    DispatchNetwork("dispatche_network")(NetworkDispatcheData)(NetworkDispatche) -> (DispatcheProfile)
    HeaderRequest("header_request")(HeaderRequestData)(HeaderRequest) -> (HeaderProfile)
    HeaderResponse("header_response")(HeaderResponseData)(HeaderResponse) -> (HeaderProfile)
    RateLimiter("ratelimiter")(RatelimiterModuleData)(RatelimiterModule) -> (RatelimiterProfile)
    Route("route")(RouteModuleData)(RouteModule) -> (())
    Return("return")(ReturnModuleData)(ReturnModule) -> (())
}
pub(crate) trait PipeModule: std::fmt::Debug + Send + Sync + Clone {
    fn name(&self) -> ModuleType;
    fn make_pipe_task(&self, next_task: Option<Box<crate::modules::PipeTask>>, pipe_data: crate::modules::PipeData) -> crate::modules::PipeTask {
        crate::modules::PipeTask { types: self.name(), next_task, pipe_data }
    }
    async fn execute(&self, ctx: GatewayContext, pipe_data: &PipeData) -> RResult<GatewayContext> ;
}
#[derive(Debug)]
pub(crate) struct PipeTask {
    pub(crate) types: ModuleType,
    pub(crate) next_task: Option<Box<PipeTask>>,
    pub(crate) pipe_data: PipeData,
}
#[derive(Debug)]
pub(crate) struct PipeLineEngine {
    pub(crate) task: Box<PipeTask>,
    pub(crate) module_scheduling: Modules,
}
impl PipeLineEngine {
    pub(crate) async fn execute(&self, mut ctx: GatewayContext) -> RResult<GatewayContext> {
        let mut current_task = self.task.as_ref();
        loop {
            // println!("CURRENT_TASK:{:#?}", current_task.types);
            if let ContextType::HttpContext(http_context) = &mut ctx.context_type {
                if (ctx.prompt_return && http_context.cache_hit) || 
                    http_context.return_context.response.is_some() {
                    return Ok(ctx);
                }
                match current_task.types {
                    ModuleType::DispatchNetwork | ModuleType::DispatchFile => {
                        if http_context.response_context.is_new() || http_context.cache_hit{
                            return Ok(ctx);
                        }
                    }
                    ModuleType::MemorySet | ModuleType::RedisSet => {
                        if http_context.cache_hit {
                            return Ok(ctx);
                        }
                    }
                    ModuleType::Return => {
                        http_context.response_context.set_old();
                        http_context.cache_hit = false;
                    }
                    _ => {
    
                    }
                }
            }
            ctx = self.module_scheduling.schedule(&current_task.types, ctx, &current_task.pipe_data).await?;
            if let Some(next_task) = &current_task.next_task {
                current_task = next_task.as_ref();
            } else {
                return Ok(ctx);
            }
        }
        // drop(task_lock);
    }
}

pub(crate) struct CommonModule {
    pub(crate) upgrade_pipe_task: Box<PipeTask>,
    pub(crate) route_pipe_task: Box<PipeTask>,
    pub(crate) return_pipe_task: Box<PipeTask>,
}