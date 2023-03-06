//! Expect with formatting

#![warn(missing_docs)]

use std::fmt::Debug;

/// Adds `expect_with()`. It is implemented for any `Option<T>` and `Result<T, E>` where `E` is `Debug` (all std types that has `expect` method).
///
/// This method functions exactly the same as `except()` but evaluates error message only when actual error occurred.
///
/// # Example
/// ```rust
/// # use expect_with::ExpectWith;
/// # fn some_heavy_function() -> String {
/// #   return String::from("42");
/// # }
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #
/// let result: Result<(), String> = Err(String::from("some error"));
/// result.expect_with(|| format!("error {}", some_heavy_function()));
/// #
/// #   Ok(())
/// # }
/// ```
pub trait ExpectWith<T> {
    /// Same as `expect` but evaluates error message only when needed
    #[track_caller]
    fn expect_with<F, S>(self, f: F) -> T
    where
        S: AsRef<str>,
        F: FnOnce() -> S;
}

impl<T, E> ExpectWith<T> for Result<T, E>
where
    E: Debug,
{
    #[inline]
    #[track_caller]
    fn expect_with<F, S>(self, f: F) -> T
    where
        S: AsRef<str>,
        F: FnOnce() -> S,
    {
        match self {
            Ok(t) => t,
            Err(e) => panic!("{}: {:?}", f().as_ref(), e),
        }
    }
}

impl<T> ExpectWith<T> for Option<T> {
    #[inline]
    #[track_caller]
    fn expect_with<F, S>(self, f: F) -> T
    where
        S: AsRef<str>,
        F: FnOnce() -> S,
    {
        match self {
            Some(t) => t,
            None => panic!("{:?}", f().as_ref()),
        }
    }
}
