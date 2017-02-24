# rusticata-macros

[![LICENSE](https://img.shields.io/badge/License-LGPL%20v2.1-blue.svg)](LICENSE)
[![Build Status](https://travis-ci.org/rusticata/rusticata-macros.svg?branch=master)](https://travis-ci.org/rusticata/rusticata-macros)
[![Crates.io Version](https://img.shields.io/crates/v/rusticata-macros.svg)](https://crates.io/crates/rusticata-macros)

## Overview

Helper macros for Rusticata

This crate also contains the serialization support with [nom](https://github.com/Geal/nom)-like syntax.
It allows to create the equivalent of a parser combinator, but for serialization, using generators.

For example, the following code writes some integers to the `s` slice:

```rust
let r = do_gen!(
	(s,0),
	gen_be_u8!(1) >>
	gen_be_u8!(2) >>
	gen_be_u16!(0x0304) >>
	gen_be_u32!(0x05060708)
	);
```

See the documentation for more details and examples.

## Documentation

Crate is documented, do running `cargo doc` will crate the offline documentation.

Reference documentation can be found [here](https://docs.rs/rusticata-macros/)

## Features

Here are the current and planned features, with their status:
- [x] **byte-oriented**: the basic type is `(&[u8],usize)` and generators will work as much as possible on byte array slices
- [ ] **bit-oriented**: address a byte slice as a bit stream

## License

This library is licensed under the GNU Lesser General Public License version 2.1, or (at your option) any later version.
