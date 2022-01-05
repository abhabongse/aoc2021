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
    (
        CHECKS_TOO_MANY:$checks_too_many:ident,
        METHOD_RETURNS:$method_returns:ident,
        UNWRAPS_ITEM:$unwraps_item:ident,
        $it:ident,
        $size:ident
    ) => {{
        let mut it = $it.peekable();
        let mut accumulated: ArrayVec<T, SIZE> = ArrayVec::new();
        for _i in 0..SIZE {
            if let Some(item) = it.next() {
                accumulated.push(generate_collect_method!(_unwrap_exit($unwraps_item, item)));
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
    (_unwrap_exit(false, $item:ident)) => {
        $item
    };
    (_unwrap_exit(true, $item:ident)) => {
        $item?
    };
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
    /// Too few or too many items produced by the iterator will return in [`anyhow::Error`].
    /// If you wish to also recover partial data produced by the iterator,
    /// use [`collect_exact_recoverable`](CollectArray::collect_exact_recoverable) instead.
    fn collect_exact<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(CHECKS_TOO_MANY:true, METHOD_RETURNS:anyhow, UNWRAPS_ITEM:false, self, SIZE)
    }

    /// Same as [`collect_exact`](CollectArray::collect_exact) but with error type [`CollectArrayError`]
    /// which contains partially processed data from the iterator.
    fn collect_exact_recoverable<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], CollectArrayError<Self, T>>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(CHECKS_TOO_MANY:true, METHOD_RETURNS:custom_error, UNWRAPS_ITEM:false, self, SIZE)
    }

    /// Same as [`collect_exact`](CollectArray::collect_exact) but each item under `Result` type
    /// will be unwrapped and boxed under [`anyhow::Error`]  upon the first error found.
    fn try_collect_exact<T, E, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        anyhow::Error: From<E>,
    {
        generate_collect_method!(CHECKS_TOO_MANY:true, METHOD_RETURNS:anyhow, UNWRAPS_ITEM:true, self, SIZE)
    }

    /// Collects all items from the iterator into a constant-sized array.
    /// Too few items produced by the iterator will return in [`anyhow::Error`].
    /// If you wish to also recover partial data produced by the iterator,
    /// use [`collect_trunc_recoverable`](CollectArray::collect_trunc_recoverable) instead.
    ///
    /// Note that extraneous items will not be consumed from the iterator
    /// and thus the upstream caller may resume working with such iterator.
    fn collect_trunc<T, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(CHECKS_TOO_MANY:false, METHOD_RETURNS:anyhow, UNWRAPS_ITEM:false, self, SIZE)
    }

    /// Same as [`collect_trunc`](CollectArray::collect_trunc) but the error type [`CollectArrayError`]
    /// which contains partially processed data from the iterator.
    fn collect_trunc_recoverable<T, const SIZE: usize>(
        self,
    ) -> Result<[T; SIZE], CollectArrayError<Self, T>>
    where
        Self: Sized + Iterator<Item = T>,
    {
        generate_collect_method!(CHECKS_TOO_MANY:false, METHOD_RETURNS:custom_error, UNWRAPS_ITEM:false, self, SIZE)
    }

    /// Same as [`collect_trunc`](CollectArray::collect_trunc) but each item under `Result` type
    /// will be unwrapped and boxed under [`anyhow::Error`]  upon the first error found.
    fn try_collect_trunc<T, E, const SIZE: usize>(self) -> anyhow::Result<[T; SIZE]>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        anyhow::Error: From<E>,
    {
        generate_collect_method!(CHECKS_TOO_MANY:false, METHOD_RETURNS:anyhow, UNWRAPS_ITEM:true, self, SIZE)
    }
}

