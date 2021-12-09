use std::error::Error;
use std::fmt::{Display, Formatter};

/// Trait extensions for [`Iterator`] type, adding a method
/// to collect all items from an iterator into a constant-sized array.
pub trait TryCollectArray: Iterator {
    fn try_collect_array<T, const SIZE: usize>(
        self,
        exact: bool,
    ) -> Result<[T; SIZE], TryCollectArrayError>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        let mut buffer: Vec<T> = Vec::with_capacity(SIZE);
        for item in self.into_iter() {
            if buffer.len() >= SIZE {
                if exact {
                    return Err(TryCollectArrayError::TooManyItems { expect: SIZE });
                } else {
                    break;
                }
            }
            buffer.push(item);
        }
        buffer
            .try_into()
            .map_err(|v: Vec<T>| TryCollectArrayError::TooFewItems {
                expect: SIZE,
                found: v.len(),
            })
    }

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few items produced will result in [`TryCollectArrayError::TooFewItems`]
    /// whereas too many items will result in [`TryCollectArrayError::TooManyItems`].
    fn try_collect_exact_array<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], TryCollectArrayError>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        self.try_collect_array(true)
    }

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few items produced will result in [`TryCollectArrayError::TooFewItems`].
    /// However, extraneous items produced by the iterator will not be collected,
    /// though it is possible that some extra items have been consumed in the process.
    fn try_collect_trunc_array<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], TryCollectArrayError>
    where
        Self: Sized + IntoIterator<Item = T>,
    {
        self.try_collect_array(false)
    }
}

impl<T: ?Sized> TryCollectArray for T where T: Iterator {}

/// Error type for trait [`TryCollectArray`] indicating that
/// the number of items produced by the iterator does not match the expected.
#[derive(Debug)]
pub enum TryCollectArrayError {
    /// Error indicating that the number of items produced by the iterator is less than the expected.
    TooFewItems { expect: usize, found: usize },
    /// Error indicating that the number of items produced by the iterator exceeds the expected.
    TooManyItems { expect: usize },
}

impl Display for TryCollectArrayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TryCollectArrayError::TooFewItems {
                expect: target,
                found,
            } => write!(
                f,
                "iterator produces too few items, found {} out of target {}",
                found, target
            ),
            TryCollectArrayError::TooManyItems { expect: target } => write!(
                f,
                "iterator produces too many items, exceeding target {}",
                target
            ),
        }
    }
}

impl Error for TryCollectArrayError {}

#[cfg(test)]
mod tests {
    use matches::assert_matches;

    use super::*;

    #[test]
    fn trunc_ok() {
        assert_matches!(
            (0..5).try_collect_trunc_array::<i64, 5>(),
            Ok([0, 1, 2, 3, 4])
        );
        assert_matches!(
            "xyz".chars().try_collect_trunc_array::<char, 3>(),
            Ok(['x', 'y', 'z'])
        );
        assert_matches!(
            vec![].into_iter().try_collect_trunc_array::<usize, 0>(),
            Ok([])
        );
        assert_matches!(
            std::iter::repeat(99).try_collect_trunc_array::<i64, 4>(),
            Ok([99, 99, 99, 99])
        );
        assert_matches!((0..1000).try_collect_trunc_array::<i64, 0>(), Ok([]));
    }

    #[test]
    fn trunc_too_few() {
        assert_matches!(
            (0..5).try_collect_trunc_array::<i64, 6>(),
            Err(TryCollectArrayError::TooFewItems {
                expect: 6,
                found: 5
            })
        );
        assert_matches!(
            "xyz".chars().try_collect_trunc_array::<char, 7>(),
            Err(TryCollectArrayError::TooFewItems {
                expect: 7,
                found: 3
            })
        );
        assert_matches!(
            vec![].into_iter().try_collect_trunc_array::<usize, 8>(),
            Err(TryCollectArrayError::TooFewItems {
                expect: 8,
                found: 0
            })
        );
    }

    #[test]
    fn exact_ok() {
        assert_matches!(
            (0..5).try_collect_exact_array::<i64, 5>(),
            Ok([0, 1, 2, 3, 4])
        );
        assert_matches!(
            "xyz".chars().try_collect_exact_array::<char, 3>(),
            Ok(['x', 'y', 'z'])
        );
        assert_matches!(
            vec![].into_iter().try_collect_exact_array::<usize, 0>(),
            Ok([])
        );
    }

    #[test]
    fn exact_too_few() {
        assert_matches!(
            (0..5).try_collect_exact_array::<i64, 6>(),
            Err(TryCollectArrayError::TooFewItems {
                expect: 6,
                found: 5
            })
        );
        assert_matches!(
            "xyz".chars().try_collect_exact_array::<char, 7>(),
            Err(TryCollectArrayError::TooFewItems {
                expect: 7,
                found: 3
            })
        );
        assert_matches!(
            vec![].into_iter().try_collect_exact_array::<usize, 8>(),
            Err(TryCollectArrayError::TooFewItems {
                expect: 8,
                found: 0
            })
        );
    }

    #[test]
    fn exact_too_many() {
        assert_matches!(
            std::iter::repeat(99).try_collect_exact_array::<i64, 8>(),
            Err(TryCollectArrayError::TooManyItems { expect: 8 })
        );
        assert_matches!(
            (0..1000).try_collect_exact_array::<i64, 0>(),
            Err(TryCollectArrayError::TooManyItems { expect: 0 })
        );
        assert_matches!(
            (0..5).try_collect_exact_array::<i64, 4>(),
            Err(TryCollectArrayError::TooManyItems { expect: 4 })
        );
        assert_matches!(
            "xyz".chars().try_collect_exact_array::<char, 1>(),
            Err(TryCollectArrayError::TooManyItems { expect: 1 })
        );
    }
}
