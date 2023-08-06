#![warn(clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(
    // Values are too small for truncation or wrap
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    // Sign is already checked and converted to positive
    clippy::cast_sign_loss
)]

mod binary;
mod decimal;
mod util;

pub use binary::*;
pub use decimal::*;
