pub mod endpoints;
pub mod error;
pub mod client;

/// Re-exports the `Client` struct from the `client` module.
pub use client::Client;
/// Re-exports the `Error` type from the `error` module.
pub use error::Error;
