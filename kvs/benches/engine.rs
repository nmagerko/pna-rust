#[macro_use]
extern crate criterion;
extern crate tempfile;

use criterion::{BatchSize, Criterion, ParameterizedBenchmark};
use kvs::{KvStore, KvsEngine, SledKvsEngine};
use tempfile::TempDir;

/// Makes the requested number of key-value entries
///
/// # Arguments
///
/// - length - the number of key-value pairs to make
fn generate_entries(length: usize) -> Vec<(String, String)> {
    let mut entries = vec![(String::new(), String::new()); length];
    for i in 0..length {
        entries[i] = (
            format!("key{}", i).to_owned(),
            format!("value{}", i).to_owned(),
        );
    }
    entries
}

/// Tests setting 50 values on the kvs and sled engines
fn write_benchmark(c: &mut Criterion) {
    let params = vec![generate_entries(50)];
    let benchmark = ParameterizedBenchmark::new(
        "kvs_write",
        |b, param_list| {
            b.iter_batched(
                || {
                    let dir = TempDir::new().unwrap();
                    KvStore::open(dir.path()).unwrap()
                },
                |mut store| {
                    for (k, v) in param_list {
                        store.set(k.to_string(), v.to_string()).unwrap();
                    }
                },
                BatchSize::SmallInput,
            )
        },
        params,
    )
    .with_function("sled_write", |b, param_list| {
        b.iter_batched(
            || {
                let dir = TempDir::new().unwrap();
                SledKvsEngine::open(dir.path()).unwrap()
            },
            |mut store| {
                for (k, v) in param_list {
                    store.set(k.to_string(), v.to_string()).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench("write_benchmark", benchmark);
}

/// Tests reading 20 existing values and 20 non-existent values on the kvs and sled engines
fn read_benchmark(c: &mut Criterion) {
    let params = vec![generate_entries(20)];
    let benchmark = ParameterizedBenchmark::new(
        "kvs_read",
        move |b, param_list| {
            let dir = TempDir::new().unwrap();
            let mut store = KvStore::open(dir.path()).unwrap();
            for (k, v) in param_list {
                store.set(k.to_string(), v.to_string()).unwrap();
            }

            b.iter(|| {
                for (k, v) in param_list {
                    match &store.get(k.to_string()).unwrap() {
                        Some(val) => assert_eq!(val, v),
                        None => panic!("Failed to get key '{}'", k),
                    }
                    match &store.get(format!("{}-bad", k)).unwrap() {
                        Some(_) => panic!("Found non-existent key"),
                        None => {}
                    }
                }
            })
        },
        params,
    )
    .with_function("sled_read", move |b, param_list| {
        let dir = TempDir::new().unwrap();
        let mut store = SledKvsEngine::open(dir.path()).unwrap();
        for (k, v) in param_list {
            store.set(k.to_string(), v.to_string()).unwrap();
        }

        b.iter(|| {
            for (k, v) in param_list {
                match &store.get(k.to_string()).unwrap() {
                    Some(val) => assert_eq!(val, v),
                    None => panic!("Failed to get key '{}'", k),
                }
                match &store.get(format!("{}-bad", k)).unwrap() {
                    Some(_) => panic!("Found non-existent key"),
                    None => {}
                }
            }
        })
    });
    c.bench("read_benchmark", benchmark);
}

criterion_group!(benches, write_benchmark, read_benchmark);
criterion_main!(benches);
