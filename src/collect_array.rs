//! Implements a blanket trait extension for [`Iterator`] trait which adds methods
//! to collect items from an iterator into a constant-sized array.
use std::fmt::Debug;
use std::iter::Peekable;

use arrayvec::ArrayVec;
use itertools::Itertools;
use thiserror::Error;

/// Error for [`CollectArray`] methods
#[derive(Error, Debug, Clone)]
pub enum CollectArrayError {
    #[error("too few items from the iterator (expected {target} but found only {found})")]
    TooFewItems { target: usize, found: usize },
    #[error("too many items from the iterator (expected only {target})")]
    TooManyItems { target: usize },
}

/// More detailed error for [`CollectArray`] methods
#[derive(Error, Debug, Clone)]
pub enum CollectArrayRecoverableError<T, I>
where
    I: Iterator<Item = T>,
{
    #[error("too few items from the iterator (expected {target} but found only {})", .accumulated.len())]
    TooFewItems { target: usize, accumulated: Vec<T> },
    #[error("too many items from the iterator (expected only {target})")]
    TooManyItems {
        target: usize,
        accumulated: Vec<T>,
        remaining: Peekable<I>,
    },
}

impl<T, I> From<CollectArrayRecoverableError<T, I>> for CollectArrayError
where
    I: Iterator<Item = T>,
{
    fn from(err: CollectArrayRecoverableError<T, I>) -> Self {
        match err {
            CollectArrayRecoverableError::TooFewItems {
                target,
                accumulated,
            } => CollectArrayError::TooFewItems {
                target,
                found: accumulated.len(),
            },
            CollectArrayRecoverableError::TooManyItems { target, .. } => {
                CollectArrayError::TooManyItems { target }
            }
        }
    }
}

/// Trait extension for [`Iterator`] trait which add methods
/// to collect items from an iterator into a constant-sized array.
pub trait CollectArray: Iterator {
    /// Collects all items from the iterator into a constant-sized array.
    /// Too few or too many items produced will result in an [`CollectArrayError`].
    /// If you want a version of this method that provides partial data recovery,
    /// see [`collect_exact_recoverable`](CollectArray::collect_exact_recoverable).
    fn collect_exact<T, const SIZE: usize>(self) -> Result<[T; SIZE], CollectArrayError>
    where
        Self: Sized + Iterator<Item = T>,
    {
        Ok(self.collect_exact_recoverable()?)
    }

    /// Same as [`collect_exact_recoverable`](CollectArray::collect_exact_recoverable)
    /// but the error type [`CollectArrayRecoverableError`] also returns partial data.
    ///
    /// TODO: create test cases to test error values of this method
    fn collect_exact_recoverable<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], CollectArrayRecoverableError<T, Self>>
    where
        Self: Sized + Iterator<Item = T>,
    {
        let mut it = self.peekable();
        let mut accumulated: ArrayVec<T, SIZE> = ArrayVec::new();
        for _i in 0..SIZE {
            if let Some(item) = it.next() {
                accumulated.push(item);
            } else {
                return Err(CollectArrayRecoverableError::TooFewItems {
                    target: SIZE,
                    accumulated: accumulated.into_iter().collect_vec(),
                });
            }
        }
        if it.peek().is_some() {
            return Err(CollectArrayRecoverableError::TooManyItems {
                target: SIZE,
                accumulated: accumulated.into_iter().collect_vec(),
                remaining: it,
            });
        }
        match accumulated.into_inner() {
            Ok(array) => Ok(array),
            Err(_) => unreachable!(),
        }
    }

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few items produced will result in an [`CollectArrayError`].
    /// Extraneous items produced by the iterator will not be collected.
    /// If you want a version of this method that provides partial data recovery,
    /// see [`collect_exact_recoverable`](CollectArray::collect_exact_recoverable).
    fn collect_trunc<T, const SIZE: usize>(self) -> Result<[T; SIZE], CollectArrayError>
    where
        Self: Sized + Iterator<Item = T>,
    {
        Ok(self.collect_trunc_recoverable()?)
    }

    /// Same as [`collect_trunc_recoverable`](CollectArray::collect_trunc_recoverable)
    /// but the error type [`CollectArrayRecoverableError`] also returns partial data.
    ///
    /// TODO: create test cases to test error values of this method
    fn collect_trunc_recoverable<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], CollectArrayRecoverableError<T, Self>>
    where
        Self: Sized + Iterator<Item = T>,
    {
        let mut it = self.peekable();
        let mut accumulated: ArrayVec<T, SIZE> = ArrayVec::new();
        for _i in 0..SIZE {
            if let Some(item) = it.next() {
                accumulated.push(item);
            } else {
                return Err(CollectArrayRecoverableError::TooFewItems {
                    target: SIZE,
                    accumulated: accumulated.into_iter().collect_vec(),
                });
            }
        }
        match accumulated.into_inner() {
            Ok(array) => Ok(array),
            Err(_) => unreachable!(),
        }
    }
}

