use iai::black_box;
use streemap::Rect;

const R0F: Rect<f32> = Rect { x: 0., y: 0., w: 0., h: 0. };
const SLICEF: [(f32, Rect<f32>); 7] =
    [(6., R0F), (6., R0F), (4., R0F), (3., R0F), (2., R0F), (2., R0F), (1., R0F)];
const RECTF: Rect<f32> = Rect { x: 0., y: 0., w: 6., h: 4. };

fn baseline() {
    let mut slice = SLICEF;
    black_box(RECTF);
    black_box(&mut slice);
}

fn dice() {
    let mut slice = SLICEF;
    streemap::dice(
        black_box(RECTF),
        black_box(&mut slice),
        |&(n, _)| n,
        |(_, item_r), r| *item_r = r,
    )
}

fn slice() {
    let mut slice = SLICEF;
    streemap::slice(
        black_box(RECTF),
        black_box(&mut slice),
        |&(n, _)| n,
        |(_, item_r), r| *item_r = r,
    )
}

fn binary() {
    let mut slice = SLICEF;
    streemap::binary(
        black_box(RECTF),
        black_box(&mut slice),
        |&(n, _)| n,
        |(_, item_r), r| *item_r = r,
    )
}

fn squarify() {
    let mut slice = SLICEF;
    streemap::squarify(
        black_box(RECTF),
        black_box(&mut slice),
        |&(n, _)| n,
        |(_, item_r), r| *item_r = r,
    )
}

fn ordered_pivot_by_middle() {
    let mut slice = SLICEF;
    streemap::ordered_pivot_by_middle(
        black_box(RECTF),
        black_box(&mut slice),
        |&(n, _)| n,
        |(_, item_r), r| *item_r = r,
    )
}

fn ordered_pivot_by_size() {
    let mut slice = SLICEF;
    streemap::ordered_pivot_by_size(
        black_box(RECTF),
        black_box(&mut slice),
        |&(n, _)| n,
        |(_, item_r), r| *item_r = r,
    )
}

iai::main!(baseline, dice, slice, binary, squarify, ordered_pivot_by_middle, ordered_pivot_by_size);
