//! # Runtime for WebAssembly smart contracts

mod encoder;
mod errors;
mod features;
mod mappings;
mod metering;
mod model;
mod parser;

pub use encoder::Encoder;
pub use errors::{WasmarinError, WasmarinResult};
pub use features::Features;
pub use model::{CodeSectionEntry, Model};
pub use parser::Parser;
