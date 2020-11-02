# FmtOr

[![Tests](https://github.com/TyPR124/fmtor/workflows/Tests%20MSVC/badge.svg)](https://github.com/TyPR124/fmtor/actions?query=workflow%3A%22stable)
[![crates.io](https://meritbadge.herokuapp.com/fmtor)](https://crates.io/crates/fmtor)
[![docs.rs](https://docs.rs/fmtor/badge.svg)](https://docs.rs/fmtor)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache2 Licensed](https://img.shields.io/badge/license-Apache2-blue.svg)](./LICENSE-APACHE)

An extension trait for easily formatting missing values.

## Example

```rust
use fmtor::FmtOr;

let maybe_box: Option<Box<()>> = None;

println!("The box is at: {:p}", maybe_box.fmt_or("Null"));
```

Prints: *The box is at: Null*
