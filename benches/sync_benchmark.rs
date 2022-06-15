use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::{Arc, Mutex, RwLock};
use sync_benchmarking::*;

// bprumo TODO: things to benchmark
// all readers vs all writers
// num threads contending
//      - 2 vs 4 vs 8 vs 16
// contending as fast as possible vs infrequent
// size of data behind the lock

fn bench_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("small");
    for ref threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(BenchmarkId::new("mutex", threads), threads, |b, threads| {
            b.iter(|| bench_mutex::<Small>(*threads))
        });
        group.bench_with_input(
            BenchmarkId::new("rwlock", threads),
            threads,
            |b, threads| b.iter(|| bench_rwlock::<Small>(*threads)),
        );
    }
}

fn bench_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("large");
    for ref threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(BenchmarkId::new("mutex", threads), threads, |b, threads| {
            b.iter(|| bench_mutex::<Large>(*threads))
        });
        group.bench_with_input(
            BenchmarkId::new("rwlock", threads),
            threads,
            |b, threads| b.iter(|| bench_rwlock::<Large>(*threads)),
        );
    }
}

fn bench_mutex<T: 'static + Default + Counter + Send>(num_threads: usize) {
    let amount_of_work = 16_000;
    let amount_of_work_per_thread = amount_of_work / num_threads;
    let t = Arc::new(Mutex::new(T::default()));

    // bprumo TODO: don't want to test the time it takes to spawn the threads...

    let threads: Vec<_> = (0..num_threads)
        .into_iter()
        .map(|_| {
            let t = Arc::clone(&t);
            std::thread::spawn(move || {
                for _ in 0..amount_of_work_per_thread {
                    let mut t = t.lock().unwrap();
                    *t.counter_mut() += 1;
                }
            })
        })
        .collect();

    threads
        .into_iter()
        .for_each(|thread| thread.join().unwrap());
    assert_eq!(t.lock().unwrap().counter() as usize, amount_of_work);
}

fn bench_rwlock<T: 'static + Default + Counter + Send + Sync>(num_threads: usize) {
    let amount_of_work = 16_000;
    let amount_of_work_per_thread = amount_of_work / num_threads;
    let t = Arc::new(RwLock::new(T::default()));

    // bprumo TODO: don't want to test the time it takes to spawn the threads...

    let threads: Vec<_> = (0..num_threads)
        .into_iter()
        .map(|_| {
            let t = Arc::clone(&t);
            std::thread::spawn(move || {
                for _ in 0..amount_of_work_per_thread {
                    let mut t = t.write().unwrap();
                    *t.counter_mut() += 1;
                }
            })
        })
        .collect();

    threads
        .into_iter()
        .for_each(|thread| thread.join().unwrap());
    assert_eq!(t.read().unwrap().counter() as usize, amount_of_work);
}

criterion_group!(benches, bench_small, bench_large);
criterion_main!(benches);
