//! # const it!
//!
//! This crate provides some utilities for use in const evaluation contexts, in particular
//! const slice and error handling.
//!
//! The [`slice!`] and [`try_slice!`] macros slice (using any usize or range expression):
//! ```rust
//! # use const_it::slice;
//! const STR: &str = slice!("const slice", ..5); // "const"
//! ```
//!
//! The [`slice_split_at!`] and [`slice_try_split_at!`] macros split a slice in two:
//! ```rust
//! # use const_it::slice_split_at;
//! const STR: (&str, &str) = slice_split_at!("const slice", 5); // ("const", " slice")
//! ```
//!
//! The [`slice_cmp!`] and [`slice_eq!`] macros compare slices. [`slice_starts_with!`] and
//! [`slice_strip_prefix!`] checks for and strips a prefix, respectively, and
//! [`slice_ends_with!`] and [`slice_strip_suffix!`] do the same for suffixes.
//!
//! The [`ok!`], [`expect_ok!`], [`unwrap_ok!`], [`unwrap_ok_or_return!`], [`expect_some!`], [`unwrap_some!`]
//! and [`unwrap_some_or_return!`] macros work with `Result`s and `Option`s.

#![no_std]

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
        let _ = $crate::__internal::SliceTypeCheck($slicable, $index);
        $crate::__internal::Slice($slicable, $index).index()
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
        let _ = $crate::__internal::SliceTypeCheck($slicable, $index);
        $crate::__internal::Slice($slicable, $index).get()
    }};
}

/// Split a slice in two at the specified index. Panics on error.
///
/// See also [`slice_try_split_at!`].
#[macro_export]
macro_rules! slice_split_at {
    ($slicable:expr, $index:expr) => {{
        let _: ::core::primitive::usize = $index;
        $crate::__internal::Slice($slicable, $index).split()
    }};
}

/// Split a slice in two at the specified index. Returns `None` on error.
///
/// See also [`slice_split_at!`].
#[macro_export]
macro_rules! slice_try_split_at {
    ($slicable:expr, $index:expr) => {{
        let _: ::core::primitive::usize = $index;
        $crate::__internal::Slice($slicable, $index).try_split()
    }};
}

#[doc(hidden)]
#[deprecated = "renamed to slice_split_at"]
#[macro_export]
macro_rules! split_slice_at {
    ($slicable:expr, $index:expr) => {{
        $crate::slice_split_at!($slicable, $index)
    }};
}

#[doc(hidden)]
#[deprecated = "renamed to slice_try_split_at"]
#[macro_export]
macro_rules! try_split_slice_at {
    ($slicable:expr, $index:expr) => {{
        $crate::slice_try_split_at!($slicable, $index)
    }};
}

/// Compare two slices, returning an `Ordering`. This only works for slices of primitive integer types and `str`.
#[macro_export]
macro_rules! slice_cmp {
    ($a:expr, $b:expr) => {
        $crate::__internal::SliceRef($a).cmp($crate::__internal::SliceRef($b))
    };
}

/// Compare two slices, returning an `Option<Ordering>`. Currently all supported types always return `Some`.
/// This only works for slices of primitive integer types and `str`.
#[macro_export]
macro_rules! slice_partial_cmp {
    ($a:expr, $b:expr) => {
        $crate::__internal::SliceRef($a).partial_cmp($crate::__internal::SliceRef($b))
    };
}

/// Check if two slices are equal. This only works for slices of primitive integer types and `str`.
#[macro_export]
macro_rules! slice_eq {
    ($a:expr, $b:expr) => {
        ::core::matches!(
            $crate::slice_partial_cmp!($a, $b),
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        )
    };
}

/// Check if a slice starts with another slice. This only works for slices of primitive integer types and `str`.
#[macro_export]
macro_rules! slice_starts_with {
    ($s:expr, $prefix:expr) => {
        $crate::slice_strip_prefix!($s, $prefix).is_some()
    };
}

/// Check if a slice ends with another slice. This only works for slices of primitive integer types and `str`.
#[macro_export]
macro_rules! slice_ends_with {
    ($s:expr, $prefix:expr) => {
        $crate::slice_strip_suffix!($s, $prefix).is_some()
    };
}

/// Strip a prefix from a slice, returning an Option with the stripped slice on success. This only works for slices of primitive integer types and `str`.
#[macro_export]
macro_rules! slice_strip_prefix {
    ($s:expr, $prefix:expr) => {{
        let (slice, prefix) = (
            $crate::__internal::SliceRef($s),
            $crate::__internal::SliceRef($prefix),
        );
        if slice.len() >= prefix.len() {
            let (pfx, rest) = $crate::slice_split_at!(slice.0, prefix.len());
            if $crate::slice_eq!(pfx, prefix.0) {
                Some(rest)
            } else {
                None
            }
        } else {
            None
        }
    }};
}

/// Strip a suffix from a slice, returning an Option with the stripped slice on success. This only works for slices of primitive integer types and `str`.
#[macro_export]
macro_rules! slice_strip_suffix {
    ($s:expr, $suffix:expr) => {{
        let (slice, suffix) = (
            $crate::__internal::SliceRef($s),
            $crate::__internal::SliceRef($suffix),
        );
        if slice.len() >= suffix.len() {
            let (rest, suff) = $crate::slice_split_at!(slice.0, slice.len() - suffix.len());
            if $crate::slice_eq!(suff, suffix.0) {
                Some(rest)
            } else {
                None
            }
        } else {
            None
        }
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
pub mod __internal {
    pub use super::slice::{Slice, SliceIndex, SliceRef, SliceTypeCheck};
}

#[cfg(test)]
mod tests;
