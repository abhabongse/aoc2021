use std::str::FromStr;

use anyhow::anyhow;

/// Provides extra [`quickparse`] method for strings
///
/// [`quickparse`]: QuickParse::quickparse
pub trait QuickParse {
    /// Shortcut syntax to parsing a string into target type implementing [`FromStr`]
    ///
    /// [`FromStr`]: std::str::FromStr
    fn quickparse<F>(&self) -> anyhow::Result<F>
    where
        F: FromStr;
}

impl QuickParse for str {
    fn quickparse<F>(&self) -> anyhow::Result<F>
    where
        F: FromStr,
    {
        self.parse().map_err(|_| {
            anyhow!(
                "cannot parse token for type {}: {}",
                std::any::type_name::<F>(),
                self
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_int() {
        let a: i64 = "-37".quickparse().unwrap();
        assert_eq!(a, -37);
        let b: usize = "683".quickparse().unwrap();
        assert_eq!(b, 683);
    }
}
