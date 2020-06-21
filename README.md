distinct
========

[![Crates.io Page](https://img.shields.io/crates/v/distinct)][crates]
[![Docs.rs Page](https://docs.rs/distinct/badge.svg)][docs]

Two traits for enforcing that two types either must be the same or must
be different.

- [Source Code][repo]
- [Documenation][docs]
- [Crates.io page][crates]

## Usage

To use in your crate, add this to your `Cargo.toml`:
```toml
[dependencies]
distinct = "0.1.0"
```

and add this where you want to use it:
```rust
use distinct::{Distinct, NonDistinct};
```

For full documentation on how to use it, consult this crate's
[documentation on docs.rs][docs].

[crates](https://crates.io/crates/distinct)
[docs](https://docs.rs/distinct)
[repo](https://github.com/JarredAllen/distinct)
