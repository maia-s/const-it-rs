# const it!

This crate provides some utilities for use in const evaluation contexts, in particular
const slice and error handling.

The `slice!` and `try_slice!` macros slice (using any usize or range expression):
```rust
# use const_it::slice;
const STR: &str = slice!("const slice", ..5); // "const"
```

The `slice_split_at!` and `slice_try_split_at!` macros split a slice in two:
```rust
# use const_it::slice_split_at;
const STR: (&str, &str) = slice_split_at!("const slice", 5); // ("const", " slice")
```

The `slice_cmp!` and `slice_eq!` macros compare slices. `slice_starts_with!` and
`slice_strip_prefix!` checks for and strips a prefix, respectively, and
`slice_ends_with!` and `slice_strip_suffix!` do the same for suffixes.

The `ok!`, `expect_ok!`, `unwrap_ok!`, `unwrap_ok_or_return!`, `expect_some!`, `unwrap_some!`
and `unwrap_some_or_return!` macros work with `Result`s and `Option`s.
