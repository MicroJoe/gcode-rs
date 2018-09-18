//! A FFI interface to the `gcode` library.
//!
//! # Error Handling
//!
//! Most functions will return a boolean `success` value to indicate whether
//! they were successful or not.
//!
//! # Examples
//!
//! ```rust
//! use gcode::ffi::{self, SIZE_OF_PARSER, SIZE_OF_GCODE};
//! use gcode::{Parser, Gcode};
//! use std::mem;
//!
//! // allocate some space on the stack for our parser. Normally you'd just use
//! // malloc(), but because we don't have an allocator, we create an
//! // appropriately sized byte buffer and use pointer casts to "pretend"
//! // it's the right thing.
//! let mut parser = [0_u8; SIZE_OF_PARSER];
//! let parser = parser.as_mut_ptr() as *mut Parser;
//!
//! let src = "G01 X-52.4 G4 P50.0";
//!
//! unsafe {
//!     let success = ffi::parser_new(parser, src.as_ptr(), src.len() as i32);
//!     assert!(success, "Creation failed");
//!
//!     let mut gcode_memory = [0; SIZE_OF_GCODE];
//!     let mut code = gcode_memory.as_mut_ptr() as *mut Gcode;
//!     let mut num_gcodes = 0;
//!     let mut cumulative_x = 0.0;
//!     let mut cumulative_y = 0.0;
//!
//!     while ffi::parser_next(parser, code) {
//!         let mut x = 0.0;
//!         if ffi::gcode_arg_value(code, 'X', &mut x) {
//!             cumulative_x += x;
//!         }
//!
//!         let mut y = 0.0;
//!         if ffi::gcode_arg_value(code, 'Y', &mut y) {
//!             cumulative_y += y;
//!         }
//!
//!         num_gcodes += 1;
//!     }
//!
//!     assert_eq!(num_gcodes, 2);
//!     assert_eq!(cumulative_x, -52.4);
//!     assert_eq!(cumulative_y, 0.0);
//! }
//! ```

#![allow(missing_docs, unsafe_code)]

use core::mem;
use core::ptr;
use core::slice;
use core::str;
use parse::Parser;
use types::{Gcode, Mnemonic, Span, Word};

cfg_if!{
    if #[cfg(target_pointer_width = "64")] {
        pub const SIZE_OF_PARSER: usize = 64;
        pub const SIZE_OF_GCODE: usize = 312;
    } else if #[cfg(target_pointer_width = "32")] {
        pub const SIZE_OF_PARSER: usize = 36;
        pub const SIZE_OF_GCODE: usize = 196;
    } else {
        compile_error!("FFI bindings aren't (yet) supported on this platform \
                        because we can't statically determine size of `Parser` \
                        and `Gcode`");
    }
}

/// Create a new parser.
///
/// # Safety
///
/// In order to maintain memory safety, there are two invariants which **must**
/// be upheld:
///
/// 1. The `Parser` must not outlive the source string
/// 2. You must allocate `SIZE_OF_PARSER` bytes (e.g. as an array on the stack)
///    for the `Parser` to be placed in
///
/// If creating the parser was successful.
#[no_mangle]
pub unsafe extern "C" fn parser_new(
    parser: *mut Parser,
    src: *const u8,
    src_len: i32,
) -> bool {
    if src.is_null() || parser.is_null() {
        return false;
    }

    // first, turn the input into a proper UTF-8 string
    let src = slice::from_raw_parts(src, src_len as usize);
    let src = match str::from_utf8(src) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // create our parser, Parser<'input>
    let local_parser = Parser::new(src);

    // Copy it to the destination, passing ownership to the caller
    ptr::write(parser, local_parser);

    true
}

/// Get the next `Gcode`, returning `false` when there are no more `Gcode`s in
/// the input.
#[no_mangle]
pub unsafe extern "C" fn parser_next(
    parser: *mut Parser,
    gcode: *mut Gcode,
) -> bool {
    let parser = &mut *parser;

    match parser.next() {
        Some(got) => {
            ptr::write(gcode, got);
            true
        }
        None => false,
    }
}

/// The overall category this `Gcode` belongs to.
#[no_mangle]
pub unsafe extern "C" fn gcode_mnemonic(gcode: *const Gcode) -> Mnemonic {
    (&*gcode).mnemonic()
}

#[no_mangle]
pub unsafe extern "C" fn gcode_number(gcode: *const Gcode) -> f32 {
    (&*gcode).number()
}

#[no_mangle]
pub unsafe extern "C" fn gcode_major_number(gcode: *const Gcode) -> u32 {
    (&*gcode).major_number()
}

/// The number of arguments in this `Gcode`.
#[no_mangle]
pub unsafe extern "C" fn gcode_num_args(gcode: *const Gcode) -> i32 {
    (&*gcode).args().len() as i32
}

/// Get a pointer to this `Gcode`'s arguments.
#[no_mangle]
pub unsafe extern "C" fn gcode_args(gcode: *const Gcode) -> *const Word {
    (&*gcode).args().as_ptr()
}

/// Get the value for the argument with a particular letter.
#[no_mangle]
pub unsafe extern "C" fn gcode_arg_value(
    gcode: *const Gcode,
    letter: char,
    value: *mut f32,
) -> bool {
    match (&*gcode).value_for(letter) {
        Some(n) => {
            *value = f64::from(n) as f32;
            true
        }
        None => false,
    }
}

/// The `Gcode`'s location in its source code.
#[no_mangle]
pub unsafe extern "C" fn gcode_span(gcode: *const Gcode) -> Span {
    (&*gcode).span()
}

/// Get a `Gcode`'s line number (the `N20` argument), if it was assigned.
#[no_mangle]
pub unsafe extern "C" fn gcode_line_number(
    gcode: *const Gcode,
    line_number: *mut u32,
) -> bool {
    match (&*gcode).line_number() {
        Some(n) => {
            *line_number = n;
            true
        }
        None => false,
    }
}
