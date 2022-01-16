//! Hashed collections with XXHash fast hashing algorithm
use std::hash::BuildHasherDefault;

use twox_hash::XxHash64;

/// HashMap with XXHash fast hashing algorithm
pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<XxHash64>>;

/// HashSet with XXHash fast hashing algorithm
pub type HashSet<T> = std::collections::HashSet<T, BuildHasherDefault<XxHash64>>;
