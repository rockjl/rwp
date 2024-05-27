/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#![feature(test)]
#![feature(impl_trait_in_assoc_type)]

use criterion::{criterion_group, criterion_main, Criterion};
use rock_waypoint::error::RResult;
use std::time::Duration;
use std::future::Future;

// ------------------------------------------------------------- test-duration test_one test_two
fn test_one(c: &mut Criterion) {
    let start = std::time::Instant::now();
    let t = Duration::from_millis(2000);
    let max = 10000;
    c.bench_function("test_one", |b| {
        b.iter(|| {
            for _ in 0..max {
                if start.elapsed() > t {}
            }
        })
    });
    println!("test_one_cost:{:?}ms", start.elapsed().as_millis());
}
fn test_two(c: &mut Criterion) {
    let start = std::time::Instant::now();
    let t = Duration::from_millis(2000);
    let max = 10000;
    c.bench_function("test_two", |b| {
        b.iter(|| {
            for _ in 0..max {
                if start.elapsed() > t {}
            }
        })
    });
    println!("test_two_cost:{:?}ms", start.elapsed().as_millis());
}
// ------------------------------------------------------------- test-trait async
fn test_trait_async(c: &mut Criterion) {
    #[derive(Debug, Clone, Copy)]
    struct Ctx {
        name: i32,
    }
    trait Module {
        type ReturnFuture: Future<Output=RResult<Ctx>>;
        fn execute(&self, ctx: Ctx) -> Self::ReturnFuture;
    }
    struct ModuleA {}
    impl Module for ModuleA {
        type ReturnFuture = impl Future<Output=RResult<Ctx>>;
        fn execute(&self, mut ctx: Ctx) -> Self::ReturnFuture {
            async move {
                ctx.name = 11;
                Ok(ctx)
            }
        }
    }
    fn a_function(mut ctx: Ctx) -> std::pin::Pin<Box<dyn Future<Output=RResult<Ctx>>>> {
        Box::pin(async move {
            ctx.name = 22;
            Ok(ctx)
        })
    }
    async fn b_function(mut ctx: Ctx) -> RResult<Ctx> {
        ctx.name = 33;
        Ok(ctx)
    }
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let m = ModuleA {};
    let mut ctx = Ctx { name: 0, };
    c.bench_function("test_aaa", |b| {
        b.iter(|| {
            rt.block_on(async {
                ctx = a_function(ctx).await.unwrap();
            });
        });
    });
    let mut ctx = Ctx { name: 0, };
    c.bench_function("test_bbb", |b| {
        b.iter(|| {
            rt.block_on(async {
                ctx = b_function(ctx).await.unwrap();
            });
        });
    });
    let mut ctx = Ctx { name: 0, };
    c.bench_function("test_ccc", |b| {
        b.iter(|| {
            rt.block_on(async {
                ctx = m.execute(ctx).await.unwrap();
            });
        });
    });
}

criterion_group!(fib_bench, test_trait_async);
criterion_main!(fib_bench);

