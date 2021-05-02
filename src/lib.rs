//! # Set of tree map algorithms
//!
//! Algorithms:
//!
//! - Slice and Dice 😵.
//! - Binary.
//! - [Squarified](https://www.win.tue.nl/~vanwijk/stm.pdf) by Bruls, Mark; Huizing, Kees; van Wijk, Jarke J. (2000).
//!
//! Example:
//!
//! ```rust
//! use streemap::Rect;
//!
//! const R0: Rect<f32> = Rect { x: 0., y: 0., w: 0., h: 0. };
//! let mut slice = [(6., R0), (6., R0), (4., R0), (3., R0), (2., R0), (2., R0), (1., R0)];
//! streemap::squarify(
//!     Rect { x: 0., y: 0., w: 6., h: 4. },
//!     &mut slice[..],
//!     |&(n, _)| n, // map item to item size
//!     |(_, item_r), r| *item_r = r, // set item distributed rect
//! );
//! assert_eq!(
//!     slice,
//!     [
//!         (6.0, Rect { x: 0.0, y: 0.0, w: 3.0, h: 2.0 }),
//!         (6.0, Rect { x: 0.0, y: 2.0, w: 3.0, h: 2.0 }),
//!         (4.0, Rect { x: 3.0, y: 0.0, w: 1.7142857, h: 2.3333333 }),
//!         (3.0, Rect { x: 4.714286, y: 0.0, w: 1.2857141, h: 2.3333333 }),
//!         (2.0, Rect { x: 3.0, y: 2.3333333, w: 1.1999999, h: 1.6666667 }),
//!         (2.0, Rect { x: 4.2, y: 2.3333333, w: 1.1999999, h: 1.6666667 }),
//!         (1.0, Rect { x: 5.3999996, y: 2.3333333, w: 0.60000014, h: 1.6666667 })
//!     ]
//! );
//! streemap::binary(
//!     Rect { x: 0., y: 0., w: 6., h: 4. },
//!     &mut slice[..],
//!     |&(n, _)| n, // map item to item size
//!     |(_, item_r), r| *item_r = r, // set item distributed rect
//! );
//! assert_eq!(
//!     slice,
//!     [
//!         (6.0, Rect { x: 0.0, y: 0.0, w: 3.0, h: 2.0 }),
//!         (6.0, Rect { x: 0.0, y: 2.0, w: 3.0, h: 2.0 }),
//!         (4.0, Rect { x: 3.0, y: 0.0, w: 3.0, h: 1.3333334 }),
//!         (3.0, Rect { x: 3.0, y: 1.3333334, w: 1.125, h: 2.6666665 }),
//!         (2.0, Rect { x: 4.125, y: 1.3333334, w: 1.875, h: 1.0666667 }),
//!         (2.0, Rect { x: 4.125, y: 2.4, w: 1.25, h: 1.5999999 }),
//!         (1.0, Rect { x: 5.375, y: 2.4, w: 0.625, h: 1.5999999 })
//!     ]
//! );
//! ```
//!
use std::cmp::Ordering;
use std::iter::Sum;

use num_traits::{NumAssignOps, NumOps, One, Zero};

/// A simple rect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<N> {
    pub x: N,
    pub y: N,
    pub w: N,
    pub h: N,
}

impl<N> Rect<N>
where
    N: Zero,
{
    /// Create a Rect at the origin from its size
    pub fn from_size(w: N, h: N) -> Self {
        Rect { x: N::zero(), y: N::zero(), w, h }
    }
}

impl<N> Rect<N>
where
    N: NumOps + Copy,
{
    /// Flip this rect horizontally within a container of height `h`
    #[inline]
    pub fn flip_h(&mut self, container_h: N) {
        self.x = container_h - self.x - self.w;
    }

    /// Flip this rect vertically within a container of width `w`
    #[inline]
    pub fn flip_v(&mut self, container_w: N) {
        self.y = container_w - self.y - self.h;
    }
}

