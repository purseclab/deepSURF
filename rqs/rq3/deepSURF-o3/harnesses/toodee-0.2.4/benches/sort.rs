use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion, Throughput, BatchSize};
use toodee::{TooDee, TooDeeOpsMut, SortOps};
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

fn sort_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort");
    for &size in [100usize, 200, 300, 400].iter() {
        
        group.throughput(Throughput::Elements(size as u64));
        
        
        // sort_by_row
        {
            let toodee = new_rnd_toodee(100, size);
            group.bench_with_input(BenchmarkId::new("sort_by_row", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(), |mut data| data.sort_by_row(size / 2, |a, b| a.cmp(b)), BatchSize::LargeInput)
            });
            group.bench_with_input(BenchmarkId::new("view_sort_by_row", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(), |mut data| data.view_mut((0, 0), (100, size)).sort_by_row(50, |a, b| a.cmp(b)), BatchSize::LargeInput)
            });
            group.bench_with_input(BenchmarkId::new("sort_unstable_by_row", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(), |mut data| data.sort_unstable_by_row(size / 2, |a, b| a.cmp(b)), BatchSize::LargeInput)
            });
        }

        // sort_by_col
        {
            let toodee = new_rnd_toodee(size, 100);
            group.bench_with_input(BenchmarkId::new("sort_by_col", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(), |mut data| data.sort_by_col(size / 2, |a, b| a.cmp(b)), BatchSize::LargeInput)
            });
            group.bench_with_input(BenchmarkId::new("view_sort_by_col", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(), |mut data| data.view_mut((0, 0), (size, 100)).sort_by_col(50, |a, b| a.cmp(b)), BatchSize::LargeInput)
            });
            group.bench_with_input(BenchmarkId::new("sort_unstable_by_col", size), &size, |b, _| {
                b.iter_batched(|| toodee.clone(), |mut data| data.sort_unstable_by_col(size / 2, |a, b| a.cmp(b)), BatchSize::LargeInput)
            });
        }
    }
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
