use core::{
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    str,
};

pub trait Sealed {
    fn __not_object_safe<T>() {}
}

/// This trait is similar to the `SliceIndex` trait in std/core, but it's
/// implemented for array types too.
pub trait SliceIndex<T: ?Sized>: Sealed {
    /// The output type when indexing `T` with this type
    type Output: ?Sized;
}

impl Sealed for usize {}

impl<T> SliceIndex<[T]> for usize {
    type Output = T;
}

impl<T, const N: usize> SliceIndex<[T; N]> for usize {
    type Output = T;
}

macro_rules! impl_si {
    ($($t:ty),* $(,)?) => { $(
        impl Sealed for $t {}

        impl SliceIndex<str> for $t {
            type Output = str;
        }

        impl<T> SliceIndex<[T]> for $t {
            type Output = [T];
        }

        impl<T, const N: usize> SliceIndex<[T; N]> for $t {
            type Output = [T];
        }
    )* };
}

impl_si!(
    Range<usize>,
    RangeFrom<usize>,
    RangeFull,
    RangeInclusive<usize>,
    RangeTo<usize>,
    RangeToInclusive<usize>,
);

pub struct SliceTypeCheck<'a, S: ?Sized, Index: SliceIndex<S>>(pub &'a S, pub Index);

/// A pending slice operation. This can be used to slice `&[T]` and `&str` in a const context
/// with any valid slice index.
///
/// You can use the [`slice!`], [`try_slice!`], [`split_slice_at!`] and [`try_split_slice_at!`]
/// convenience macros instead of using this directly.
///
/// ```rust
/// # use { const_it::Slice, core::ops::Range };
/// const STR: &str = Slice("const slice", ..5).index(); // "const"
/// const BYTES: &[u8] = Slice(b"01234", 1..=3).index(); // b"123"
/// ```
pub struct Slice<'a, S: ?Sized, Index>(pub &'a S, pub Index);

const fn slice<T>(s: &[T], start: usize, end: usize) -> Result<&[T], &'static str> {
    let ptr = s.as_ptr();
    let len = s.len();
    if start > end {
        return Err("slice index start is higher than end");
    }
    if end > len {
        return Err("slice index out of range");
    }
    let new_len = end - start;
    Ok(unsafe {
        // safety: the range has been checked to be valid above
        core::slice::from_raw_parts(ptr.add(start), new_len)
    })
}

const fn slice_inclusive<T>(s: &[T], start: usize, end: usize) -> Result<&[T], &'static str> {
    let ptr = s.as_ptr();
    let len = s.len();
    if start > end {
        return Err("slice index start is higher than end");
    }
    if end >= len {
        return Err("slice index out of range");
    }
    let new_len = end - start + 1;
    Ok(unsafe {
        // safety: the range has been checked to be valid above
        core::slice::from_raw_parts(ptr.add(start), new_len)
    })
}

const fn str_slice(s: &str, start: usize, end: usize) -> Result<&str, &'static str> {
    let bytes = s.as_bytes();
    let sliced = unwrap_ok_or_return!(slice(bytes, start, end));
    if (start < bytes.len() && bytes[start] & 0xc0 == 0x80)
        || (end < bytes.len() && bytes[end] & 0xc0 == 0x80)
    {
        return Err("slice splits utf-8 codepoint");
    }
    Ok(unsafe {
        // safety: the slice was valid utf-8 before and has been checked to not split codepoints
        str::from_utf8_unchecked(sliced)
    })
}

const fn str_slice_inclusive(s: &str, start: usize, end: usize) -> Result<&str, &'static str> {
    let bytes = s.as_bytes();
    let sliced = unwrap_ok_or_return!(slice_inclusive(bytes, start, end));
    if (start < bytes.len() && bytes[start] & 0xc0 == 0x80)
        || (end < usize::MAX && end + 1 < bytes.len() && bytes[end + 1] & 0xc0 == 0x80)
    {
        return Err("slice splits utf-8 codepoint");
    }
    Ok(unsafe {
        // safety: the slice was valid utf-8 before and has been checked to not split codepoints
        str::from_utf8_unchecked(sliced)
    })
}

macro_rules! impl_slice {
    ($(<$(@[$($gen:tt)*])? $slice:ty, $index:ty> $self:ident $imp:block)*) => { $(
        impl<'a $(, $($gen)*)?> Slice<'a, $slice, $index> {
            /// Evaluate this slice operation, or return `None` on error
            pub const fn get(&$self) -> Option<&'a <$index as SliceIndex<$slice>>::Output> {
                ok!($imp)
            }

            /// Evaluate this slice operation, or panic on error
            pub const fn index(&$self) -> &'a <$index as SliceIndex<$slice>>::Output {
                expect_ok!($imp)
            }
        }
    )* };
}

