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
//! - `"physics"`
//! - `"tracing"`
//! - `"valueable"`
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
#[cfg(feature = "physics")]
#[cfg_attr(docsrs, doc(cfg(feature = "physics")))]
pub(crate) mod physics;
#[cfg(feature = "physics")]
#[cfg_attr(docsrs, doc(cfg(feature = "physics")))]
pub use physics::*;

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub mod tracing {
    use tracing_subscriber::util::SubscriberInitExt;

    use crate::IntoDiagnosticWithLocation;

    /// ```
    /// use rusty_viking::tracing::setup_tracing;
    /// use tracing::Level;
    /// fn main() -> miette::Result<()> {
    ///     let my_app_level = Level::INFO;
    ///     let other_app_level = Level::WARN;
    ///     let _ = setup_tracing(
    ///         module_path!(),
    ///         my_app_level,
    ///         other_app_level,
    ///         Some(vec![("metrics", Level::TRACE)]),
    ///     )?;
    ///     Ok(())
    /// }
    /// ```
    pub fn setup_tracing(
        app_name: &'static str,
        app_level: tracing::Level,
        other_app_level: tracing::Level,
        optional_targets: Option<Vec<(&str, tracing::Level)>>,
    ) -> miette::Result<()> {
        #[allow(unused_mut)]
        let mut builder = tracing_subscriber::fmt()
            .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339());
        let mut target = tracing_subscriber::filter::Targets::new()
            .with_target(app_name, app_level)
            .with_default(other_app_level);
        if let Some(ts) = optional_targets {
            target = target.with_targets(ts);
        }
        dbg!(&target.to_string());
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(target.to_string()));

        builder
            .with_env_filter(env_filter)
            .finish()
            .try_init()
            .into_diagnostic_with_help(Some(
                "This will fail if a default subscriber is already set or if it is called twice."
                    .into(),
            ))
    }
}
