/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::{pin::Pin, ptr::NonNull};

use criterion::{criterion_group, criterion_main, Criterion};
use rock_waypoint::error::RResult;
use std::future::Future;
#[derive(Debug)]
struct Ctx {
    index: u32,
}
//-----------------------------new main framework-----------------------
trait OldModule: Send + Sync {
    fn execute(&self, ctx: Ctx) -> Pin<Box<dyn Future<Output=RResult<Ctx>> + Send>>;
}
struct OldModule_A {}
impl OldModule for OldModule_A {
    fn execute(&self, mut ctx: Ctx) -> Pin<Box<dyn Future<Output=RResult<Ctx>> + Send>> {
        Box::pin(async move {
            ctx.index = ctx.index + 1;
            Ok(ctx)
        })
    }
}
struct OldModule_B {}
impl OldModule for OldModule_B {
    fn execute(&self, mut ctx: Ctx) -> Pin<Box<dyn Future<Output=RResult<Ctx>> + Send>> {
        Box::pin(async move {
            ctx.index = ctx.index + 1;
            Ok(ctx)
        })
    }
}
struct OldModule_C {}
impl OldModule for OldModule_C {
    fn execute(&self, mut ctx: Ctx) -> Pin<Box<dyn Future<Output=RResult<Ctx>> + Send>> {
        Box::pin(async move {
            ctx.index = ctx.index + 1;
            Ok(ctx)
        })
    }
}
struct OldModule_D {}
impl OldModule for OldModule_D {
    fn execute(&self, mut ctx: Ctx) -> Pin<Box<dyn Future<Output=RResult<Ctx>> + Send>> {
        Box::pin(async move {
            ctx.index = ctx.index + 1;
            Ok(ctx)
        })
    }
}
struct OldModule_E {}
impl OldModule for OldModule_E {
    fn execute(&self, mut ctx: Ctx) -> Pin<Box<dyn Future<Output=RResult<Ctx>> + Send>> {
        Box::pin(async move {
            ctx.index = ctx.index + 1;
            Ok(ctx)
        })
    }
}
struct OldModule_F {}
impl OldModule for OldModule_F {
    fn execute(&self, mut ctx: Ctx) -> Pin<Box<dyn Future<Output=RResult<Ctx>> + Send>> {
        Box::pin(async move {
            ctx.index = ctx.index + 1;
            Ok(ctx)
        })
    }
}
fn init_module_a() -> Box<dyn OldModule> {
    Box::new(OldModule_A{})
}
fn init_module_b() -> Box<dyn OldModule> {
    Box::new(OldModule_B{})
}
fn init_module_c() -> Box<dyn OldModule> {
    Box::new(OldModule_C{})
}
fn init_module_d() -> Box<dyn OldModule> {
    Box::new(OldModule_D{})
}
fn init_module_e() -> Box<dyn OldModule> {
    Box::new(OldModule_E{})
}
fn init_module_f() -> Box<dyn OldModule> {
    Box::new(OldModule_F{})
}
struct OldPipe {
    pipe_module: Box<dyn OldModule>,
}
//-----------------------------new main framework-----------------------
#[derive(Debug)]
enum ModuleType {
    MODULEA,
    MODULEB,
    MODULEC,
    MODULED,
    MODULEE,
    MODULERETURN,
}

