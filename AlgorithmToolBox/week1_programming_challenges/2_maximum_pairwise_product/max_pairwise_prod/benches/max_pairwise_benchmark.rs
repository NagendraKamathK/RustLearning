use max_pairwise_prod::max_pair;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
fn max_pairwise_benchmark(criterion: &mut Criterion) {
    let arr=black_box(
        vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]
    );
    criterion.bench_function("max_pairwise_benchmark",
                             |b| b.iter(|| max_pair::max_product_fastest(&arr)));
}

criterion_group!(benches, max_pairwise_benchmark);
criterion_main!(benches);