#![cfg_attr(docsrs, feature(doc_cfg))]
//! To add this library simply use
//!
//! ```sh
//! cargo add rusty_viking
//! ```
//!
//! ## Features
//! - `"default"`: All features are enabled by default.
//! - `"serde"`
//! - `"miette"`
//! - `"macros"`: Re-exports [mod@viking_macros]
//!
//! ```sh
//! cargo add rusty_viking --no-default-features
//! ```
pub(crate) mod error;
#[cfg(feature = "error")]
pub use error::*;
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use viking_macros::*;
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub(crate) mod physics;
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use physics::*;