impl<I: ?Sized> CollectArray for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use paste::paste;
    use test_case::test_case;

    use super::*;

    #[test_case(0..5, [0, 1, 2, 3, 4])]
    #[test_case("xyz".chars(), ['x', 'y', 'z'])]
    #[test_case(Vec::<usize>::new().into_iter(), [])]
    fn exact_ok<I, T, const SIZE: usize>(input_it: I, expected: [T; SIZE])
    where
        I: Debug + Clone + Iterator<Item = T>,
        T: Debug + PartialEq,
    {
        let result: [_; SIZE] = input_it.clone().collect_exact().unwrap();
        assert_eq!(result, expected);
        let result: [_; SIZE] = input_it.collect_exact_recoverable().unwrap();
        assert_eq!(result, expected);
    }

    #[test_case(0..5, [0, 1, 2, 3, 4])]
    #[test_case("xyz".chars(), ['x', 'y', 'z'])]
    #[test_case(Vec::<usize>::new().into_iter(), [])]
    #[test_case(std::iter::repeat(99), [99, 99, 99, 99])]
    #[test_case(0..1000, [])]
    fn trunc_ok<I, T, const SIZE: usize>(input_it: I, expected: [T; SIZE])
    where
        I: Debug + Clone + Iterator<Item = T>,
        T: Debug + PartialEq,
    {
        let result: [_; SIZE] = input_it.clone().collect_trunc().unwrap();
        assert_eq!(result, expected);
        let result: [_; SIZE] = input_it.collect_trunc_recoverable().unwrap();
        assert_eq!(result, expected);
    }

    macro_rules! test_collect_too_few {
        ($test_name:tt, $input:expr, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< too_few_ $test_name >] () {
                    assert_eq!($input.collect_trunc::<_, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_trunc_recoverable::<_, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_exact::<_, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_exact_recoverable::<_, $size>().unwrap_err().to_string(), $expected);
                }
            }
        };
    }
    test_collect_too_few!(
        0,
        (0..5),
        6,
        "too few items from the iterator (expected 6 but found only 5)"
    );
    test_collect_too_few!(
        1,
        "xyz".chars(),
        7,
        "too few items from the iterator (expected 7 but found only 3)"
    );
    test_collect_too_few!(
        2,
        Vec::<usize>::new().into_iter(),
        8,
        "too few items from the iterator (expected 8 but found only 0)"
    );

    macro_rules! test_collect_too_many {
        ($test_name:tt, $input:expr, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< too_many_ $test_name >] () {
                    assert_eq!($input.collect_exact::<_, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.collect_exact_recoverable::<_, $size>().unwrap_err().to_string(), $expected);
                }
            }
        };
    }
    test_collect_too_many!(
        0,
        std::iter::repeat(99),
        8,
        "too many items from the iterator (expected only 8)"
    );
    test_collect_too_many!(
        1,
        (0..1000),
        0,
        "too many items from the iterator (expected only 0)"
    );
    test_collect_too_many!(
        2,
        (0..5),
        4,
        "too many items from the iterator (expected only 4)"
    );
    test_collect_too_many!(
        3,
        "xyz".chars(),
        1,
        "too many items from the iterator (expected only 1)"
    );

    #[test_case((0..5).map(|v| (v >= -1).then(|| v).context("")), [0, 1, 2, 3, 4])]
    #[test_case("abc".chars().map(|v| v.to_digit(16).context("not a digit")), [10, 11, 12])]
    #[test_case(Vec::<anyhow::Result<i32>>::new().into_iter(), [])]
    fn try_exact_ok<I, T, E, const SIZE: usize>(input_it: I, expected: [T; SIZE])
    where
        I: Debug + Iterator<Item = Result<T, E>>,
        T: Debug + PartialEq,
        anyhow::Error: From<E>,
    {
        let result: [_; SIZE] = input_it.try_collect_exact().unwrap();
        assert_eq!(result, expected);
    }

    #[test_case((0..5).map(|v| (v >= 0).then(|| v).context("")), [0, 1, 2, 3, 4])]
    #[test_case("abc".chars().map(|v| v.to_digit(16).context("not a digit")), [10, 11, 12])]
    #[test_case(Vec::<anyhow::Result<i32>>::new().into_iter(), [])]
    #[test_case(std::iter::repeat(()).map(|_| -> anyhow::Result<_> { Ok(99) }), [99, 99, 99, 99])]
    #[test_case((0..1000).map(|v| match v {
        v if v % 2 == 0 => Ok(v / 2),
        _ => bail!("do not do anything"),
    }), [])]
    fn try_trunc_ok<I, T, E, const SIZE: usize>(input_it: I, expected: [T; SIZE])
    where
        I: Debug + Iterator<Item = Result<T, E>>,
        T: Debug + PartialEq,
        anyhow::Error: From<E>,
    {
        let result: [_; SIZE] = input_it.try_collect_trunc().unwrap();
        assert_eq!(result, expected);
    }

    macro_rules! test_try_collect_too_few {
        ($test_name:tt, $input:expr, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< try_too_few_ $test_name >] () {
                    assert_eq!($input.try_collect_trunc::<_, _, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.try_collect_exact::<_, _, $size>().unwrap_err().to_string(), $expected);
                }
            }
        };
    }
    test_try_collect_too_few!(
        0,
        (0..5).map(|v| (v >= 0).then(|| v).context("")),
        6,
        "too few items from the iterator (expected 6 but found only 5)"
    );
    test_try_collect_too_few!(
        1,
        "abc".chars().map(|v| v.to_digit(16).context("not a digit")),
        7,
        "too few items from the iterator (expected 7 but found only 3)"
    );
    test_try_collect_too_few!(
        2,
        Vec::<anyhow::Result<i32>>::new().into_iter(),
        8,
        "too few items from the iterator (expected 8 but found only 0)"
    );

    macro_rules! test_try_collect_too_many {
        ($test_name:tt, $input:expr, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< try_too_many_ $test_name >] () {
                    assert_eq!($input.try_collect_exact::<_, _, $size>().unwrap_err().to_string(), $expected);
                }
            }
        };
    }
    test_try_collect_too_many!(
        0,
        std::iter::repeat(99).map(|v| (v >= 0).then(|| v).context("")),
        8,
        "too many items from the iterator (expected only 8)"
    );
    test_try_collect_too_many!(
        1,
        (0..).map(|_| -> anyhow::Result<i64> { bail!("oh well") }),
        0,
        "too many items from the iterator (expected only 0)"
    );
    test_try_collect_too_many!(
        2,
        (0..).map(|v| match v {
            v if v < 100 => Ok(v * 2),
            _ => bail!("oh no"),
        }),
        4,
        "too many items from the iterator (expected only 4)"
    );
    test_try_collect_too_many!(
        3,
        "abc".chars().map(|v| v.to_digit(16).context("not a digit")),
        0,
        "too many items from the iterator (expected only 0)"
    );

    macro_rules! test_try_collect_unwrap {
        ($test_name:tt, $input:expr, $size:literal, $expected:expr) => {
            paste! {
                #[test]
                fn [< try_unwrap_ $test_name >] () {
                    assert_eq!($input.try_collect_trunc::<_, _, $size>().unwrap_err().to_string(), $expected);
                    assert_eq!($input.try_collect_exact::<_, _, $size>().unwrap_err().to_string(), $expected);
                }
            }
        };
    }
    test_try_collect_unwrap!(
        0,
        ('a'..='z').map(|v| v.to_digit(16).context("oh well that doesn't work")),
        8,
        "oh well that doesn't work"
    );
    test_try_collect_unwrap!(
        1,
        ('a'..='z').map(|v| v.to_digit(16).context("oh well that doesn't work")),
        30,
        "oh well that doesn't work"
    );
    test_try_collect_unwrap!(
        2,
        ["1", "3e4", "4"].into_iter().map(|v| v.parse::<i64>()),
        3,
        "invalid digit found in string"
    );
    test_try_collect_unwrap!(
        3,
        ["1", "3e4", "4"].into_iter().map(|v| v.parse::<i64>()),
        10,
        "invalid digit found in string"
    );
}
