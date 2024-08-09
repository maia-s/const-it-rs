use super::*;
use core::ops::{Range, RangeInclusive};

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
fn split_slice_at() {
    const SPLIT: (&str, &str) = split_slice_at!("abcde", 3);
    assert_eq!(SPLIT.0, "abc");
    assert_eq!(SPLIT.1, "de");

    const TRY_SPLIT: Option<(&str, &str)> = try_split_slice_at!("abcde", 9);
    assert_eq!(TRY_SPLIT, None);

    const TRY_SPLIT_2: Option<(&str, &str)> = try_split_slice_at!("✨💖", 2);
    assert_eq!(TRY_SPLIT_2, None);

    const SPLIT_2: (&str, &str) = split_slice_at!("✨💖", 3);
    assert_eq!(SPLIT_2, ("✨", "💖"));
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn eq() {
    const EQ: bool = str_eq("hi", "hi");
    assert!(EQ);

    const NEQ: bool = str_eq("hi", "ho");
    assert!(!NEQ);

    const NEQ2: bool = str_eq("hi", "hello");
    assert!(!NEQ2);
}

#[test]
fn cmp() {
    const CMP1: Ordering = str_cmp("hi", "hi");
    assert_eq!(CMP1, Ordering::Equal);

    const CMP2: Ordering = str_cmp("hi", "ho");
    assert_eq!(CMP2, Ordering::Less);

    const CMP3: Ordering = str_cmp("ho", "hi");
    assert_eq!(CMP3, Ordering::Greater);

    const CMP4: Ordering = str_cmp("h", "hi");
    assert_eq!(CMP4, Ordering::Less);

    const CMP5: Ordering = str_cmp("hi", "h");
    assert_eq!(CMP5, Ordering::Greater);
}
