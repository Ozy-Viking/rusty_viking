#[cfg(feature = "miette")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "miette", feature = "serde"))))]
pub(crate) mod into_diagnostic_with_location;
#[cfg(feature = "miette")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "miette", feature = "serde"))))]
pub use into_diagnostic_with_location::IntoDiagnosticWithLocation;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub(crate) mod serde_json_error;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use serde_json_error::SerdeError;

#[cfg(feature = "miette")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "miette", feature = "serde"))))]
pub struct MietteDefaultConfig;

#[cfg(feature = "miette")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "miette", feature = "serde"))))]
impl MietteDefaultConfig {
    /// Sets up opinionated defaults for [mod@miette] and calls [miette::set_hook].
    pub fn init(footer: Option<String>) -> miette::Result<(), miette::InstallError> {
        miette::set_hook(Box::new(move |_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .terminal_links(true)
                    .with_cause_chain()
                    .show_related_errors_as_siblings()
                    .unicode(true)
                    .context_lines(3)
                    .tab_width(4)
                    .rgb_colors(miette::RgbColors::Always)
                    .break_words(true)
                    .footer(footer.clone().unwrap_or_default())
                    .force_graphical(true)
                    .build(),
            )
        }))
    }

    /// Uses [MietteDefaultConfig::init] then runs [miette::set_panic_hook].
    pub fn init_set_panic_hook(footer: Option<String>) -> miette::Result<(), miette::InstallError> {
        MietteDefaultConfig::init(footer)?;
        miette::set_panic_hook();
        Ok(())
    }
}
