# const it!

This crate provides some utilities for use in const evaluation contexts, in particular
const slicing and error handling.

The `slice!` and `try_slice!` macros slice (using any usize or range expression):
```rust
const STR: &str = slice!("const slice", ..5); // "const"
```

The `split_slice_at!` and `try_split_slice_at!` macros split a slice in two:
```rust
const STR: (&str, &str) = split_slice_at!("const slice", 5); // ("const", " slice")
```

The `ok!`, `expect_ok!`, `unwrap_ok!`, `unwrap_ok_or_return!`, `expect_some!`, `unwrap_some!`
and `unwrap_some_or_return!` macros work with `Result`s and `Option`s.

See the documentation at [https://docs.rs/const-it](https://docs.rs/const-it)
