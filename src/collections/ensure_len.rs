use std::error::Error;
use std::fmt::{Display, Formatter};

/// Trait extensions that provides blanket implementation for every single type.
pub trait EnsureThat {
    /// Ensure that the object satisfies the given predicate.
    /// It returns itself inside [`Ok` result] if the predicate check has passed,
    /// otherwise it returns an error of type [`EnsureThatError`].
    ///
    /// [`Ok` result]: std::result::Result
    fn ensure_that(self, predicate: impl FnOnce(&Self) -> bool) -> Result<Self, EnsureThatError>
    where
        Self: Sized,
    {
        match predicate(&self) {
            true => Ok(self),
            false => Err(EnsureThatError),
        }
    }
}

impl<T: ?Sized> EnsureThat for T {}

/// Error type for trait [`EnsureThat`] indicating that an item failed the predicate check.
#[derive(Debug)]
pub struct EnsureThatError;

impl Display for EnsureThatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ensure on the object failed the predicate")
    }
}

impl Error for EnsureThatError {}

#[cfg(test)]
mod tests {
    use matches::assert_matches;

    use super::*;

    #[test]
    fn on_string() {
        assert_matches!(
            "Hello, World!"
                .ensure_that(|s| s.len() > 3)
                .map_err(|e| e.to_string()),
            Ok("Hello, World!")
        );
        assert_matches!(
            "Hello, World!".to_string().ensure_that(|s| s.len() < 3),
            Err(EnsureThatError)
        );
    }
}
