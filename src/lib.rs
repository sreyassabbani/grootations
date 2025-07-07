#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod blades;
mod multivector;

pub use blades::*;
pub use multivector::*;

#[cfg(test)]
mod tests;
