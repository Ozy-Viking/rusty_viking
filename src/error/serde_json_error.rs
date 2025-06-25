use miette::{Report, SourceOffset, diagnostic, miette};

#[derive(Debug, serde::Deserialize)]
pub struct Library {
    #[allow(unused)]
    pub name: String,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[diagnostic(code(rusty_vikings::SerdeError))]
#[diagnostic(url(docsrs))]
pub struct SerdeError {
    msg: String,
    cause: serde_json::Error,
    label: String,
    #[source_code]
    input: String,
    #[label("{label}")]
    location: SourceOffset,
    #[help]
    help: Option<String>,
    #[related]
    caller_code: Option<Report>,
    code: Option<String>,
}

impl ::std::fmt::Display for SerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl SerdeError {
    /// Takes the input and the `serde_json::Error` and returns a SerdeError
    /// that can be rendered nicely with miette.
    #[track_caller]
    pub fn from_serde_error(
        input: impl Into<String>,
        cause: serde_json::Error,
        help: Option<&str>,
    ) -> Self {
        let (cat, msg) = cause.parse();
        let input = input.into();
        #[allow(unused_assignments)]
        let mut caller_code: Option<Report> = None;
        #[cfg(debug_assertions)]
        {
            let caller_location = std::panic::Location::caller();
            caller_code = Some(miette!("Error occured at: {}", caller_location));
        }
        let location = SourceOffset::from_location(&input, cause.line(), cause.column());
        Self {
            msg: format!("Json {:?} Error: {}", cat, &cause),
            label: msg,
            cause,
            input,
            location,
            help: help.map(|s| s.to_string()),
            caller_code,
            code: None,
        }
    }
}

trait SerdeJsonParser {
    fn parse(&self) -> (serde_json::error::Category, String);
}

impl SerdeJsonParser for serde_json::Error {
    fn parse(&self) -> (serde_json::error::Category, String) {
        let error_string = self.to_string();
        let classification = self.classify();
        let (l, _) = error_string.split_once(" at").unwrap_or_default();
        (classification, l.into())
    }
}
