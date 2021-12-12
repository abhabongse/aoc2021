//! Provides a blanket implementation of method [`ensure_that`]
//! which validates an item itself with a predicate,
//! and returns itself wrapped inside [`Ok` result] if the predicate is satisfied.
//!
//! [`ensure_that`]: EnsureThat::ensure_that
use anyhow::ensure;

/// Trait extension that provides blanket implementation of the of method [`ensure_that`]
/// which validates an item itself with a predicate,
/// and returns itself wrapped inside [`Ok` result] if the predicate is satisfied.
///
/// [`ensure_that`]: EnsureThat::ensure_that
pub trait EnsureThat {
    /// Ensures that the object satisfies the provided predicate.
    /// It returns itself wrapped inside [`Ok` result] if the predicate is satisfied.
    ///
    /// [`Ok` result]: std::result::Result
    fn ensure_that(self, predicate: impl FnOnce(&Self) -> bool) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        ensure!(
            predicate(&self),
            "the object failed to validate the provided predicate"
        );
        Ok(self)
    }
}

impl<T: ?Sized> EnsureThat for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_string() {
        assert_eq!(
            "Hello, World!".ensure_that(|s| s.len() > 3).unwrap(),
            "Hello, World!"
        );
        assert_eq!(
            "Hello, World!"
                .ensure_that(|s| s.len() < 3)
                .unwrap_err()
                .to_string(),
            "the object failed to validate the provided predicate"
        )
    }

    #[test]
    fn on_integer() {
        assert_eq!(3.ensure_that(|x| *x > 2).unwrap(), 3);
        assert_eq!(
            20.ensure_that(|x| *x < 2).unwrap_err().to_string(),
            "the object failed to validate the provided predicate"
        );
    }
}
