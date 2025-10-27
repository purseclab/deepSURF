use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion, Throughput, BatchSize};
use toodee::{TooDee, TooDeeOps, TooDeeOpsMut};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;

fn new_rnd_toodee(cols: usize, rows: usize) -> TooDee<u32>
{
    let size = cols * rows;
    let rng : StdRng = SeedableRng::seed_from_u64(42);
//    let v = vec![0]
    let generator = rng.sample_iter(Uniform::from(0u32..100_000));
    TooDee::from_vec(cols, rows, generator.take(size).collect())
}

fn fill_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill");
    for dims in [(32usize, 20usize), (320, 200), (640, 480)].iter() {
        let size = dims.0 * dims.1;
        group.throughput(Throughput::Elements(size as u64));
        let mut toodee = TooDee::init(dims.0, dims.1, 0u32);
        
        group.bench_with_input(BenchmarkId::new("fill", size), &size, |b, _| {
            b.iter(|| toodee.fill(42));
        });

        let mut view = toodee.view_mut((0, 0), (dims.0, dims.1));

        group.bench_with_input(BenchmarkId::new("view_fill", size), &size, |b, _| {
            b.iter(|| view.fill(42))
        });
    }
}

fn iter_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_sum");
    for dims in [(32usize, 20usize), (320, 200), (640, 480)].iter() {
        let size = dims.0 * dims.1;
        group.throughput(Throughput::Elements(size as u64));
        let mut toodee = TooDee::init(dims.0, dims.1, 1u32);
        group.bench_with_input(BenchmarkId::new("data", size), &size, |b, _| {
            b.iter(|| black_box(toodee.data().iter().sum::<u32>()));
        });
        let view = toodee.view_mut((0, 0), (dims.0, dims.1));
        group.bench_with_input(BenchmarkId::new("cells", size), &size, |b, _| {
            b.iter(|| black_box(view.cells().sum::<u32>()));
        });
        group.bench_with_input(BenchmarkId::new("rows", size), &size, |b, _| {
            b.iter(|| black_box(view.rows().map(|r| r.iter().sum::<u32>()).sum::<u32>()));
        });
    }
}

fn iter_mut_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_mut_sum");
    for dims in [(32usize, 20usize), (320, 200), (640, 480)].iter() {
        let size = dims.0 * dims.1;
        group.throughput(Throughput::Elements(size as u64));
        let mut toodee = TooDee::init(dims.0, dims.1, 1u32);
        group.bench_with_input(BenchmarkId::new("data_mut", size), &size, |b, _| {
            b.iter(|| black_box(toodee.data_mut().iter().sum::<u32>()));
        });
        let mut view = toodee.view_mut((0, 0), (dims.0, dims.1));
        group.bench_with_input(BenchmarkId::new("cells_mut", size), &size, |b, _| {
            b.iter(|| black_box(view.cells_mut().map(|x| *x).sum::<u32>()));
        });
        group.bench_with_input(BenchmarkId::new("rows_mut", size), &size, |b, _| {
            b.iter(|| black_box(view.rows_mut().map(|r| r.iter_mut().map(|x| *x).sum::<u32>()).sum::<u32>()));
        });
    }
}

fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    for &size in [100usize, 200, 300, 400].iter() {
        
        group.throughput(Throughput::Elements((size * size) as u64));
        
        let toodee = new_rnd_toodee(size, size);
        
        let new_data = 0u32..(size as u32);
        
        // insert_row
        {
            // reserves space to exclude memory allocation from benchmark time
            group.bench_with_input(BenchmarkId::new("insert_row", size), &size, |b, _| {
                b.iter_batched(|| { let mut tmp = toodee.clone(); tmp.reserve(size); (tmp, new_data.clone()) },
                |(mut data, new_data)| data.insert_row(0, new_data), BatchSize::LargeInput)
            });
        }

        // insert_row_alloc
        {
            // force allocation by shrinking the Vec
            group.bench_with_input(BenchmarkId::new("insert_row_alloc", size), &size, |b, _| {
                b.iter_batched(|| { let mut tmp = toodee.clone(); tmp.shrink_to_fit(); (tmp, new_data.clone()) },
                |(mut data, new_data)| data.insert_row(0, new_data), BatchSize::LargeInput)
            });
        }

        // insert_col
        {
            // reserves space to exclude memory allocation from benchmark time
            group.bench_with_input(BenchmarkId::new("insert_col", size), &size, |b, _| {
                b.iter_batched(|| { let mut tmp = toodee.clone(); tmp.reserve(size); (tmp, new_data.clone()) },
                |(mut data, new_data)| data.insert_col(0, new_data), BatchSize::LargeInput)
            });
        }
        
        // insert_col_alloc
        {
            // reserves space to exclude memory allocation from benchmark time
            group.bench_with_input(BenchmarkId::new("insert_col_alloc", size), &size, |b, _| {
                b.iter_batched(|| { let mut tmp = toodee.clone(); tmp.shrink_to_fit(); (tmp, new_data.clone()) },
                |(mut data, new_data)| data.insert_col(0, new_data), BatchSize::LargeInput)
            });
        }
    }
}

fn remove_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("remove");
    for &size in [100usize, 200, 300, 400].iter() {
        
        group.throughput(Throughput::Elements((size * size) as u64));
        
        let toodee = new_rnd_toodee(size, size);
        
        // remove_row
        {
            group.bench_with_input(BenchmarkId::new("remove_row", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(),
                |mut data| { let drain = data.remove_row(0); black_box(drain.sum::<u32>()); }, BatchSize::LargeInput)
            });
        }

        // remove_col
        {
            group.bench_with_input(BenchmarkId::new("remove_col", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(),
                |mut data| { let drain = data.remove_col(0); black_box(drain.sum::<u32>()); }, BatchSize::LargeInput)
            });
        }
        
    }
}

criterion_group!(benches, fill_benchmark, iter_benchmark, iter_mut_benchmark, insert_benchmark, remove_benchmark);
criterion_main!(benches);
