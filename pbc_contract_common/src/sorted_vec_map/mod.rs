//! Definition of sorted [`Vec`]-based [`SortedVecMap`] and [`SortedVecSet`].
//!
//! [`SortedVecMap`] and [`SortedVecSet`] provides constant time serialization/deserialization if
//! the entries is serializable by copy (Constant size determinable at compile time), and
//! additionally amortized `O(n)` insert and `O(log(n))` lookup.

pub mod entry;
mod map;
mod set;
mod vec;

pub use map::SortedVecMap;
pub use set::SortedVecSet;
pub use vec::SortedVec;