impl<I> CollectArray for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trunc_ok() {
        assert_eq!((0..5).collect_trunc::<i64, 5>().unwrap(), [0, 1, 2, 3, 4]);
        assert_eq!(
            "xyz".chars().collect_trunc::<char, 3>().unwrap(),
            ['x', 'y', 'z']
        );
        assert_eq!(vec![].into_iter().collect_trunc::<usize, 0>().unwrap(), []);
        assert_eq!(
            std::iter::repeat(99).collect_trunc::<i64, 4>().unwrap(),
            [99, 99, 99, 99]
        );
        assert_eq!((0..1000).collect_trunc::<i64, 0>().unwrap(), []);
    }

    #[test]
    fn trunc_too_few() {
        assert_eq!(
            (0..5).collect_trunc::<i64, 6>().unwrap_err().to_string(),
            "too few items from the iterator (expected 6 but found only 5)"
        );
        assert_eq!(
            "xyz"
                .chars()
                .collect_trunc::<char, 7>()
                .unwrap_err()
                .to_string(),
            "too few items from the iterator (expected 7 but found only 3)"
        );
        assert_eq!(
            vec![]
                .into_iter()
                .collect_trunc::<usize, 8>()
                .unwrap_err()
                .to_string(),
            "too few items from the iterator (expected 8 but found only 0)"
        );
    }

    #[test]
    fn exact_ok() {
        assert_eq!((0..5).collect_exact::<i64, 5>().unwrap(), [0, 1, 2, 3, 4]);
        assert_eq!(
            "xyz".chars().collect_exact::<char, 3>().unwrap(),
            ['x', 'y', 'z']
        );
        assert_eq!(vec![].into_iter().collect_exact::<usize, 0>().unwrap(), []);
    }

    #[test]
    fn exact_too_few() {
        assert_eq!(
            (0..5).collect_exact::<i64, 6>().unwrap_err().to_string(),
            "too few items from the iterator (expected 6 but found only 5)"
        );
        assert_eq!(
            "xyz"
                .chars()
                .collect_exact::<char, 7>()
                .unwrap_err()
                .to_string(),
            "too few items from the iterator (expected 7 but found only 3)"
        );
        assert_eq!(
            vec![]
                .into_iter()
                .collect_exact::<usize, 8>()
                .unwrap_err()
                .to_string(),
            "too few items from the iterator (expected 8 but found only 0)"
        );
    }

    #[test]
    fn exact_too_many() {
        assert_eq!(
            std::iter::repeat(99)
                .collect_exact::<i64, 8>()
                .unwrap_err()
                .to_string(),
            "too many items from the iterator (expected only 8)"
        );
        assert_eq!(
            (0..1000).collect_exact::<i64, 0>().unwrap_err().to_string(),
            "too many items from the iterator (expected only 0)"
        );
        assert_eq!(
            (0..5).collect_exact::<i64, 4>().unwrap_err().to_string(),
            "too many items from the iterator (expected only 4)"
        );
        assert_eq!(
            "xyz"
                .chars()
                .collect_exact::<char, 1>()
                .unwrap_err()
                .to_string(),
            "too many items from the iterator (expected only 1)"
        );
    }
}
