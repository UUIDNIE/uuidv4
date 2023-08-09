use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use uuid::Uuid;

fn uuid_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("uuid_v4");
    group.measurement_time(std::time::Duration::new(60, 0));  // 60 seconds
    
    group.throughput(Throughput::Elements(1));
    group.bench_function("uuid_v4", |b| b.iter(|| Uuid::new_v4()));
    
    group.finish();
}

criterion_group!(benches, uuid_benchmark);
criterion_main!(benches);