/// Compute the ratio (numer / denom) of an item.
///
/// `size_item` is the item size.
/// `size_total` is the item container total size.
/// `side_squared` is item container side length already squared.
///
/// __Complexity__: `O(1)`
fn ratio<N>(side_squared: N, size_total: N, size_item: N) -> (N, N)
where
    N: NumOps + PartialOrd + Copy,
{
    let a = size_total * size_total;
    let b = side_squared * size_item;
    if a >= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Compute the scale to apply to item sizes for them to fit inside `rect`
///
/// __Complexity__: `O(items.len())`
fn scale<N, T, S>(rect: Rect<N>, items: &[T], f_item_size: S) -> N
where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy + Sum,
    S: Fn(&T) -> N,
{
    let rect_size = rect.w * rect.h;
    let size_total = items.iter().map(|x| f_item_size(x)).sum();
    let scale = rect_size / size_total;
    scale
}

/// Distribute `items` inside `rect` vertically without checking if they fit perfectly.
///
/// - `f_item_size` provide the size of an item
/// - `f_item_set_rect` receive the item distributed Rect.
///   Called once for each item and in a stable order.
///
/// __Complexity__: `O(items.len())`
fn _slice<N, T, S, R>(rect: Rect<N>, items: &mut [T], f_item_size: S, mut f_item_set_rect: R)
where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy,
    S: Fn(&T) -> N,
    R: FnMut(&mut T, Rect<N>),
{
    let mut y = rect.y;
    let mut it = items.iter_mut();
    while let Some(item) = it.next() {
        let size_item = f_item_size(item);
        let rect_item = Rect {
            x: rect.x,
            y,
            w: rect.w,
            h: if it.len() > 0 { size_item / rect.w } else { rect.h - (y - rect.y) },
        };
        y += rect_item.h;
        f_item_set_rect(item, rect_item);
    }
}

/// Distribute `items` inside `rect` vertically.
///
/// - `f_item_size` provide the size of an item
/// - `f_item_set_rect` receive the item distributed Rect.
///   Called once for each item and in a stable order.
///
/// __Complexity__: `O(2⨯items.len())`
pub fn slice<N, T, S, R>(rect: Rect<N>, items: &mut [T], f_item_size: S, f_item_set_rect: R)
where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy + Sum,
    S: Fn(&T) -> N,
    R: FnMut(&mut T, Rect<N>),
{
    let scale = scale(rect, items, &f_item_size);
    _slice(rect, items, |item| f_item_size(item) * scale, f_item_set_rect);
}

/// Distribute `items` inside `rect` horizontally without checking if they fit perfectly.
///
/// - `f_item_size` provide the size of an item
/// - `f_item_set_rect` receive the item distributed Rect.
///   Called once for each item and in a stable order.
///
/// __Complexity__: `O(items.len())`
fn _dice<N, T, S, R>(rect: Rect<N>, items: &mut [T], f_item_size: S, mut f_item_set_rect: R)
where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy,
    S: Fn(&T) -> N,
    R: FnMut(&mut T, Rect<N>),
{
    let mut x = rect.x;
    let mut it = items.iter_mut();
    while let Some(item) = it.next() {
        let size_item = f_item_size(item);
        let rect_item = Rect {
            x,
            y: rect.y,
            w: if it.len() > 0 { size_item / rect.h } else { rect.w - (x - rect.x) },
            h: rect.h,
        };
        x += rect_item.w;
        f_item_set_rect(item, rect_item);
    }
}

/// Distribute `items` inside `rect` horizontally.
///
/// - `f_item_size` provide the size of an item
/// - `f_item_set_rect` receive the item distributed Rect.
///   Called once for each item and in a stable order.
///
/// __Complexity__: `O(2⨯items.len())`
pub fn dice<N, T, S, R>(rect: Rect<N>, items: &mut [T], f_item_size: S, f_item_set_rect: R)
where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy + Sum,
    S: Fn(&T) -> N,
    R: FnMut(&mut T, Rect<N>),
{
    let scale = scale(rect, items, &f_item_size);
    _dice(rect, items, |item| f_item_size(item) * scale, f_item_set_rect);
}

fn _binary<N, T, R>(
    rect: Rect<N>,
    items: &mut [T],
    f_item_set_rect: &mut R,
    sums: &[N],
    offset: N,
    value: N,
) where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy,
    R: FnMut(&mut T, Rect<N>),
{
    if items.is_empty() || value.is_zero() {
        return;
    } else if items.len() == 1 {
        f_item_set_rect(&mut items[0], rect);
        return;
    }

    let target = value / (N::one() + N::one()) + offset;
    let mid = sums
        .binary_search_by(|&p| if p > target { Ordering::Greater } else { Ordering::Less })
        .unwrap_or_else(|x| if x == 0 { 1 } else { x });
    let left = sums[mid - 1] - offset;
    let right = value - left;
    let (lrect, rrect) = if rect.w > rect.h {
        let xe = rect.x + rect.w;
        let xm = (rect.x * right + xe * left) / value;
        (Rect { w: xm - rect.x, ..rect }, Rect { x: xm, w: xe - xm, ..rect })
    } else {
        let ye = rect.y + rect.h;
        let ym = (rect.y * right + ye * left) / value;
        (Rect { h: ym - rect.y, ..rect }, Rect { y: ym, h: ye - ym, ..rect })
    };
    _binary(lrect, &mut items[0..mid], f_item_set_rect, &sums[0..mid], offset, left);
    _binary(rrect, &mut items[mid..], f_item_set_rect, &sums[mid..], sums[mid - 1], right);
}

/// Distribute `items` inside `rect` by splitting it recursively in 2 areas close to the same sizes.
///
/// - `f_item_size` provide the size of an item
/// - `f_item_set_rect` receive the item distributed Rect.
///   Called once for each item and in a stable order.
///
/// To maximize the output quality its best to sort items by size in descending order.
///
/// __Complexity__: `O(3⨯items.len()⨯log(items.len()))`
pub fn binary<N, T, S, R>(rect: Rect<N>, items: &mut [T], f_item_size: S, mut f_item_set_rect: R)
where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy,
    S: Fn(&T) -> N,
    R: FnMut(&mut T, Rect<N>),
{
    let mut size_total = N::zero();
    let sums: Vec<N> = items
        .iter()
        .map(|item| {
            let item_size = f_item_size(item);
            size_total += item_size;
            size_total
        })
        .collect();
    _binary(rect, items, &mut f_item_set_rect, sums.as_slice(), N::zero(), size_total);
}

/// Distribute `items` inside `rect` while trying to get the aspect ratio as close
/// to 1 as possible without checking is they fit.
///
/// - `f_item_size` provide the size of an item
/// - `f_item_set_rect` receive the item distributed Rect.
///   Called once for each item and in a stable order.
///
/// __Complexity__: `O(2⨯items.len())`
fn _squarify<N, T, S, R>(
    mut rect: Rect<N>,
    mut items: &mut [T],
    f_item_size: S,
    mut f_item_set_rect: R,
) where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy,
    S: Fn(&T) -> N,
    R: FnMut(&mut T, Rect<N>),
{
    while !items.is_empty() {
        let is_wide = rect.w > rect.h;
        let side = if is_wide { rect.h } else { rect.w };
        let mut split_side = if is_wide { rect.w } else { rect.h };
        let side_squared = side * side;
        let mut size_total0 = N::zero();
        let (mut numer0, mut denom0) = (N::one(), N::zero());
        let split_idx = items
            .iter()
            .position(|item| {
                let size_item = f_item_size(item);
                let size_total1 = size_total0 + size_item;

                let (numer1, denom1) = ratio(side_squared, size_total1, size_item);
                let split = numer1 * denom0 > numer0 * denom1;
                if split {
                    split_side = size_total0 / side;
                }
                size_total0 = size_total1;
                numer0 = numer1;
                denom0 = denom1;

                split
            })
            .unwrap_or(items.len());
        let (head, tail) = items.split_at_mut(split_idx);
        items = tail;
        if is_wide {
            let w = rect.w - split_side;
            rect.w = split_side;
            _slice(rect, head, &f_item_size, &mut f_item_set_rect);
            rect.w = w;
            rect.x += split_side;
        } else {
            let h = rect.h - split_side;
            rect.h = split_side;
            _dice(rect, head, &f_item_size, &mut f_item_set_rect);
            rect.h = h;
            rect.y += split_side;
        };
    }
}

/// Distribute `items` inside `rect` while trying to get the aspect ratio as close
/// to 1 as possible.
///
/// - `f_item_size` provide the size of an item
/// - `f_item_set_rect` receive the item distributed Rect.
///   Called once for each item and in a stable order.
///
/// To maximize the output quality its best to sort items by size in descending order.
///
/// __Complexity__: `O(3⨯items.len())`
pub fn squarify<N, T, S, R>(rect: Rect<N>, items: &mut [T], f_item_size: S, f_item_set_rect: R)
where
    N: NumAssignOps + NumOps + PartialOrd + Zero + One + Copy + Sum,
    S: Fn(&T) -> N,
    R: FnMut(&mut T, Rect<N>),
{
    let scale = scale(rect, items, &f_item_size);
    _squarify(rect, items, |item| f_item_size(item) * scale, f_item_set_rect);
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    fn svg<N: NumOps + Copy + Display>(
        view_box: Rect<N>,
        slice: &[(usize, N, Rect<N>)],
        scale: N,
    ) -> String {
        use std::fmt::Write;

        let mut f = String::new();
        writeln!(
            &mut f,
            r#"<svg viewBox="{} {} {} {}" width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <radialGradient id="g" cx="0.5" cy="0.5" r="0.5"
    fx="0.75" fy="0.6" fr="5%" gradientTransform="scale(2) translate(-0.25, -0.25)">
      <stop offset="0%" stop-color="white"/>
      <stop offset="100%" stop-color="darkseagreen"/>
    </radialGradient>
  </defs>"#,
            view_box.x,
            view_box.y,
            view_box.w,
            view_box.h,
            view_box.w * scale,
            view_box.h * scale
        )
        .unwrap();
        for (_i, _size, r) in slice {
            writeln!(
                &mut f,
                r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="url(#g)" />"#,
                r.x, r.y, r.w, r.h
            )
            .unwrap();
        }
        writeln!(&mut f, "</svg>").unwrap();

        f
    }

    fn mkslice<N: Copy + Zero>(slice: &[N]) -> Vec<(usize, N, Rect<N>)> {
        slice
            .iter()
            .copied()
            .enumerate()
            .map(|(i, n)| (i, n, Rect::from_size(N::zero(), N::zero())))
            .collect()
    }

    fn mkset_rect<N>() -> impl FnMut(&mut (usize, N, Rect<N>), Rect<N>) {
        let mut idx = 0;
        move |(i, _n, item_r), r| {
            assert_eq!(*i, idx, "f_item_set_rect must be called in stable order");
            *item_r = r;
            idx += 1;
        }
    }

    #[test]
    fn binary_f32() {
        let mut slice = mkslice::<f32>(&[6., 6., 4., 3., 2., 2., 1.]);
        binary(Rect { x: 0., y: 0., w: 6., h: 4. }, &mut slice[..], |&(_, n, _)| n, mkset_rect());
        assert_eq!(
            slice,
            [
                (0, 6.0, Rect { x: 0.0, y: 0.0, w: 3.0, h: 2.0 }),
                (1, 6.0, Rect { x: 0.0, y: 2.0, w: 3.0, h: 2.0 }),
                (2, 4.0, Rect { x: 3.0, y: 0.0, w: 3.0, h: 1.3333334 }),
                (3, 3.0, Rect { x: 3.0, y: 1.3333334, w: 1.125, h: 2.6666665 }),
                (4, 2.0, Rect { x: 4.125, y: 1.3333334, w: 1.875, h: 1.0666667 }),
                (5, 2.0, Rect { x: 4.125, y: 2.4, w: 1.25, h: 1.5999999 }),
                (6, 1.0, Rect { x: 5.375, y: 2.4, w: 0.625, h: 1.5999999 })
            ]
        );
        eprintln!(
            "<!-- binary -->\n{}",
            svg(Rect { x: 0., y: 0., w: 6., h: 4. }, &slice[..], 50.0)
        );

        let mut slice = mkslice::<f32>(&[12., 12., 8., 6., 4., 4., 2.]);
        binary(Rect { x: 0., y: 0., w: 6., h: 4. }, &mut slice[..], |&(_, n, _)| n, mkset_rect());
        assert_eq!(
            slice,
            [
                (0, 12.0, Rect { x: 0.0, y: 0.0, w: 3.0, h: 2.0 }),
                (1, 12.0, Rect { x: 0.0, y: 2.0, w: 3.0, h: 2.0 }),
                (2, 8.0, Rect { x: 3.0, y: 0.0, w: 3.0, h: 1.3333334 }),
                (3, 6.0, Rect { x: 3.0, y: 1.3333334, w: 1.125, h: 2.6666665 }),
                (4, 4.0, Rect { x: 4.125, y: 1.3333334, w: 1.875, h: 1.0666667 }),
                (5, 4.0, Rect { x: 4.125, y: 2.4, w: 1.25, h: 1.5999999 }),
                (6, 2.0, Rect { x: 5.375, y: 2.4, w: 0.625, h: 1.5999999 })
            ]
        );
    }

    #[test]
    fn squarify_paper_example_f32() {
        let mut slice = mkslice::<f32>(&[6., 6., 4., 3., 2., 2., 1.]);
        _squarify(
            Rect { x: 0., y: 0., w: 6., h: 4. },
            &mut slice[..],
            |&(_, n, _)| n,
            mkset_rect(),
        );
        assert_eq!(
            slice,
            [
                (0, 6.0, Rect { x: 0.0, y: 0.0, w: 3.0, h: 2.0 }),
                (1, 6.0, Rect { x: 0.0, y: 2.0, w: 3.0, h: 2.0 }),
                (2, 4.0, Rect { x: 3.0, y: 0.0, w: 1.7142857, h: 2.3333333 }),
                (3, 3.0, Rect { x: 4.714286, y: 0.0, w: 1.2857141, h: 2.3333333 }),
                (4, 2.0, Rect { x: 3.0, y: 2.3333333, w: 1.1999999, h: 1.6666667 }),
                (5, 2.0, Rect { x: 4.2, y: 2.3333333, w: 1.1999999, h: 1.6666667 }),
                (6, 1.0, Rect { x: 5.3999996, y: 2.3333333, w: 0.60000014, h: 1.6666667 })
            ]
        );
        eprintln!(
            "<!-- squarify -->\n{}",
            svg(Rect { x: 0., y: 0., w: 6., h: 4. }, &slice[..], 50.0)
        );
        squarify(
            Rect { x: 0., y: 0., w: 12., h: 8. },
            &mut slice[..],
            |&(_, n, _)| n,
            mkset_rect(),
        );
        assert_eq!(
            slice,
            [
                (0, 6.0, Rect { x: 0.0, y: 0.0, w: 6.0, h: 4.0 }),
                (1, 6.0, Rect { x: 0.0, y: 4.0, w: 6.0, h: 4.0 }),
                (2, 4.0, Rect { x: 6.0, y: 0.0, w: 3.4285715, h: 4.6666665 }),
                (3, 3.0, Rect { x: 9.428572, y: 0.0, w: 2.5714283, h: 4.6666665 }),
                (4, 2.0, Rect { x: 6.0, y: 4.6666665, w: 2.3999999, h: 3.3333335 }),
                (5, 2.0, Rect { x: 8.4, y: 4.6666665, w: 2.3999999, h: 3.3333335 }),
                (6, 1.0, Rect { x: 10.799999, y: 4.6666665, w: 1.2000003, h: 3.3333335 })
            ]
        );
        squarify(
            Rect { x: 1., y: 2., w: 12., h: 8. },
            &mut slice[..],
            |&(_, n, _)| n,
            mkset_rect(),
        );
        assert_eq!(
            slice,
            [
                (0, 6.0, Rect { x: 1.0, y: 2.0, w: 6.0, h: 4.0 }),
                (1, 6.0, Rect { x: 1.0, y: 6.0, w: 6.0, h: 4.0 }),
                (2, 4.0, Rect { x: 7.0, y: 2.0, w: 3.4285715, h: 4.6666665 }),
                (3, 3.0, Rect { x: 10.428572, y: 2.0, w: 2.5714283, h: 4.6666665 }),
                (4, 2.0, Rect { x: 7.0, y: 6.6666665, w: 2.3999999, h: 3.3333335 }),
                (5, 2.0, Rect { x: 9.4, y: 6.6666665, w: 2.3999999, h: 3.3333335 }),
                (6, 1.0, Rect { x: 11.799999, y: 6.6666665, w: 1.2000003, h: 3.3333335 })
            ]
        );
    }
}
