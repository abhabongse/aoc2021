use anyhow::bail;

/// Trait extension that provides blanket implementation of the method [`ensure_that`]
/// for every single type.
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
        match predicate(&self) {
            true => Ok(self),
            false => bail!("ensure predicate failed on the object"),
        }
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
            "ensure predicate failed on the object"
        )
    }
}
