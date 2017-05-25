//! A crate for parsing gcodes without relying on `std`.

#![no_std]
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
extern crate log;
extern crate arrayvec;

pub mod parser;
pub mod lexer;
mod helpers;

pub use lexer::Span;
pub use parser::BasicParser;
pub use errors::*;

mod errors {
    use super::*;

    /// An alias for the `Result` type.
    pub type Result<T> = ::core::result::Result<T, Error>;

    /// The error type.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Error {
        /// Encountered an unknown token at a particular location.
        UnknownToken(char, Span),
        /// Reached the end of input, unexpectedly.
        UnexpectedEOF,

        /// A syntax error and its location.
        SyntaxError(&'static str, Span),
    }
}
