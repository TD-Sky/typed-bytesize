# Typed Bytesize

[![crates.io](https://img.shields.io/crates/v/typed-bytesize.svg)](https://crates.io/crates/typed-bytesize)
[![docs.rs](https://docs.rs/typed-bytesize/badge.svg)](https://docs.rs/typed-bytesize/)
[![check](https://github.com/TD-Sky/typed-bytesize/actions/workflows/check.yml/badge.svg)](https://github.com/TD-Sky/typed-bytesize/actions/workflows/check.yml)
[![test](https://github.com/TD-Sky/typed-bytesize/actions/workflows/test.yml/badge.svg)](https://github.com/TD-Sky/typed-bytesize/actions/workflows/test.yml)

The library provides two types to represent bytesize:
- [ByteSizeSi](https://docs.rs/typed-bytesize/latest/typed_bytesize/struct.ByteSizeSi.html) for **decimal** prefix unit ([SI](https://en.wikipedia.org/wiki/International_System_of_Units))
- [ByteSizeIec](https://docs.rs/typed-bytesize/latest/typed_bytesize/struct.ByteSizeIec.html) for **binary** prefix unit (IEC 80000-13)

## Functions

- Bytesize types can parse each other's units (e.g. `ByteSizeIec` can parse SI values like `114514GB`);
- Bytesize values will only be formatted as the unit has their owned prefix;
- Bytesize types can be converted to each other;
- Supporting *addition*, *subtraction*, *scalar multiplication* arithmetic operations;
- Optional [serde](https://crates.io/crates/serde) support.

## Example

```rust
use typed_bytesize::{ByteSizeIec, ByteSizeSi};

// SI
assert_eq!(ByteSizeSi::b(114u64), "114".parse().unwrap());
assert_eq!(ByteSizeSi::mb(114), "114MB".parse().unwrap());
print!("{}", ByteSizeSi::kb(310)); // 310.0kB

// IEC
assert_eq!(ByteSizeIec::b(514u64), "514".parse().unwrap());
assert_eq!(ByteSizeIec::mib(514), "514MiB".parse().unwrap());
print!("{}", ByteSizeIec::gib(93696)); // 91.5GiB
```

For more detailed examples, refer to [tests](https://docs.rs/crate/typed-bytesize/latest/source/src/tests.rs).


## BNF

Parsing follows the rule:

```ignore
expr    ::= num | term
term    ::= decimal " "* unit
decimal ::= num | float
float   ::= num "." num
num     ::= [0-9]+
```


## Features

- `serde`: enable [serde](https://crates.io/crates/serde) on `ByteSizeSi` and `ByteSizeIec`.
- `u128`: use `u128` instead of `u64` as inner numeric type to support larger units. (TODO)
