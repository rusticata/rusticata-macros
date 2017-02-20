//! # Rusticata-macros
//!
//! Helper macros for the [rusticata](https://github.com/rusticata) project.


#[macro_use]
extern crate nom;

pub use macros::*;
#[macro_use]
pub mod macros;

pub use gen::*;
#[macro_use]
pub mod gen;
