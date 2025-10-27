use criterion::{criterion_group, criterion_main, Criterion, ParameterizedBenchmark, Throughput};
use nano_arena::{Arena, Idx};

#[derive(Default)]
struct Small(usize);

#[derive(Default)]
struct Big([usize; 32]);

fn insert<T: Default>(n: usize) {
    let mut arena = Arena::<T>::new();
    for _ in 0..n {
        let idx = arena.insert(Default::default());
        arena.swap_remove(idx);
        let idx = arena.insert(Default::default());
        criterion::black_box(idx);
    }
}

fn insert_and_delete<T: Default>(n: usize) {
    let mut arena = Arena::<T>::new();
    for _ in 0..n {
        let idx = arena.insert(Default::default());
        arena.swap_remove(idx);
        let idx = arena.insert(Default::default());
        criterion::black_box(idx);
    }
}

fn lookup<T>(arena: &Arena<T>, idx: &Idx, n: usize) {
    for _ in 0..n {
        criterion::black_box(&arena.get(idx).unwrap());
    }
}

fn collect<T>(arena: &Arena<T>, n: usize) {
    for _ in 0..n {
        criterion::black_box(arena.iter().collect::<Vec<_>>());
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "insert",
        ParameterizedBenchmark::new(
            "insert-small",
            |b, n| b.iter(|| insert::<Small>(*n)),
            (1..3).map(|n| n * 100).collect::<Vec<usize>>(),
        )
        .throughput(|n| Throughput::Elements(*n as u64)),
    );

    c.bench(
        "insert",
        ParameterizedBenchmark::new(
            "insert-big",
            |b, n| b.iter(|| insert::<Big>(*n)),
            (1..3).map(|n| n * 100).collect::<Vec<usize>>(),
        )
        .throughput(|n| Throughput::Elements(*n as u64)),
    );

    c.bench(
        "lookup",
        ParameterizedBenchmark::new(
            "lookup-small",
            |b, n| {
                let mut small_arena = Arena::<Small>::new();
                for _ in 0..1024 {
                    small_arena.insert(Default::default());
                }
                let small_idx = small_arena.entries().map(|pair| pair.0).next().unwrap();
                b.iter(|| lookup(&small_arena, &small_idx, *n))
            },
            (1..3).map(|n| n * 100).collect::<Vec<usize>>(),
        )
        .throughput(|n| Throughput::Elements(*n as u64)),
    );

    c.bench(
        "lookup",
        ParameterizedBenchmark::new(
            "lookup-big",
            |b, n| {
                let mut big_arena = Arena::<Big>::new();
                for _ in 0..1024 {
                    big_arena.insert(Default::default());
                }
                let big_idx = big_arena.entries().map(|pair| pair.0).next().unwrap();
                b.iter(|| lookup(&big_arena, &big_idx, *n))
            },
            (1..3).map(|n| n * 100).collect::<Vec<usize>>(),
        )
        .throughput(|n| Throughput::Elements(*n as u64)),
    );

    c.bench(
        "collect",
        ParameterizedBenchmark::new(
            "collect-small",
            |b, n| {
                let mut small_arena = Arena::<Small>::new();
                for _ in 0..1024 {
                    small_arena.insert(Default::default());
                }
                b.iter(|| collect(&small_arena, *n))
            },
            (1..3).map(|n| n * 100).collect::<Vec<usize>>(),
        )
        .throughput(|n| Throughput::Elements(*n as u64)),
    );

    c.bench(
        "collect",
        ParameterizedBenchmark::new(
            "collect-big",
            |b, n| {
                let mut big_arena = Arena::<Big>::new();
                for _ in 0..1024 {
                    big_arena.insert(Default::default());
                }
                b.iter(|| collect(&big_arena, *n))
            },
            (1..3).map(|n| n * 100).collect::<Vec<usize>>(),
        )
        .throughput(|n| Throughput::Elements(*n as u64)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
