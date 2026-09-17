#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

pub use block::Block;
pub use chunk::Chunk;
pub use region::Region;

mod block;
mod chunk;
mod region;

#[must_use]
pub(crate) const fn floor_i64(a: i64, b: i64) -> i64 {
    let quotient = a / b;
    let remainder = a % b;

    if remainder != 0 && quotient < 0 {
        quotient - 1
    } else {
        quotient
    }
}
