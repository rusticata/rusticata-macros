# rusticata-macros

[![LICENSE](https://img.shields.io/badge/License-LGPL%20v2.1-blue.svg)](LICENSE)
[![Build Status](https://travis-ci.org/rusticata/rusticata-macros.svg?branch=master)](https://travis-ci.org/rusticata/rusticata-macros)
[![Crates.io Version](https://img.shields.io/crates/v/rusticata-macros.svg)](https://crates.io/crates/rusticata-macros)

## Overview

Helper macros for Rusticata

This crate contains some additions to [nom](https://github.com/Geal/nom).

For example, the `error_if!` macro allows to test a condition and return an error from the parser if the condition
fails:

```rust
let r = do_parse!(
	s,
	l: be_u8 >>
	error_if!(l < 0, ErrorKind::Custom(128)) >>
	data: take!(l - 4) >>
	);
```

See the documentation for more details and examples.

## Documentation

Crate is documented, do running `cargo doc` will crate the offline documentation.

Reference documentation can be found [here](https://docs.rs/rusticata-macros/)

## License

This library is licensed under the GNU Lesser General Public License version 2.1, or (at your option) any later version.
