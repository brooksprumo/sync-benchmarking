use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use sync_benchmarking::*;

// bprumo TODO: things to benchmark
// all readers vs all writers
// num threads contending
//      - 2 vs 4 vs 8 vs 16
// contending as fast as possible vs infrequent
// size of data behind the lock

fn bench_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("small");
    for ref num_threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::new("std::mutex", num_threads),
            num_threads,
            |b, num_threads| b.iter(|| bench_std_mutex::<Small>(*num_threads)),
        );
        group.bench_with_input(
            BenchmarkId::new("parking_log::mutex", num_threads),
            num_threads,
            |b, num_threads| b.iter(|| bench_parking_lot_mutex::<Small>(*num_threads)),
        );
        group.bench_with_input(
            BenchmarkId::new("std::rwlock", num_threads),
            num_threads,
            |b, num_threads| b.iter(|| bench_std_rwlock::<Small>(*num_threads)),
        );
        group.bench_with_input(
            BenchmarkId::new("parking_lot::rwlock", num_threads),
            num_threads,
            |b, num_threads| b.iter(|| bench_parking_lot_rwlock::<Small>(*num_threads)),
        );
    }
}

fn bench_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("large");
    for ref num_threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::new("std::mutex", num_threads),
            num_threads,
            |b, threads| b.iter(|| bench_std_mutex::<Large>(*threads)),
        );
        group.bench_with_input(
            BenchmarkId::new("parking_log::mutex", num_threads),
            num_threads,
            |b, threads| b.iter(|| bench_parking_lot_mutex::<Large>(*threads)),
        );
        group.bench_with_input(
            BenchmarkId::new("std::rwlock", num_threads),
            num_threads,
            |b, threads| b.iter(|| bench_std_rwlock::<Large>(*threads)),
        );
        group.bench_with_input(
            BenchmarkId::new("parking_lot::rwlock", num_threads),
            num_threads,
            |b, threads| b.iter(|| bench_parking_lot_rwlock::<Large>(*threads)),
        );
    }
}

fn bench_std_mutex<T: 'static + Default + Counter + Send>(num_threads: usize) {
    let amount_of_work = 16_000;
    let amount_of_work_per_thread = amount_of_work / num_threads;
    let t = Arc::new(std::sync::Mutex::new(T::default()));

    // bprumo TODO: don't want to test the time it takes to spawn the threads...

    (0..num_threads)
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
        .for_each(|thread| thread.join().unwrap());

    assert_eq!(t.lock().unwrap().counter() as usize, amount_of_work);
}

fn bench_parking_lot_mutex<T: 'static + Default + Counter + Send>(num_threads: usize) {
    let amount_of_work = 16_000;
    let amount_of_work_per_thread = amount_of_work / num_threads;
    let t = Arc::new(parking_lot::Mutex::new(T::default()));

    // bprumo TODO: don't want to test the time it takes to spawn the threads...

    (0..num_threads)
        .into_iter()
        .map(|_| {
            let t = Arc::clone(&t);
            std::thread::spawn(move || {
                for _ in 0..amount_of_work_per_thread {
                    let mut t = t.lock();
                    *t.counter_mut() += 1;
                }
            })
        })
        .for_each(|thread| thread.join().unwrap());

    assert_eq!(t.lock().counter() as usize, amount_of_work);
}

fn bench_std_rwlock<T: 'static + Default + Counter + Send + Sync>(num_threads: usize) {
    let amount_of_work = 16_000;
    let amount_of_work_per_thread = amount_of_work / num_threads;
    let t = Arc::new(std::sync::RwLock::new(T::default()));

    // bprumo TODO: don't want to test the time it takes to spawn the threads...

    (0..num_threads)
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
        .for_each(|thread| thread.join().unwrap());

    assert_eq!(t.read().unwrap().counter() as usize, amount_of_work);
}

fn bench_parking_lot_rwlock<T: 'static + Default + Counter + Send + Sync>(num_threads: usize) {
    let amount_of_work = 16_000;
    let amount_of_work_per_thread = amount_of_work / num_threads;
    let t = Arc::new(parking_lot::RwLock::new(T::default()));

    // bprumo TODO: don't want to test the time it takes to spawn the threads...

    (0..num_threads)
        .into_iter()
        .map(|_| {
            let t = Arc::clone(&t);
            std::thread::spawn(move || {
                for _ in 0..amount_of_work_per_thread {
                    let mut t = t.write();
                    *t.counter_mut() += 1;
                }
            })
        })
        .for_each(|thread| thread.join().unwrap());

    assert_eq!(t.read().counter() as usize, amount_of_work);
}

criterion_group!(benches, bench_small, bench_large);
criterion_main!(benches);
