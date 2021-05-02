use criterion::{black_box, criterion_group, criterion_main, Criterion};
use streemap::*;

fn criterion_benchmark(c: &mut Criterion) {
    const R0: Rect<f32> = Rect { x: 0., y: 0., w: 0., h: 0. };
    let mut slice_f32 = [(6., R0), (6., R0), (4., R0), (3., R0), (2., R0), (2., R0), (1., R0)];
    const R1: Rect<i32> = Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut slice_i32 = [(6, R1), (6, R1), (4, R1), (3, R1), (2, R1), (2, R1), (1, R1)];
    const R2: Rect<f64> = Rect { x: 0., y: 0., w: 0., h: 0. };
    let mut slice_f64 = [(6., R2), (6., R2), (4., R2), (3., R2), (2., R2), (2., R2), (1., R2)];
    const R3: Rect<i64> = Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut slice_i64 = [(6, R3), (6, R3), (4, R3), (3, R3), (2, R3), (2, R3), (1, R3)];

    c.bench_function("squarify f32", |b| {
        b.iter(|| {
            squarify(
                Rect { x: 0., y: 0., w: 6., h: 4. },
                black_box(&mut slice_f32[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });

    c.bench_function("squarify_scaled f32", |b| {
        b.iter(|| {
            squarify_scaled(
                Rect { x: 0., y: 0., w: 6., h: 4. },
                black_box(&mut slice_f32[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });

    c.bench_function("squarify i32", |b| {
        b.iter(|| {
            squarify(
                Rect { x: 0, y: 0, w: 6, h: 4 },
                black_box(&mut slice_i32[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });

    c.bench_function("squarify_scaled i32", |b| {
        b.iter(|| {
            squarify_scaled(
                Rect { x: 0, y: 0, w: 6, h: 4 },
                black_box(&mut slice_i32[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });

    c.bench_function("squarify f64", |b| {
        b.iter(|| {
            squarify(
                Rect { x: 0., y: 0., w: 6., h: 4. },
                black_box(&mut slice_f64[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });

    c.bench_function("squarify_scaled f64", |b| {
        b.iter(|| {
            squarify_scaled(
                Rect { x: 0., y: 0., w: 6., h: 4. },
                black_box(&mut slice_f64[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });

    c.bench_function("squarify i64", |b| {
        b.iter(|| {
            squarify(
                Rect { x: 0, y: 0, w: 6, h: 4 },
                black_box(&mut slice_i64[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });

    c.bench_function("squarify_scaled i64", |b| {
        b.iter(|| {
            squarify_scaled(
                Rect { x: 0, y: 0, w: 6, h: 4 },
                black_box(&mut slice_i64[..]),
                |&(n, _)| n,
                |(_, item_r), r| *item_r = r,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
