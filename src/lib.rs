#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate vec_map;
pub use core::{Player, PieceType, Cell};
pub use parser::*;
mod core;

peg_file! parser("sfen.rustpeg");
