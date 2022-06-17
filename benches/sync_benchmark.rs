use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use sync_benchmarking::*;

const AMOUNT_OF_WORK: usize = 16_000;

fn bench_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("small");
    for ref num_threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::new("std::mutex", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(std::sync::Mutex::new(Small::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.lock().unwrap();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.lock().unwrap().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new("parking_lot::mutex", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(parking_lot::Mutex::new(Small::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.lock();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.lock().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new("std::rwlock", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(std::sync::RwLock::new(Small::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.write().unwrap();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.read().unwrap().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new("parking_lot::rwlock", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(parking_lot::RwLock::new(Small::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.write();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.read().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

fn bench_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("large");
    for ref num_threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::new("std::mutex", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(std::sync::Mutex::new(Large::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.lock().unwrap();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.lock().unwrap().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new("parking_lot::mutex", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(parking_lot::Mutex::new(Large::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.lock();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.lock().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new("std::rwlock", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(std::sync::RwLock::new(Large::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.write().unwrap();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.read().unwrap().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new("parking_lot::rwlock", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter_batched(
                    || {
                        let x = Arc::new(parking_lot::RwLock::new(Large::default()));
                        let go = Arc::new(AtomicBool::new(false));
                        let threads: Vec<_> = (0..num_threads)
                            .into_iter()
                            .map(|_| {
                                let x = Arc::clone(&x);
                                let go = Arc::clone(&go);
                                let amount_of_work_per_thread = AMOUNT_OF_WORK / num_threads;
                                std::thread::spawn(move || {
                                    while !go.load(Ordering::Acquire) {}
                                    for _ in 0..amount_of_work_per_thread {
                                        let mut x = x.write();
                                        *x.counter_mut() += 1;
                                    }
                                })
                            })
                            .collect();
                        (x, go, threads)
                    },
                    |(x, go, threads)| {
                        go.store(true, Ordering::Release);
                        threads
                            .into_iter()
                            .for_each(|thread| thread.join().unwrap());
                        assert_eq!(x.read().counter() as usize, AMOUNT_OF_WORK);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

criterion_group!(benches, bench_small, bench_large);
criterion_main!(benches);
