#![cfg_attr(not(feature = "std"), no_std)]

extern crate arrayvec;
#[macro_use]
extern crate cfg_if;
extern crate libm;

#[cfg(all(not(feature = "std"), test))]
#[macro_use]
extern crate std;

mod lexer;
mod parser;
pub mod types;

pub use parser::Parser;
pub use types::Gcode;

pub fn parse<'input>(src: &'input str) -> impl Iterator<Item = Gcode> + 'input {
    Parser::new(src).flat_map(|block| block.into_commands())
}
