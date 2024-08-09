#![allow(clippy::bool_assert_comparison)]

use super::*;
use core::{
    cmp::Ordering,
    ops::{Range, RangeInclusive},
};

macro_rules! cmp_slice {
    ($t:ty, $item:expr, $index:expr) => {{
        const SLICED: $t = slice!($item, $index);
        assert_eq!(SLICED, &$item[$index]);

        const TRY_SLICED: Option<$t> = try_slice!($item, $index);
        assert_eq!(TRY_SLICED, Some(&$item[$index]));
    }};
}

macro_rules! slice_fail {
    ($t:ty, $item:expr, $index:expr) => {{
        const TRY_SLICED: Option<$t> = try_slice!($item, $index);
        assert_eq!(TRY_SLICED, None);
    }};
}

#[test]
fn str_slice() {
    cmp_slice!(&str, "abcde", 1..3);
    cmp_slice!(&str, "abcde", 1..=3);
    cmp_slice!(&str, "abcde", 1..);
    cmp_slice!(&str, "abcde", ..3);
    cmp_slice!(&str, "abcde", ..=3);
    cmp_slice!(&str, "abcde", ..);
    cmp_slice!(&str, "abcde", 3..3);
    cmp_slice!(&str, "abcde", 3..=3);
    cmp_slice!(&str, "✨💖", ..3);
    cmp_slice!(&str, "✨💖", 3..);

    slice_fail!(&str, "abcde", Range { start: 4, end: 3 });
    slice_fail!(&str, "abcde", RangeInclusive::new(4, 3));
    slice_fail!(&str, "✨", 1..);
    slice_fail!(&str, "✨", ..1);
}

#[test]
fn byte_slice() {
    cmp_slice!(&[u8], b"abcde", 1..3);
    cmp_slice!(&[u8], b"abcde", 1..=3);
    cmp_slice!(&[u8], b"abcde", 1..);
    cmp_slice!(&[u8], b"abcde", ..3);
    cmp_slice!(&[u8], b"abcde", ..=3);
    cmp_slice!(&[u8], b"abcde", ..);
    cmp_slice!(&[u8], b"abcde", 3..3);
    cmp_slice!(&[u8], b"abcde", 3..=3);

    slice_fail!(&[u8], b"abcde", Range { start: 4, end: 3 });
    slice_fail!(&[u8], b"abcde", RangeInclusive::new(4, 3));
}

#[test]
fn slice_split_at() {
    const SPLIT: (&str, &str) = slice_split_at!("abcde", 3);
    assert_eq!(SPLIT.0, "abc");
    assert_eq!(SPLIT.1, "de");

    const TRY_SPLIT: Option<(&str, &str)> = slice_try_split_at!("abcde", 9);
    assert_eq!(TRY_SPLIT, None);

    const TRY_SPLIT_2: Option<(&str, &str)> = slice_try_split_at!("✨💖", 2);
    assert_eq!(TRY_SPLIT_2, None);

    const SPLIT_2: (&str, &str) = slice_split_at!("✨💖", 3);
    assert_eq!(SPLIT_2, ("✨", "💖"));
}

#[test]
fn eq() {
    const EMPTY: bool = slice_eq!("", "");
    assert_eq!(EMPTY, true);

    const EQ: bool = slice_eq!("hi", "hi");
    assert_eq!(EQ, true);

    const NEQ: bool = slice_eq!("hi", "ho");
    assert_eq!(NEQ, false);

    const NEQ2: bool = slice_eq!("hi", "hello");
    assert_eq!(NEQ2, false);
}

#[test]
fn cmp() {
    const CMP1: Ordering = slice_cmp!("hi", "hi");
    assert_eq!(CMP1, Ordering::Equal);

    const CMP2: Ordering = slice_cmp!("hi", "ho");
    assert_eq!(CMP2, Ordering::Less);

    const CMP3: Ordering = slice_cmp!("ho", "hi");
    assert_eq!(CMP3, Ordering::Greater);

    const CMP4: Ordering = slice_cmp!("h", "hi");
    assert_eq!(CMP4, Ordering::Less);

    const CMP5: Ordering = slice_cmp!("hi", "h");
    assert_eq!(CMP5, Ordering::Greater);
}

#[test]
fn prefix() {
    const STARTS_WITH: bool = slice_starts_with!("abcde", "ab");
    assert_eq!(STARTS_WITH, true);

    const NOT_STARTS_WITH: bool = slice_starts_with!("abcde", "aba");
    assert_eq!(NOT_STARTS_WITH, false);

    const STRIPPED: Option<&str> = slice_strip_prefix!("abcde", "abc");
    assert_eq!(STRIPPED, Some("de"));

    const NOT_STRIPPED: Option<&str> = slice_strip_prefix!("abcde", "ace");
    assert_eq!(NOT_STRIPPED, None);
}

#[test]
fn suffix() {
    const ENDS_WITH: bool = slice_ends_with!("abcde", "de");
    assert_eq!(ENDS_WITH, true);

    const NOT_ENDS_WITH: bool = slice_ends_with!("abcde", "ee");
    assert_eq!(NOT_ENDS_WITH, false);

    const STRIPPED: Option<&str> = slice_strip_suffix!("abcde", "cde");
    assert_eq!(STRIPPED, Some("ab"));

    const NOT_STRIPPED: Option<&str> = slice_strip_suffix!("abcde", "cdf");
    assert_eq!(NOT_STRIPPED, None);
}
