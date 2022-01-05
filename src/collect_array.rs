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

macro_rules! generate_collect_method {
    (CHECKS_TOO_MANY:$checks_too_many:ident, METHOD_RETURNS:$method_returns:ident, $it:ident, $size:ident) => {{
        let mut it = $it.peekable();
        let mut accumulated: ArrayVec<T, SIZE> = ArrayVec::new();
        for _i in 0..SIZE {
            if let Some(item) = it.next() {
                accumulated.push(item);
            } else {
                generate_collect_method!(_too_few_exit($method_returns, $size, accumulated));
            }
        }
        generate_collect_method!(_too_many_exit(
            $checks_too_many,
            $method_returns,
            it,
            $size,
            accumulated
        ));
        match accumulated.into_inner() {
            Ok(array) => Ok(array),
            Err(_) => unreachable!(),
        }
    }};
    (_too_few_exit(custom_error, $size:ident, $accumulated:ident)) => {
        return Err(CollectArrayError::TooFewItems {
            target: $size,
            accumulated: $accumulated.into_iter().collect_vec(),
        });
    };
    (_too_few_exit(anyhow, $size:ident, $accumulated:ident)) => {
        bail!(
            "too few items from the iterator (expected {} but found only {})",
            $size,
            $accumulated.len()
        );
    };
    (_too_many_exit(false, $($_:expr),*)) => {};
    (_too_many_exit(true, custom_error, $it:ident, $size:ident, $accumulated:ident)) => {
        if $it.peek().is_some() {
            return Err(CollectArrayError::TooManyItems {
                target: $size,
                accumulated: $accumulated.into_iter().collect_vec(),
                remaining: $it,
            });
        }
    };
    (_too_many_exit(true, anyhow, $it:ident, $size:ident, $accumulated:ident)) => {
        if $it.peek().is_some() {
            bail!("too many items from the iterator (expected only {})", $size);
        }
    };
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
        generate_collect_method!(CHECKS_TOO_MANY:true, METHOD_RETURNS:anyhow, self, SIZE)
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
        generate_collect_method!(CHECKS_TOO_MANY:true, METHOD_RETURNS:custom_error, self, SIZE)
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
        generate_collect_method!(CHECKS_TOO_MANY:false, METHOD_RETURNS:anyhow, self, SIZE)
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
        generate_collect_method!(CHECKS_TOO_MANY:false, METHOD_RETURNS:custom_error, self, SIZE)
    }
}

impl<I: ?Sized> CollectArray for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use paste::paste;

    use super::*;

    macro_rules! test_collect_trunc_ok {
        ($test_name:tt, $input:expr, $type:ty, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< trunc_ok_ $test_name >] () {
                    assert_eq!($input.collect_trunc::<$type, $size>().unwrap(), $expected);
                    assert_eq!($input.collect_trunc_recoverable::<$type, $size>().unwrap(), $expected);
                }
            }
        };
    }
    test_collect_trunc_ok!(0, (0..5), i64, 5, [0, 1, 2, 3, 4]);
    test_collect_trunc_ok!(1, "xyz".chars(), char, 3, ['x', 'y', 'z']);
    test_collect_trunc_ok!(2, vec![].into_iter(), usize, 0, []);
    test_collect_trunc_ok!(3, std::iter::repeat(99), i64, 4, [99, 99, 99, 99]);
    test_collect_trunc_ok!(4, (0..1000), i64, 0, []);

    macro_rules! test_collect_exact_ok {
        ($test_name:tt, $input:expr, $type:ty, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< exact_ok_ $test_name >] () {
                    assert_eq!($input.collect_exact::<$type, $size>().unwrap(), $expected);
                    assert_eq!($input.collect_exact_recoverable::<$type, $size>().unwrap(), $expected);
                }
            }
        };
    }
    test_collect_exact_ok!(0, (0..5), i64, 5, [0, 1, 2, 3, 4]);
    test_collect_exact_ok!(1, "xyz".chars(), char, 3, ['x', 'y', 'z']);
    test_collect_exact_ok!(2, vec![].into_iter(), usize, 0, []);

    macro_rules! test_collect_too_few {
        ($test_name:tt, $input:expr, $type:ty, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< too_few_ $test_name >] () {
                    assert_eq!($input.collect_trunc::<$type, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_trunc_recoverable::<$type, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_exact::<$type, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_exact_recoverable::<$type, $size>().unwrap_err().to_string(), $expected);
                }
            }
        };
    }
    test_collect_too_few!(
        0,
        (0..5),
        i64,
        6,
        "too few items from the iterator (expected 6 but found only 5)"
    );
    test_collect_too_few!(
        1,
        "xyz".chars(),
        char,
        7,
        "too few items from the iterator (expected 7 but found only 3)"
    );
    test_collect_too_few!(
        2,
        vec![].into_iter(),
        usize,
        8,
        "too few items from the iterator (expected 8 but found only 0)"
    );

    macro_rules! test_collect_too_many {
        ($test_name:tt, $input:expr, $type:ty, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< too_many_ $test_name >] () {
                    assert_eq!($input.collect_exact::<$type, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_exact_recoverable::<$type, $size>().unwrap_err().to_string(), $expected);
                }
            }
        };
    }
    test_collect_too_many!(
        0,
        std::iter::repeat(99),
        i64,
        8,
        "too many items from the iterator (expected only 8)"
    );
    test_collect_too_many!(
        1,
        (0..1000),
        i64,
        0,
        "too many items from the iterator (expected only 0)"
    );
    test_collect_too_many!(
        2,
        (0..5),
        i64,
        4,
        "too many items from the iterator (expected only 4)"
    );
    test_collect_too_many!(
        3,
        "xyz".chars(),
        char,
        1,
        "too many items from the iterator (expected only 1)"
    );
}
