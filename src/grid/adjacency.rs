use num::{CheckedAdd, CheckedSub, One, PrimInt};

use crate::grid::GridPoint;

/// An iterator that produces up to four grid points which are orthogonally
/// (horizontally or vertically) adjacent to the `center` grid point.
pub struct OrthAdjacent<T>
where
    T: PrimInt,
{
    pub center: GridPoint<T>,
    count: usize,
}

impl<T> OrthAdjacent<T>
where
    T: PrimInt,
{
    /// Iterator of grid points orthogonally adjacent to the `center`.
    pub fn new(center: GridPoint<T>) -> Self {
        OrthAdjacent { center, count: 0 }
    }

    /// Subsequence of grid points bounded by the rectangular region
    /// described by two extremes: `min_point` and `max_point`.
    pub fn within_region(
        self,
        min_point: GridPoint<T>,
        max_point: GridPoint<T>,
    ) -> impl Iterator<Item = GridPoint<T>> {
        self.filter(move |p| {
            min_point.0 <= p.0 && p.0 <= max_point.0 && min_point.1 <= p.1 && p.1 <= max_point.1
        })
    }
}

impl OrthAdjacent<usize> {
    /// Subsequence of grid points bounded by the rectangular `shape`.
    pub fn within_shape(self, shape: GridPoint<usize>) -> impl Iterator<Item = GridPoint<usize>> {
        self.filter(move |p| p.0 < shape.0 && p.1 < shape.1)
    }
}

impl<T> Iterator for OrthAdjacent<T>
where
    T: PrimInt,
{
    type Item = GridPoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((dx, dy)) = ORTH_NEIGHBORS.get(self.count).copied() {
            self.count += 1;
            let x = match add_step(self.center.0, dx) {
                Some(result) => result,
                None => continue,
            };
            let y = match add_step(self.center.1, dy) {
                Some(result) => result,
                None => continue,
            };
            return Some((x, y));
        }
        None
    }
}

/// An iterator that produces up to eight grid points which are a king's move away from the `center`
/// (i.e. horizontally, vertically, or diagonally adjacent grid points).
pub struct KingAdjacent<T>
where
    T: PrimInt,
{
    pub center: GridPoint<T>,
    count: usize,
}

impl<T> KingAdjacent<T>
where
    T: PrimInt,
{
    /// Iterator of grid points which are a king's move away from the `center`.
    pub fn new(center: GridPoint<T>) -> Self {
        KingAdjacent { center, count: 0 }
    }

    /// Subsequence of grid points bounded by the rectangular region
    /// described by two extremes: `min_point` and `max_point`.
    pub fn within_region(
        self,
        min_point: GridPoint<T>,
        max_point: GridPoint<T>,
    ) -> impl Iterator<Item = GridPoint<T>> {
        self.filter(move |p| {
            min_point.0 <= p.0 && p.0 <= max_point.0 && min_point.1 <= p.1 && p.1 <= max_point.1
        })
    }
}

impl KingAdjacent<usize> {
    /// Subsequence of grid points bounded by the rectangular `shape`.
    pub fn within_shape(self, shape: GridPoint<usize>) -> impl Iterator<Item = GridPoint<usize>> {
        self.filter(move |p| p.0 < shape.0 && p.1 < shape.1)
    }
}

impl<T> Iterator for KingAdjacent<T>
where
    T: PrimInt,
{
    type Item = GridPoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((dx, dy)) = KING_NEIGHBORS.get(self.count).copied() {
            self.count += 1;
            let x = match add_step(self.center.0, dx) {
                Some(result) => result,
                None => continue,
            };
            let y = match add_step(self.center.1, dy) {
                Some(result) => result,
                None => continue,
            };
            return Some((x, y));
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Zero,
    Add1,
    Sub1,
}

fn add_step<T>(lhs: T, rhs: Step) -> Option<T>
where
    T: CheckedAdd + CheckedSub + One,
{
    match rhs {
        Step::Zero => Some(lhs),
        Step::Add1 => lhs.checked_add(&T::one()),
        Step::Sub1 => lhs.checked_sub(&T::one()),
    }
}

static ORTH_NEIGHBORS: [GridPoint<Step>; 4] = [
    (Step::Sub1, Step::Zero),
    (Step::Add1, Step::Zero),
    (Step::Zero, Step::Sub1),
    (Step::Zero, Step::Add1),
];

static KING_NEIGHBORS: [GridPoint<Step>; 8] = [
    (Step::Sub1, Step::Zero),
    (Step::Sub1, Step::Add1),
    (Step::Zero, Step::Add1),
    (Step::Add1, Step::Add1),
    (Step::Add1, Step::Zero),
    (Step::Add1, Step::Sub1),
    (Step::Zero, Step::Sub1),
    (Step::Sub1, Step::Sub1),
];
