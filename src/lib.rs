// #![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    // warnings,
    // unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

mod fields;

pub use fields::*;

#[cfg(test)]
mod tests;
