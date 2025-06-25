#![allow(dead_code, unused_imports, unused_variables)]

use std::error::Error;

use miette::Context;
use miette::Diagnostic;
// use miette::IntoDiagnostic as _;
use miette::LabeledSpan;
use miette::Result;
use miette::SourceOffset;
use miette::SourceSpan;
use miette::bail;
use miette::miette;
use miette::set_panic_hook;
use semver::Version;
use thiserror::Error;

use serde_json::{self, json};

use rusty_vikings::IntoDiagnosticWithLocation;
use rusty_vikings::error::serde_json_error::{Library, SerdeError};

fn main() -> Result<()> {
    miette::set_hook(Box::new(|panic_info| {
        // dbg!(panic_info);
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .with_cause_chain()
                .show_related_errors_as_nested()
                .unicode(true)
                .context_lines(3)
                .tab_width(4)
                .rgb_colors(miette::RgbColors::Always)
                .break_words(true)
                .footer("If bug persists raise an issue with maintainer on GitHub.".into())
                .force_graphical(true)
                // .force_narrated(true)
                .build(),
        )
    }))?;

    // // miette::set_panic_hook();
    // let input = serde_json::to_string_pretty(&json!({ "name" : 10})).into_diagnostic()?;

    let input = r#"{"name" = 10}"#;
    let _library: Library = serde_json::from_str(&input)
        .map_err(|cause| SerdeError::from_serde_error(input, cause, None))?;

    // let _library: Library = serde_json::from_str(&input).into_diagnostic()?;
    Ok(())
}
