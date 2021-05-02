use std::iter::Sum;

use criterion::measurement::Measurement;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};
use num_traits::{AsPrimitive, NumAssignOps, NumOps, One, Zero};
use streemap::Rect;

fn mkslice<T: AsPrimitive<N>, N: Copy + Zero + 'static>(slice: &[T]) -> Vec<(N, Rect<N>)> {
    slice.iter().copied().map(|n| (n.as_(), Rect::from_size(N::zero(), N::zero()))).collect()
}

fn s<N: Copy>() -> impl Copy + Fn(&(N, Rect<N>)) -> N {
    |&(n, _)| n
}
fn r<N: Copy>() -> impl Copy + Fn(&mut (N, Rect<N>), Rect<N>) {
    |(_, item_r), r| *item_r = r
}

fn bench_function<N, M, F, S, R>(
    g: &mut BenchmarkGroup<'_, M>,
    id: &str,
    slice: &[f32],
    f: F,
    s: S,
    r: R,
) where
    M: Measurement,
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy + Sum + 'static,
    f32: AsPrimitive<N>,
    F: Fn(Rect<N>, &mut [(N, Rect<N>)], S, R),
    S: Copy + Fn(&(N, Rect<N>)) -> N,
    R: Copy + Fn(&mut (N, Rect<N>), Rect<N>),
{
    let mut slice_x = mkslice::<f32, N>(slice);
    g.bench_function(id, |b| {
        b.iter(|| {
            f(
                black_box(Rect { x: 0f32.as_(), y: 0f32.as_(), w: 6f32.as_(), h: 4f32.as_() }),
                black_box(&mut slice_x[..]),
                s,
                r,
            )
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let d: &[f32] = &[6., 6., 4., 3., 2., 2., 1.];

    let mut g = c.benchmark_group("treemap");
    bench_function::<f32, _, _, _, _>(&mut g, "dice f32", d, streemap::dice, s(), r());
    bench_function::<f32, _, _, _, _>(&mut g, "slice f32", d, streemap::slice, s(), r());
    bench_function::<f32, _, _, _, _>(&mut g, "binary f32", d, streemap::binary, s(), r());
    bench_function::<f32, _, _, _, _>(&mut g, "squarify f32", d, streemap::squarify, s(), r());

    bench_function::<f64, _, _, _, _>(&mut g, "dice f64", d, streemap::dice, s(), r());
    bench_function::<f64, _, _, _, _>(&mut g, "slice f64", d, streemap::slice, s(), r());
    bench_function::<f64, _, _, _, _>(&mut g, "binary f64", d, streemap::binary, s(), r());
    bench_function::<f64, _, _, _, _>(&mut g, "squarify f64", d, streemap::squarify, s(), r());

    bench_function::<i32, _, _, _, _>(&mut g, "dice i32", d, streemap::dice, s(), r());
    bench_function::<i32, _, _, _, _>(&mut g, "slice i32", d, streemap::slice, s(), r());
    bench_function::<i32, _, _, _, _>(&mut g, "binary i32", d, streemap::binary, s(), r());
    bench_function::<i32, _, _, _, _>(&mut g, "squarify i32", d, streemap::squarify, s(), r());

    bench_function::<i64, _, _, _, _>(&mut g, "dice i64", d, streemap::dice, s(), r());
    bench_function::<i64, _, _, _, _>(&mut g, "slice i64", d, streemap::slice, s(), r());
    bench_function::<i64, _, _, _, _>(&mut g, "binary i64", d, streemap::binary, s(), r());
    bench_function::<i64, _, _, _, _>(&mut g, "squarify i64", d, streemap::squarify, s(), r());
    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
