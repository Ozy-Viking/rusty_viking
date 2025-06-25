#![cfg_attr(docsrs, feature(doc_cfg))]
//! To add this library simply use
//!
//! ```sh
//! cargo add rusty_vikings
//! ```
//!
//! ## Features
//! - `"default"`: All features are enabled by default.
//! - `"serde"`
//! - `"miette"`
//! - `"macros"`: Re-exports [mod@viking_macros]
//!
//! ```sh
//! cargo add rusty_vikings --no-default-features
//! ```
pub(crate) mod error;
#[cfg(feature = "error")]
pub use error::*;
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use viking_macros::*;
