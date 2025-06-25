use miette::SourceOffset;
use miette::{Diagnostic, Report, SourceSpan};
#[derive(Debug, thiserror::Error, Diagnostic)]
pub struct DiagnosticWithLocationError {
    #[source]
    pub source_error: Box<dyn std::error::Error + Send + Sync + 'static>,
    #[label(primary, "Error")]
    pub caller_location: SourceSpan,
    pub msg: Option<String>,
    #[source_code]
    pub source_code: Option<String>,
    #[help]
    pub help: Option<String>,
    #[related]
    pub related: Option<Report>,
}

impl DiagnosticWithLocationError {
    #[allow(dead_code)]
    #[track_caller]
    pub fn new(
        source_error: Box<dyn std::error::Error + Send + Sync + 'static>,
        caller_location: Option<SourceSpan>,
        msg: Option<String>,
        source_code: Option<String>,
        help: Option<String>,
    ) -> Self {
        Self {
            source_error,
            caller_location: caller_location.unwrap_or_else(|| {
                miette::SourceSpan::new(miette::SourceOffset::from_current_location().unwrap().1, 0)
            }),
            msg,
            source_code,
            help,
            related: None,
        }
    }

    #[track_caller]
    pub fn from_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            caller_location: miette::SourceSpan::new(
                miette::SourceOffset::from_current_location().unwrap().1,
                0,
            ),
            msg: None,
            source_error: source,
            source_code: None,
            help: None,
            related: None,
        }
    }
}

impl ::std::fmt::Display for DiagnosticWithLocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.msg {
            Some(msg) => write!(f, "{}", msg),
            None => write!(f, "{}", self.source_error),
        }
    }
}

/// Extension of [`miette::IntoDiagnostic`] trait.
///
/// <div class="warning">
/// Calling any of these method calls will clobber the type if it implements <a href="../../miette/protocol/trait.Diagnostic.html" title="trait miette::protocol::Diagnostic">miette::Diagnostic</a> and reduce it to <a href="https://doc.rust-lang.org/1.86.0/core/error/trait.Error.html" title="trait core::error::Error">std::error::Error</a>.
/// </div>
pub trait IntoDiagnosticWithLocation<T, E> {
    /// Converts [`Result`] types that return regular [`std::error::Error`]s
    /// into a [`Result`] that returns a [`Diagnostic`].
    ///

    fn into_diagnostic(self) -> Result<T, Report>;
    fn into_diagnostic_with_help(self, help: Option<String>) -> Result<T, Report>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> IntoDiagnosticWithLocation<T, E>
    for Result<T, E>
{
    #[track_caller]
    fn into_diagnostic(self) -> Result<T, Report> {
        self.into_diagnostic_with_help(None)
    }

    fn into_diagnostic_with_help(self, help: Option<String>) -> Result<T, Report> {
        let location: &'static std::panic::Location<'static> = ::std::panic::Location::caller();
        let res = self.map_err(|e| Box::new(dbg!(e)));
        if res.is_ok() {
            return Ok(res.unwrap());
        }
        let error: Box<dyn std::error::Error + Send + Sync + 'static> = res.err().unwrap();

        let mut diagnostic_location = DiagnosticWithLocationError::from_error(error);
        diagnostic_location.help = help;
        #[cfg(debug_assertions)]
        {
            let (filepath, offset) =
                SourceOffset::from_current_location().expect("Only accessible from dev build.");

            let file =
                ::std::fs::read_to_string(&filepath).expect("All files should be availble in dev.");
            diagnostic_location.source_code = Some(file);
            diagnostic_location.caller_location = SourceSpan::new(offset, 0);
            diagnostic_location.msg = Some(format!("Error occured at {}", location));
            // diagnostic_location.related = Some(miette::miette!(error))
        }
        Err(Report::from(diagnostic_location))
    }
}
