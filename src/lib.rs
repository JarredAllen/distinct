//! A pair of traits which can be used to ensure that two types either
//! are distinct or are not distinct.
//!
//! There are two traits: `Distinct` and `NonDistinct`. `Distinct` is
//! implemented for tuples of two distinct types. `NonDistinct` is
//! implemented for tuples of two of the same type.
//!
//! # Examples
//!
//! ```
//! use distinct::{Distinct, NonDistinct};
//! // Two functions which enforce that the given type is distinct.
//! fn assert_is_distinct<T: Distinct + ?Sized>() {}
//! fn assert_is_non_distinct<T: NonDistinct + ?Sized>() {}
//!
//! assert_is_distinct::<(u32, u64)>();
//! assert_is_non_distinct::<(u32, u32)>();
//! ```

#![feature(negative_impls)]

use std::marker::PhantomData;

/// This struct exists for stuff internal to the module. Don't use it
/// for anything (though it has no behavior, so you can't do anything
/// with it).
pub struct PairOfPhantoms<T, U> {
    t: PhantomData<T>,
    u: PhantomData<U>,
}

impl<T> !Sync for PairOfPhantoms<T, T> {}

/// A trait which indicates that the two types in the tuple are
/// distinct.
///
/// See the [crate docs](../distinct/index.html) for more information
pub trait Distinct {}

/// A trait which indicates that the two types in the tuple are not
/// distinct.
///
/// See the [crate docs](../distinct/index.html) for more information
pub trait NonDistinct {}

impl<T, U> Distinct for (T, U) where PairOfPhantoms<T, U>: Sync {}

impl<T> NonDistinct for (T, T) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is_distinct<T: Distinct + ?Sized>() {}
    fn assert_is_non_distinct<T: NonDistinct + ?Sized>() {}

    #[test]
    fn test() {
        assert_is_distinct::<(u32, u64)>();
        assert_is_distinct::<(usize, u64)>();

        assert_is_non_distinct::<(u64, u64)>();
        assert_is_non_distinct::<(String, String)>();
    }
}
