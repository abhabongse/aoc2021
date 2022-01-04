//! Implements a blanket trait extension for [`Iterator`] trait which adds methods
//! to collect items from an iterator into a constant-sized array.
use std::fmt::Debug;
use std::iter::Peekable;

use anyhow::bail;
use arrayvec::ArrayVec;
use itertools::Itertools;
use thiserror::Error;

/// More detailed error for [`CollectArray`] methods
#[derive(Error, Debug, Clone)]
pub enum CollectArrayError<I, T>
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

macro_rules! generate_too_few_return {
    (RETURN_CUSTOM_ERROR, $size:ident, $accumulated:ident) => {
        return Err(CollectArrayError::TooFewItems {
            target: $size,
            accumulated: $accumulated.into_iter().collect_vec(),
        });
    };
    (RETURN_ANYHOW, $size:ident, $accumulated:ident) => {
        bail!(
            "too few items from the iterator (expected {} but found only {})",
            $size,
            $accumulated.len()
        );
    };
}

macro_rules! generate_too_many_return {
    (NO_CHECK_TOO_MANY, $_1:ident, $_2:ident, $_3:ident, $_4:ident) => {};
    (CHECK_TOO_MANY, RETURN_CUSTOM_ERROR, $it:ident, $size:ident, $accumulated:ident) => {
        if $it.peek().is_some() {
            return Err(CollectArrayError::TooManyItems {
                target: $size,
                accumulated: $accumulated.into_iter().collect_vec(),
                remaining: $it,
            });
        }
    };
    (CHECK_TOO_MANY, RETURN_ANYHOW, $it:ident, $size:ident, $accumulated:ident) => {
        if $it.peek().is_some() {
            bail!("too many items from the iterator (expected only {})", $size);
        }
    };
}

macro_rules! generate_collect_method {
    ($checks_too_many:ident, $method_returns:ident, $it:ident, $size:ident) => {{
        let mut it = $it.peekable();
        let mut accumulated: ArrayVec<T, SIZE> = ArrayVec::new();
        for _i in 0..SIZE {
            if let Some(item) = it.next() {
                accumulated.push(item);
            } else {
                generate_too_few_return!($method_returns, $size, accumulated);
            }
        }
        generate_too_many_return!($checks_too_many, $method_returns, it, $size, accumulated);
        match accumulated.into_inner() {
            Ok(array) => Ok(array),
            Err(_) => unreachable!(),
        }
    }};
}

/// Trait extension for [`Iterator`] trait which add methods
/// to collect items from an iterator into a constant-sized array.
pub trait CollectArray: Iterator {
    /// Collects all items from the iterator into a constant-sized array.
    /// Too few or too many items produced will result in an [`CollectArrayError`].
    /// If you want a version of this method that provides partial data recovery,
    /// see [`collect_exact_recoverable`](CollectArray::collect_exact_recoverable).
    fn collect_exact<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(CHECK_TOO_MANY, RETURN_ANYHOW, self, SIZE)
    }

    /// Same as [`collect_exact_recoverable`](CollectArray::collect_exact_recoverable)
    /// but the error type [`CollectArrayError`] also returns partially processed data.
    ///
    /// TODO: create test cases to test error values of this method
    fn collect_exact_recoverable<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], CollectArrayError<Self, T>>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(CHECK_TOO_MANY, RETURN_CUSTOM_ERROR, self, SIZE)
    }

    // /// Short-circuit version of [`collect_exact_recoverable`](CollectArray::collect_exact_recoverable).
    // /// The result would contain the value wrapped inside `Ok` item from the original iterator.
    // ///
    // /// - TODO: Replace anyhow with custom structs
    // /// - TODO: Provide recoverable version of this method
    // /// - TODO: Provide truncated version of this method
    // fn try_collect_exact<T, E, const SIZE: usize>(mut self) -> anyhow::Result<[T; SIZE]>
    // where
    //     Self: Sized + Iterator<Item = Result<T, E>>,
    //     E: 'static + StdError + Send + Sync,
    // {
    //     let mut accumulated: ArrayVec<T, SIZE> = ArrayVec::new();
    //     for _i in 0..SIZE {
    //         let item = match self.next() {
    //             Some(item) => item?,
    //             None => bail!("too few items from the iterator"),
    //         };
    //         accumulated.push(item);
    //     }
    //     if self.next().is_some() {
    //         bail!("too many items from the iterator");
    //     }
    //     match accumulated.into_inner() {
    //         Ok(array) => Ok(array),
    //         Err(_) => unreachable!(),
    //     }
    // }

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few items produced will result in an [`CollectArrayError`].
    /// Extraneous items produced by the iterator will not be collected.
    /// If you want a version of this method that provides partial data recovery,
    /// see [`collect_trunc_recoverable`](CollectArray::collect_trunc_recoverable).
    fn collect_trunc<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(NO_CHECK_TOO_MANY, RETURN_ANYHOW, self, SIZE)
    }

    /// Same as [`collect_trunc_recoverable`](CollectArray::collect_trunc_recoverable)
    /// but the error type [`CollectArrayError`] also returns partial data.
    ///
    /// TODO: create test cases to test error values of this method
    fn collect_trunc_recoverable<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], CollectArrayError<Self, T>>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(NO_CHECK_TOO_MANY, RETURN_CUSTOM_ERROR, self, SIZE)
    }
}

impl<I: ?Sized> CollectArray for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_trunc_ok() {
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
