/*! Intermediate representation shared by the interpreter and the native backend. */

pub mod ir;
pub mod lower;
pub mod stacks;

pub use ir::*;
pub use lower::{lower, lower_with_tests};
