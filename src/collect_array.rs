//! Implements a blanket trait extension for [`Iterator`] trait which adds methods
//! to collect items from an iterator into a constant-sized array.
//! - TODO: Implement this for tuples of lower numbered elements as well as arrays.
use anyhow::{anyhow, ensure};

/// Trait extension for [`Iterator`] trait which add two of the following methods
/// to collect items from an iterator into a constant-sized array:
/// -  [`try_collect_exact_array`](CollectArray::collect_exact_array)
/// -  [`try_collect_trunc_array`](CollectArray::collect_trunc_array)
pub trait CollectArray: Iterator {
    /// Collects all items from the iterator into a constant-sized array.
    /// Too few or too many items produced will result in an error.
    fn collect_exact_array<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        collect_array(self, true)
    }

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few items produced will result in an error.
    /// However, extraneous items produced by the iterator will not be collected,
    /// though it is possible that some extra items have been consumed in the process.
    fn collect_trunc_array<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        collect_array(self, false)
    }
}

fn collect_array<T, I, const SIZE: usize>(it: I, exact: bool) -> anyhow::Result<[T; SIZE]>
where
    I: IntoIterator<Item = T>,
{
    let mut collected: Vec<T> = Vec::with_capacity(SIZE);
    for item in it.into_iter() {
        if collected.len() >= SIZE {
            ensure!(
                !exact,
                "iterator produces too many items, exceeding target {}",
                SIZE
            );
            break;
        }
        collected.push(item);
    }
    collected.try_into().map_err(|v: Vec<T>| {
        anyhow!(
            "iterator produces too few items, found {} out of target {}",
            v.len(),
            SIZE
        )
    })
}

impl<T: ?Sized> CollectArray for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trunc_ok() {
        assert_eq!(
            (0..5).collect_trunc_array::<i64, 5>().unwrap(),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(
            "xyz".chars().collect_trunc_array::<char, 3>().unwrap(),
            ['x', 'y', 'z']
        );
        assert_eq!(
            vec![]
                .into_iter()
                .collect_trunc_array::<usize, 0>()
                .unwrap(),
            []
        );
        assert_eq!(
            std::iter::repeat(99)
                .collect_trunc_array::<i64, 4>()
                .unwrap(),
            [99, 99, 99, 99]
        );
        assert_eq!((0..1000).collect_trunc_array::<i64, 0>().unwrap(), []);
    }

    #[test]
    fn trunc_too_few() {
        assert_eq!(
            (0..5)
                .collect_trunc_array::<i64, 6>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 5 out of target 6"
        );
        assert_eq!(
            "xyz"
                .chars()
                .collect_trunc_array::<char, 7>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 3 out of target 7"
        );
        assert_eq!(
            vec![]
                .into_iter()
                .collect_trunc_array::<usize, 8>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 0 out of target 8"
        );
    }

    #[test]
    fn exact_ok() {
        assert_eq!(
            (0..5).collect_exact_array::<i64, 5>().unwrap(),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(
            "xyz".chars().collect_exact_array::<char, 3>().unwrap(),
            ['x', 'y', 'z']
        );
        assert_eq!(
            vec![]
                .into_iter()
                .collect_exact_array::<usize, 0>()
                .unwrap(),
            []
        );
    }

    #[test]
    fn exact_too_few() {
        assert_eq!(
            (0..5)
                .collect_exact_array::<i64, 6>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 5 out of target 6"
        );
        assert_eq!(
            "xyz"
                .chars()
                .collect_exact_array::<char, 7>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 3 out of target 7"
        );
        assert_eq!(
            vec![]
                .into_iter()
                .collect_exact_array::<usize, 8>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 0 out of target 8"
        );
    }

    #[test]
    fn exact_too_many() {
        assert_eq!(
            std::iter::repeat(99)
                .collect_exact_array::<i64, 8>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 8"
        );
        assert_eq!(
            (0..1000)
                .collect_exact_array::<i64, 0>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 0"
        );
        assert_eq!(
            (0..5)
                .collect_exact_array::<i64, 4>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 4"
        );
        assert_eq!(
            "xyz"
                .chars()
                .collect_exact_array::<char, 1>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 1"
        );
    }
}
