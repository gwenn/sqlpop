#![allow(unreachable_patterns)]
#![warn(clippy::large_stack_frames)]

#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod tok;
