//! # const it!
//!
//! This crate provides some utilities for use in const evaluation contexts, in particular
//! const slicing and error handling.
//!
//! The [`slice!`] and [`try_slice!`] macros slice (using any usize or range expression):
//! ```rust
//! # use const_it::slice;
//! const STR: &str = slice!("const slice", ..5); // "const"
//! ```
//!
//! The [`split_slice_at!`] and [`try_split_slice_at!`] macros split a slice in two:
//! ```rust
//! # use const_it::split_slice_at;
//! const STR: (&str, &str) = split_slice_at!("const slice", 5); // ("const", " slice")
//! ```
//!
//! The [`ok!`], [`expect_ok!`], [`unwrap_ok!`], [`unwrap_ok_or_return!`], [`expect_some!`], [`unwrap_some!`]
//! and [`unwrap_some_or_return!`] macros work with `Result`s and `Option`s.
//!
//! The [`bytes_cmp`], [`bytes_eq`], [`str_cmp`] and [`str_eq`] functions compare byte
//! slices and strings.

#![no_std]

use core::cmp::Ordering;

/// Turn a `Result` into an `Option`.
#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        match $expr {
            Ok(ok) => Some(ok),
            Err(_) => None,
        }
    };
}

/// Slice an item in a const context. The first argument is the item to slice, and
/// the second is the slice index, which can be a usize or any usize range type.
/// Panics if the index is out of range or, for strings, if the slice would split a
/// unicode codepoint.
///
/// Alternately use [`try_slice!`] to get an `Option` instead of panicing.
///
/// ```rust
/// # use { const_it::slice, core::ops::Range };
/// const STR: &str = slice!("const slice", ..5); // "const"
/// const BYTES: &[u8] = slice!(b"01234", 1..=3); // b"123"
/// const RANGE: Range<usize> = (BYTES[0] - b'0') as usize..(BYTES[2] - b'0') as usize;
/// const STR2: &str = slice!(STR, RANGE); // "on"
/// ```
#[macro_export]
macro_rules! slice {
    ($slicable:expr, $index:expr) => {{
        let _ = $crate::SliceTypeCheck($slicable, $index);
        $crate::Slice($slicable, $index).index()
    }};
}

/// Slice an item in a const context. The first argument is the item to slice, and
/// the second is the slice index, which can be a usize or any usize range type.
/// Returns `Some(sliced)`, or `None` if the index is out of range or, for strings,
/// if the slice would split a unicode codepoint.
///
/// Alternately use [`slice!`] if you want to panic on error instead.
///
/// ```rust
/// # use { const_it::{try_slice, unwrap_some}, core::ops::Range };
/// const STR: Option<&str> = try_slice!("const slice", ..5); // Some("const")
/// const BYTES: Option<&[u8]> = try_slice!(b"01234", 1..=3); // Some(b"123")
/// const BYTES2: &[u8] = unwrap_some!(BYTES); // b"123"
/// const RANGE: Range<usize> = (BYTES2[0] - b'0') as usize..(BYTES2[2] - b'0') as usize;
/// const STR2: Option<&str> = try_slice!(unwrap_some!(STR), RANGE); // Some("on")
/// ```
#[macro_export]
macro_rules! try_slice {
    ($slicable:expr, $index:expr) => {{
        let _ = $crate::SliceTypeCheck($slicable, $index);
        $crate::Slice($slicable, $index).get()
    }};
}

/// Split a slice in two at the specified index. Panics on error.
///
/// See also [`try_split_slice_at!`].
#[macro_export]
macro_rules! split_slice_at {
    ($slicable:expr, $index:expr) => {{
        let _: usize = $index;
        $crate::Slice($slicable, $index).split()
    }};
}

/// Split a slice in two at the specified index. Returns `None` on error.
///
/// See also [`split_slice_at!`].
#[macro_export]
macro_rules! try_split_slice_at {
    ($slicable:expr, $index:expr) => {{
        let _: usize = $index;
        $crate::Slice($slicable, $index).try_split()
    }};
}

/// Takes a `Result` and returns the unwrapped `Ok` value, or panics if it's `Err`.
/// The second argument is the message to use on panic. If the panic message
/// is omitted, the `Err` value must be of type `&str` and is used as the panic message.
///
/// See also [`expect_some!`] and [`unwrap_ok!`].
#[macro_export]
macro_rules! expect_ok {
    ($expr:expr) => {
        match $expr {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(err) => panic!("{}", err),
        }
    };

    ($expr:expr, $message:expr) => {
        match $expr {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(_) => panic!("{}", $message),
        }
    };
}

/// Takes an `Option` and returns the unwrapped `Some` value, or panics if it's `None`.
/// The second argument is the message to use on panic.
///
/// See also [`expect_ok!`] and [`unwrap_some!`].
#[macro_export]
macro_rules! expect_some {
    ($expr:expr, $message:expr) => {
        match $expr {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => panic!("{}", $message),
        }
    };
}

/// Takes a `Result` and returns the unwrapped `Ok` value, or panics if it's `Err`.
///
/// See also [`unwrap_some!`] and [`expect_ok!`].
#[macro_export]
macro_rules! unwrap_ok {
    ($expr:expr) => {
        $crate::expect_ok!($expr, "unwrapped Err value")
    };
}

/// Takes an `Option` and returns the unwrapped `Some` value, or panics if it's `None`.
///
/// See also [`unwrap_ok!`] and [`expect_some!`].
#[macro_export]
macro_rules! unwrap_some {
    ($expr:expr) => {
        $crate::expect_some!($expr, "unwrapped None value")
    };
}

/// Takes a `Result` and evaluates to the unwrapped `Ok` value, or if it's `Err`, returns the `Err`
/// to the current function's caller.
///
/// See also [`unwrap_some_or_return!`].
#[macro_export]
macro_rules! unwrap_ok_or_return {
    ($expr:expr) => {
        match $expr {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(err) => return ::core::result::Result::Err(err),
        }
    };
}

/// Takes an `Option` and evaluates to the unwrapped `Some` value, or if it's `None`, returns the `None`
/// to the current function's caller.
///
/// See also [`unwrap_ok_or_return!`].
#[macro_export]
macro_rules! unwrap_some_or_return {
    ($expr:expr) => {
        match $expr {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => return ::core::option::Option::None,
        }
    };
}

mod slice;

#[doc(hidden)]
pub use slice::SliceTypeCheck;
pub use slice::{Slice, SliceIndex};

#[cfg(test)]
mod tests;

/// Compare two byte slices
pub const fn bytes_cmp(a: &[u8], b: &[u8]) -> Ordering {
    let len = a.len();
    if len < b.len() {
        return Ordering::Less;
    } else if len > b.len() {
        return Ordering::Greater;
    }
    let mut i = 0;
    while i < len {
        if a[i] < b[i] {
            return Ordering::Less;
        } else if a[i] > b[i] {
            return Ordering::Greater;
        }
        i += 1
    }
    Ordering::Equal
}

/// Compare two byte slices for equality
pub const fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    matches!(bytes_cmp(a, b), Ordering::Equal)
}

/// Compare two strings lexicographically by byte values
pub const fn str_cmp(a: &str, b: &str) -> Ordering {
    bytes_cmp(a.as_bytes(), b.as_bytes())
}

/// Compare two strings for equality
pub const fn str_eq(a: &str, b: &str) -> bool {
    bytes_eq(a.as_bytes(), b.as_bytes())
}