trait Module {
    fn name(&self) -> ModuleType;
    fn make_pipe_task(&self, next_task: Option<Box<PipeTask>>, pipe_data: PipeData) -> PipeTask;
    async fn execute(&self, ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> ;
}
struct Module_A{}
impl Module for Module_A {
    fn name(&self) -> ModuleType {
        ModuleType::MODULEA
    }
    async fn execute(&self, mut ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> {
        if let PipeData::ModuleAData { data_a } = pipe_data {
            ctx.index = ctx.index + data_a.clone();   
        }
        Ok(ctx)
    }
    
    fn make_pipe_task(&self, next_task: Option<Box<PipeTask>>, pipe_data: PipeData) -> PipeTask {
        PipeTask {
            name: self.name(),
            next_task,
            pipe_data
        }
    }
}
struct Module_B {}
impl Module for Module_B {
    fn name(&self) -> ModuleType {
        ModuleType::MODULEB
    }
    async fn execute(&self, mut ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> {
        if let PipeData::ModuleBData { data_b } = pipe_data {
            ctx.index = ctx.index + data_b.clone();   
        }
        Ok(ctx)
    }
    
    fn make_pipe_task(&self, next_task: Option<Box<PipeTask>>, pipe_data: PipeData) -> PipeTask {
        PipeTask {
            name: self.name(),
            next_task,
            pipe_data
        }
    }
}
struct Module_C {}
impl Module for Module_C {
    fn name(&self) -> ModuleType {
        ModuleType::MODULEC
    }
    async fn execute(&self, mut ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> {
        if let PipeData::ModuleCData { data_c } = pipe_data {
            ctx.index = ctx.index + data_c.clone();   
        }
        Ok(ctx)
    }
    
    fn make_pipe_task(&self, next_task: Option<Box<PipeTask>>, pipe_data: PipeData) -> PipeTask {
        PipeTask {
            name: self.name(),
            next_task,
            pipe_data
        }
    }
}
struct Module_D {}
impl Module for Module_D {
    fn name(&self) -> ModuleType {
        ModuleType::MODULED
    }
    async fn execute(&self, mut ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> {
        if let PipeData::ModuleDData { data_d } = pipe_data {
            ctx.index = ctx.index + data_d.clone();   
        }
        Ok(ctx)
    }
    
    fn make_pipe_task(&self, next_task: Option<Box<PipeTask>>, pipe_data: PipeData) -> PipeTask {
        PipeTask {
            name: self.name(),
            next_task,
            pipe_data
        }
    }
}
struct Module_E {}
impl Module for Module_E {
    fn name(&self) -> ModuleType {
        ModuleType::MODULEE
    }
    async fn execute(&self, mut ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> {
        if let PipeData::ModuleEData { data_e } = pipe_data {
            ctx.index = ctx.index + data_e.clone();   
        }
        Ok(ctx)
    }
    
    fn make_pipe_task(&self, next_task: Option<Box<PipeTask>>, pipe_data: PipeData) -> PipeTask {
        PipeTask {
            name: self.name(),
            next_task,
            pipe_data
        }
    }
}
struct Module_Return {}
impl Module for Module_Return {
    fn name(&self) -> ModuleType {
        ModuleType::MODULERETURN
    }
    async fn execute(&self, mut ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> {
        if let PipeData::ModuleReturn = pipe_data {  
            ctx.index = ctx.index + 1;
        }
        Ok(ctx)
    }
    
    fn make_pipe_task(&self, _: Option<Box<PipeTask>>, pipe_data: PipeData) -> PipeTask {
        PipeTask {
            name: self.name(),
            next_task: None,
            pipe_data
        }
    }
}
struct PipeLine {
    task: Box<PipeTask>,
    module_scheduling: ModuleScheduling,
}
struct PipeLineOne {
    tasks: Vec<PipeTask>,
    module_scheduling: ModuleScheduling,
}
#[derive(Debug)]
struct PipeTask {
    name: ModuleType,
    next_task: Option<Box<PipeTask>>,
    pipe_data: PipeData,
}   
#[derive(Debug)]
enum PipeData {
    ModuleAData {
        data_a: u32,
    },
    ModuleBData {
        data_b: u32,
    },
    ModuleCData {
        data_c: u32,
    },
    ModuleDData {
        data_d: u32,
    },
    ModuleEData {
        data_e: u32,
    },
    ModuleReturn,

}
struct ModuleScheduling {
    modulea: Module_A,
    moduleb: Module_B,
    modulec: Module_C,
    moduled: Module_D,
    modulee: Module_E,
    modulereturn: Module_Return,
}
impl ModuleScheduling {
    fn register_module() -> Self {
        Self {
            modulea: Module_A{},
            moduleb: Module_B{},
            modulec: Module_C{},
            moduled: Module_D{},
            modulee: Module_E{},
            modulereturn: Module_Return{}
        }
    }
    #[inline(always)]
    async fn schedule(&self, pipe_name: &ModuleType, ctx: Ctx, pipe_data: &mut PipeData) -> RResult<Ctx> {
        match pipe_name {
            ModuleType::MODULEA => {
                return self.modulea.execute(ctx, pipe_data).await;
            }
            ModuleType::MODULEB => {
                return self.moduleb.execute(ctx, pipe_data).await;
            }
            ModuleType::MODULEC => {
                return self.modulec.execute(ctx, pipe_data).await;
            }
            ModuleType::MODULED => {
                return self.moduled.execute(ctx, pipe_data).await;
            }
            ModuleType::MODULEE => {
                return self.modulee.execute(ctx, pipe_data).await;
            }
            ModuleType::MODULERETURN => {
                return self.modulereturn.execute(ctx, pipe_data).await;
            }
        }
    }
    fn make_pipe_task(&self, pipe_name: ModuleType, pipe_data: PipeData) -> PipeTask {
        match pipe_name {
            ModuleType::MODULEA => {
                return self.modulea.make_pipe_task(None, pipe_data);
            }
            ModuleType::MODULEB => {
                return self.moduleb.make_pipe_task(None, pipe_data);
            }
            ModuleType::MODULEC => {
                return self.modulec.make_pipe_task(None, pipe_data);
            }
            ModuleType::MODULED => {
                return self.moduled.make_pipe_task(None, pipe_data);
            }
            ModuleType::MODULEE => {
                return self.modulee.make_pipe_task(None, pipe_data);
            }
            ModuleType::MODULERETURN => {
                return self.modulereturn.make_pipe_task(None, pipe_data);
            }
        }
    }
    async fn execute(&self, mut ctx: Ctx, pipe_task: &mut PipeTask) -> RResult<Ctx> {
        let mut c_t = pipe_task;
        loop {
            // println!("c_t:{:#?}", c_t);
            ctx = self.schedule(&c_t.name, ctx, &mut c_t.pipe_data).await?;
            c_t = if let Some(n_t) = &mut c_t.next_task {
                n_t.as_mut()
            } else {
                return Ok(ctx);
            };
        }
    }
    async fn execute_one(&self, mut ctx: Ctx, pipe_task: &mut Vec<PipeTask>) -> RResult<Ctx> {
        for task in pipe_task {
            ctx = self.schedule(&task.name, ctx, &mut task.pipe_data).await?;
        }
        Ok(ctx)
    }
}
impl PipeLine {
    async fn execute(&mut self, ctx: Ctx) -> RResult<Ctx> {
        self.module_scheduling.execute(ctx, self.task.as_mut()).await
    }
}
impl PipeLineOne {
    async fn execute_one(&mut self, ctx: Ctx) -> RResult<Ctx> {
        self.module_scheduling.execute_one(ctx, &mut self.tasks).await
    }
}
const MAX: usize = 100000;
fn test_old_main(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    c.bench_function("test_main_framework", |b| {
        b.iter(|| {
            let pipe_schedule = ModuleScheduling::register_module();
            let mut pipe_task = pipe_schedule.make_pipe_task(ModuleType::MODULEA, PipeData::ModuleAData { data_a: 1 });
            let mut pipe_task = Box::new(pipe_task);
            let mut pipe_datas = Vec::new();
            pipe_datas.push((ModuleType::MODULEB, PipeData::ModuleBData { data_b: 1 }));
            pipe_datas.push((ModuleType::MODULEC, PipeData::ModuleCData { data_c: 1 }));
            pipe_datas.push((ModuleType::MODULED, PipeData::ModuleDData { data_d: 1 }));
            pipe_datas.push((ModuleType::MODULEE, PipeData::ModuleEData { data_e: 1 }));
            pipe_datas.push((ModuleType::MODULERETURN, PipeData::ModuleReturn));

            let mut cur_mut_task = pipe_task.as_mut();
            for (module_name, pipe_data) in pipe_datas {
                let mut next_task = pipe_schedule.make_pipe_task(module_name, pipe_data);
                let mut next_mut_task = Box::new(next_task);
                cur_mut_task.next_task = Some(next_mut_task);
                cur_mut_task = match &mut cur_mut_task.next_task {
                    Some(n_t) => {
                        n_t.as_mut()
                    },
                    None => {
                        unreachable!()
                    }
                };
            }
            let mut pipe_line = PipeLine {
                task: pipe_task,
                module_scheduling: pipe_schedule,
            };
                
            let mut ctx = Ctx { index: 0 };
            rt.block_on(async move {
                let start = std::time::Instant::now();
                for _ in 0..MAX {
                    ctx = pipe_line.execute(ctx).await.unwrap();
                }
                // println!("new -({:?}) cost:{:?}ms", ctx, start.elapsed().as_millis());
            });
        });
    });
    c.bench_function("test_old_main_framework", |b| {
        
        b.iter(  || {
            let mut pipes = Vec::new();
            pipes.push(OldPipe { pipe_module: init_module_a() });
            pipes.push(OldPipe { pipe_module: init_module_b() });
            pipes.push(OldPipe { pipe_module: init_module_c() });
            pipes.push(OldPipe { pipe_module: init_module_d() });
            pipes.push(OldPipe { pipe_module: init_module_e() });
            pipes.push(OldPipe { pipe_module: init_module_f() });
            let mut ctx = Ctx { index: 0 };
            rt.block_on(async move {
                let start = std::time::Instant::now();
                for _ in 0..MAX {
                    for pipe in &pipes {
                        ctx = pipe.pipe_module.execute(ctx).await.unwrap();
                    }
                }
                // println!("old -({:?}) cost:{:?}ms", ctx, start.elapsed().as_millis());
            });
        });
    });
    
    c.bench_function("test_main_framework_vec", |b| {
        b.iter(|| {
            let pipe_schedule = ModuleScheduling::register_module();
            let mut pipe_datas = Vec::new();
            pipe_datas.push((ModuleType::MODULEA, PipeData::ModuleAData { data_a: 1 }));
            pipe_datas.push((ModuleType::MODULEB, PipeData::ModuleBData { data_b: 1 }));
            pipe_datas.push((ModuleType::MODULEC, PipeData::ModuleCData { data_c: 1 }));
            pipe_datas.push((ModuleType::MODULED, PipeData::ModuleDData { data_d: 1 }));
            pipe_datas.push((ModuleType::MODULEE, PipeData::ModuleEData { data_e: 1 }));
            pipe_datas.push((ModuleType::MODULERETURN, PipeData::ModuleReturn));

            let mut pipe_line = Vec::new();
            for (module_name, pipe_data) in pipe_datas {
                let mut pipe_task = pipe_schedule.make_pipe_task(module_name, pipe_data);
                pipe_line.push(pipe_task);
            }
            let mut pipe_line = PipeLineOne {
                tasks: pipe_line,
                module_scheduling: pipe_schedule,
            };
                
            let mut ctx = Ctx { index: 0 };
            rt.block_on(async move {
                let start = std::time::Instant::now();
                for _ in 0..MAX {
                    ctx = pipe_line.execute_one(ctx).await.unwrap();
                }
                // println!("new vec -({:?}) cost:{:?}ms", ctx, start.elapsed().as_millis());
            });
        });
    });
}
criterion_group!(fib_bench, test_old_main);
criterion_main!(fib_bench);