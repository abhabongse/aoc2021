use anyhow::{anyhow, bail};

/// Fallible conversion of an iterator producing item of type `A`
/// into the implemented (targeted) type.
/// TODO: convert this into trait extensions of [`std::convert::TryFrom`] instead
pub trait TryFromIterator<A>: Sized {
    /// Attempts to convert an iterator into the targeted type.
    fn try_from_iter<I>(iter: I) -> Result<Self, anyhow::Error>
    where
        I: IntoIterator<Item = A>;
}

impl<T, const SIZE: usize> TryFromIterator<T> for [T; SIZE] {
    fn try_from_iter<I>(iter: I) -> Result<Self, anyhow::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let mut buffer: Vec<T> = Vec::with_capacity(SIZE);
        for item in iter.into_iter() {
            if buffer.len() >= SIZE {
                bail!("iterator produces items over the target size {}", SIZE);
            }
            buffer.push(item);
        }
        buffer.try_into().map_err(|v: Vec<T>| {
            anyhow!(
                "iterator produces {} items which is smaller than the target size {}",
                v.len(),
                SIZE
            )
        })
    }
}

/// Trait extensions for [`Iterator`] type, adding a few extra methods.
///
/// [`Iterator`]: std::iter::Iterator
pub trait TryCollectExt: Iterator {
    fn try_collect_into<T>(self) -> anyhow::Result<T>
    where
        Self: Sized,
        T: TryFromIterator<Self::Item>,
    {
        T::try_from_iter(self)
    }
}

impl<T: ?Sized> TryCollectExt for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_iter() {
        assert_eq!(
            <[i64; 5]>::try_from_iter(0..5).map_err(|err| err.to_string()),
            Ok([0, 1, 2, 3, 4])
        );
        assert_eq!(
            <[i64; 5]>::try_from_iter(0..4).map_err(|err| err.to_string()),
            Err("iterator produces 4 items which is smaller than the target size 5".to_string()),
        );
        assert_eq!(
            <[i64; 5]>::try_from_iter(0..6).map_err(|err| err.to_string()),
            Err("iterator produces items over the target size 5".to_string()),
        );
    }

    #[test]
    fn test_collect_vec_cool() {
        let result = (0..5).try_collect_into::<[i64; 5]>();
        assert_eq!(result.map_err(|err| err.to_string()), Ok([0, 1, 2, 3, 4]));
        let result = (0..4).try_collect_into::<[i64; 5]>();
        assert_eq!(
            result.map_err(|err| err.to_string()),
            Err("iterator produces 4 items which is smaller than the target size 5".to_string())
        );
        let result = (0..6).try_collect_into::<[i64; 5]>();
        assert_eq!(
            result.map_err(|err| err.to_string()),
            Err("iterator produces items over the target size 5".to_string())
        );
    }
}
