//! A gcode parsing library designed for `no_std` environments.
//!
//! # Examples
//!
//! The `gcode` API is extremely minimal, containing a single `parse()`
//! function which takes a string and returns an iterator over the `Gcode`s it
//! contains.
//!
//! Parsing is done on a best-effort basis, with any syntax errors being
//! silently ignored. Because of this, you'll probably want to ensure a file is
//! well formed.
//!
//! > **Note:** The [examples/] folder in this project's repository contains a
//! > simple program that will read an input file and print out any `Gcode`s it
//! > sees.
//!
//! ```rust
//! use gcode::Mnemonic;
//!
//! let src = "O1000
//!     T1 M6
//!     G90
//!     G01 X-75 Y-75 S500 M3
//!     G43 Z100 H1
//!     G01 Z5
//!     N20 G01 Z-20 F100";
//!
//! let mut lines = gcode::parse(src);
//!
//! let program_number = lines.next().unwrap();
//! assert_eq!(program_number.major_number(), 1000);
//!
//! let tool_change = lines.next().unwrap();
//! assert_eq!(tool_change.mnemonic(), Mnemonic::ToolChange);
//! assert_eq!(tool_change.major_number(), 1);
//!
//! // skip the M6 and G90
//! let _ = lines.next();
//! let _ = lines.next();
//!
//! let g01 = lines.next().unwrap();
//! assert_eq!(g01.major_number(), 1);
//! assert_eq!(g01.args().len(), 3);
//! assert_eq!(g01.value_for('X'), Some(-75.0));
//!
//! let rest: Vec<_> = lines.collect();
//! assert_eq!(rest.len(), 4);
//! assert_eq!(rest[3].line_number(), Some(20));
//! ```
//!
//! # FFI Bindings
//!
//! The `ffi` feature (disabled by default) will also generate FFI bindings for
//! calling the library from a non-Rust language. The FFI bindings are unstable
//! and no stability guarantees are made about them.
//!
//! A C-style header file can be generated by running the `cbindgen` program
//! from this repository's root directory.
//!
//! ```console
//! $ cargo install cbindgen
//! $ git clone https://github.com/Michael-F-Bryan/gcode-rs
//! $ cd gcode-rs
//! $ cbindgen --output gcode.h
//! ```
//!
//! When you compile the library, it will automatically generate dynamic and
//! static libraries which can be called from C.
//!
//! ```console
//! $ cargo build --release
//! $ ls target/release/libgcode.*
//! libgcode.a  libgcode.rlib  libgcode.so
//!
//! # Print out some of the available function symbols
//! $ nm target/release/libgcode.so | grep ' T gcode\|parser' | grep -v '_Z'
//! 0000000000005e10 T gcode_args
//! 0000000000005e20 T gcode_arg_value
//! 0000000000005ef0 T gcode_line_number
//! 0000000000005de0 T gcode_major_number
//! 0000000000005dc0 T gcode_mnemonic
//! 0000000000005e00 T gcode_num_args
//! 0000000000005dd0 T gcode_number
//! 0000000000005ed0 T gcode_span
//! 0000000000005f10 T parser_destroy
//! 0000000000005d20 T parser_new
//! 0000000000005d70 T parser_next
//! ```
//!
//! The repository also contains [a basic example] showing how this library can
//! be used from a normal C program.
//!
//! [examples/]: https://github.com/Michael-F-Bryan/gcode-rs/tree/master/examples
//! [a basic example]: https://github.com/Michael-F-Bryan/gcode-rs/blob/master/ffi-example/main.c

#![no_std]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate arrayvec;
#[cfg(feature = "ffi")]
#[macro_use]
extern crate cfg_if;

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(feature = "ffi")]
pub mod ffi;
mod lexer;
mod parse;
mod types;

pub use parse::{parse, Parser};
pub use types::*;

use core::fmt::{self, Write};

/// Print a bunch of gcodes as ASCII text.
pub fn dump<W, I>(mut writer: W, gcodes: I) -> fmt::Result
where
    W: Write,
    I: IntoIterator<Item = Gcode>,
{
    for item in gcodes {
        writeln!(writer, "{}", item)?;
    }

    Ok(())
}
