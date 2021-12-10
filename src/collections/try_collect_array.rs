use anyhow::{anyhow, bail};

/// Trait extension for [`Iterator`] trait, adding the methods
/// [`try_collect_exact_array`] and [`try_collect_trunc_array`]
/// to collect items from an iterator into a constant-sized array.
///
/// [`try_collect_exact_array`]: TryCollectArray::try_collect_exact_array
/// [`try_collect_trunc_array`]: TryCollectArray::try_collect_trunc_array
pub trait TryCollectArray: Iterator {
    fn try_collect_array<T, const SIZE: usize>(self, exact: bool) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        let mut collected: Vec<T> = Vec::with_capacity(SIZE);
        for item in self.into_iter() {
            if collected.len() >= SIZE {
                if exact {
                    bail!(
                        "iterator produces too many items, exceeding target {}",
                        SIZE
                    );
                } else {
                    break;
                }
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

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few or too many items produced will result in an error.
    fn try_collect_exact_array<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        self.try_collect_array(true)
    }

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few items produced will result in an error.
    /// However, extraneous items produced by the iterator will not be collected,
    /// though it is possible that some extra items have been consumed in the process.
    fn try_collect_trunc_array<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        self.try_collect_array(false)
    }
}

impl<T: ?Sized> TryCollectArray for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trunc_ok() {
        assert_eq!(
            (0..5).try_collect_trunc_array::<i64, 5>().unwrap(),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(
            "xyz".chars().try_collect_trunc_array::<char, 3>().unwrap(),
            ['x', 'y', 'z']
        );
        assert_eq!(
            vec![]
                .into_iter()
                .try_collect_trunc_array::<usize, 0>()
                .unwrap(),
            []
        );
        assert_eq!(
            std::iter::repeat(99)
                .try_collect_trunc_array::<i64, 4>()
                .unwrap(),
            [99, 99, 99, 99]
        );
        assert_eq!((0..1000).try_collect_trunc_array::<i64, 0>().unwrap(), []);
    }

    #[test]
    fn trunc_too_few() {
        assert_eq!(
            (0..5)
                .try_collect_trunc_array::<i64, 6>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 5 out of target 6"
        );
        assert_eq!(
            "xyz"
                .chars()
                .try_collect_trunc_array::<char, 7>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 3 out of target 7"
        );
        assert_eq!(
            vec![]
                .into_iter()
                .try_collect_trunc_array::<usize, 8>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 0 out of target 8"
        );
    }

    #[test]
    fn exact_ok() {
        assert_eq!(
            (0..5).try_collect_exact_array::<i64, 5>().unwrap(),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(
            "xyz".chars().try_collect_exact_array::<char, 3>().unwrap(),
            ['x', 'y', 'z']
        );
        assert_eq!(
            vec![]
                .into_iter()
                .try_collect_exact_array::<usize, 0>()
                .unwrap(),
            []
        );
    }

    #[test]
    fn exact_too_few() {
        assert_eq!(
            (0..5)
                .try_collect_exact_array::<i64, 6>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 5 out of target 6"
        );
        assert_eq!(
            "xyz"
                .chars()
                .try_collect_exact_array::<char, 7>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 3 out of target 7"
        );
        assert_eq!(
            vec![]
                .into_iter()
                .try_collect_exact_array::<usize, 8>()
                .unwrap_err()
                .to_string(),
            "iterator produces too few items, found 0 out of target 8"
        );
    }

    #[test]
    fn exact_too_many() {
        assert_eq!(
            std::iter::repeat(99)
                .try_collect_exact_array::<i64, 8>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 8"
        );
        assert_eq!(
            (0..1000)
                .try_collect_exact_array::<i64, 0>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 0"
        );
        assert_eq!(
            (0..5)
                .try_collect_exact_array::<i64, 4>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 4"
        );
        assert_eq!(
            "xyz"
                .chars()
                .try_collect_exact_array::<char, 1>()
                .unwrap_err()
                .to_string(),
            "iterator produces too many items, exceeding target 1"
        );
    }
}