impl<'a, T> Slice<'a, [T], usize> {
    /// Split the slice at the stored index, or panic on error
    pub const fn split(&self) -> (&'a [T], &'a [T]) {
        self.0.split_at(self.1)
    }

    /// Split the slice at the stored index, or return `None` on error
    pub const fn try_split(&self) -> Option<(&'a [T], &'a [T])> {
        if self.1 <= self.0.len() {
            Some(self.split())
        } else {
            None
        }
    }
}

impl<'a, T, const N: usize> Slice<'a, [T; N], usize> {
    /// Split the slice at the stored index, or panic on error
    pub const fn split(&self) -> (&'a [T], &'a [T]) {
        self.0.split_at(self.1)
    }

    /// Split the slice at the stored index, or return `None` on error
    pub const fn try_split(&self) -> Option<(&'a [T], &'a [T])> {
        if self.1 <= self.0.len() {
            Some(self.split())
        } else {
            None
        }
    }
}

impl<'a> Slice<'a, str, usize> {
    /// Split the slice at the stored index, or panic on error
    pub const fn split(&self) -> (&'a str, &'a str) {
        expect_some!(
            self.try_split(),
            "index out of range or inside a unicode codepoint"
        )
    }

    /// Split the slice at the stored index, or return `None` on error
    pub const fn try_split(&self) -> Option<(&'a str, &'a str)> {
        let (a, b) = unwrap_some_or_return!(Slice(self.0.as_bytes(), self.1).try_split());
        if b[0] & 0xc0 == 0x80 {
            None
        } else {
            Some(unsafe {
                // safety: split wasn't in the middle of a codepoint
                (str::from_utf8_unchecked(a), str::from_utf8_unchecked(b))
            })
        }
    }
}

impl_slice! {
    <@[T] [T], usize> self { Ok::<_, &'static str>(&self.0[self.1]) }

    <@[T, const N: usize] [T; N], usize> self { Ok::<_, &'static str>(&self.0[self.1]) }

    <@[T] [T], Range<usize>> self {
        slice(self.0, self.1.start, self.1.end)
    }

    <@[T, const N: usize] [T; N], Range<usize>> self {
        slice(self.0, self.1.start, self.1.end)
    }

    <str, Range<usize>> self {
        str_slice(self.0, self.1.start, self.1.end)
    }

    <@[T] [T], RangeInclusive<usize>> self {
        slice_inclusive(self.0, *self.1.start(), *self.1.end())
    }

    <@[T, const N: usize] [T; N], RangeInclusive<usize>> self {
        slice_inclusive(self.0, *self.1.start(), *self.1.end())
    }

    <str, RangeInclusive<usize>> self {
        str_slice_inclusive(self.0, *self.1.start(), *self.1.end())
    }

    <@[T] [T], RangeFrom<usize>> self {
        slice(self.0, self.1.start, self.0.len())
    }

    <@[T, const N: usize] [T; N], RangeFrom<usize>> self {
        slice(self.0, self.1.start, self.0.len())
    }

    <str, RangeFrom<usize>> self {
        str_slice(self.0, self.1.start, self.0.len())
    }

    <@[T] [T], RangeFull> self {
        Ok::<_, &'static str>(self.0)
    }

    <@[T, const N: usize] [T; N], RangeFull> self {
        Ok::<_, &'static str>(self.0)
    }

    <str, RangeFull> self {
        Ok::<_, &'static str>(self.0)
    }

    <@[T] [T], RangeTo<usize>> self {
        slice(self.0, 0, self.1.end)
    }

    <@[T, const N: usize] [T; N], RangeTo<usize>> self {
        slice(self.0, 0, self.1.end)
    }

    <str, RangeTo<usize>> self {
        str_slice(self.0, 0, self.1.end)
    }

    <@[T] [T], RangeToInclusive<usize>> self {
        slice_inclusive(self.0, 0, self.1.end)
    }

    <@[T, const N: usize] [T; N], RangeToInclusive<usize>> self {
        slice_inclusive(self.0, 0, self.1.end)
    }

    <str, RangeToInclusive<usize>> self {
        str_slice_inclusive(self.0, 0, self.1.end)
    }
}
