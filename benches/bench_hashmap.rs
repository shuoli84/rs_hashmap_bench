use criterion::{criterion_group, criterion_main, Criterion};
use hash_perf::*;
use rand::RngCore;

fn prepare_test_hashmap<H: TestMap>(values: &[u64]) -> H {
    let mut map = H::default();
    for v in values.iter() {
        map.insert(*v, *v);
    }
    map
}

fn run_get<H: TestMap>(map: &H, keys: &[u64], result: &mut Vec<u64>) {
    for key in keys.iter() {
        if let Some(v) = map.get(*key) {
            result.push(*v);
        }
    }
}

fn run_insert<H: TestMap>(map: &mut H, values: &[u64]) {
    for key in values.iter() {
        map.insert(*key, *key);
    }
}

fn bench_get<H: TestMap>(values: &[u64], c: &mut Criterion) {
    let type_name = H::type_name();
    let mut result = Vec::with_capacity(values.len());

    let n = values.len() as u64;

    let map = prepare_test_hashmap::<H>(values);
    c.bench_function(&format!("get_{type_name}_{n}"), |b| {
        b.iter(|| run_get(&map, values, &mut result))
    });
}

fn bench_insert<H: TestMap>(values: &[u64], c: &mut Criterion) {
    let type_name = H::type_name();
    let n = values.len();

    let mut map = H::default();

    c.bench_function(&format!("insert_{type_name}_{n}"), |b| {
        b.iter(|| run_insert(&mut map, values))
    });
}

pub fn cirterion_benchmark_get(c: &mut Criterion) {
    for n in [2000, 10000_u64] {
        let mut rng = rand::thread_rng();
        let mut values = Vec::with_capacity(n as usize);
        for _ in 0..n {
            values.push(rng.next_u64());
        }

        let values = &values;

        bench_get::<StdDefault>(values, c);

        bench_get::<StdFxHash>(values, c);
        bench_get::<HashBrown13FxHash>(values, c);

        bench_get::<StdAHash>(values, c);
        bench_get::<HashBrown13AHash>(values, c);
    }
}

pub fn cirterion_benchmark_insert(c: &mut Criterion) {
    for n in [2000, 10000] {
        let mut rng = rand::thread_rng();
        let mut values = Vec::with_capacity(n as usize);
        for _ in 0..n {
            values.push(rng.next_u64());
        }

        let values = &values;

        bench_insert::<StdDefault>(values, c);

        bench_insert::<StdFxHash>(values, c);
        bench_insert::<HashBrown13FxHash>(values, c);

        bench_insert::<StdAHash>(values, c);
        bench_insert::<HashBrown13AHash>(values, c);
    }
}

criterion_group!(benches, cirterion_benchmark_get, cirterion_benchmark_insert);
criterion_main!(benches);
