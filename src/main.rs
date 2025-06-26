#![allow(dead_code, unused_imports, unused_variables)]

use std::error::Error;
use std::ops::DerefMut;

use miette::Context;
use miette::Diagnostic;
use miette::LabeledSpan;
use miette::Result;
use miette::SourceOffset;
use miette::SourceSpan;
use miette::bail;
use miette::miette;
use miette::set_panic_hook;
use thiserror::Error;

use serde_json::{self, json};

use rusty_viking::IntoDiagnosticWithLocation;
use rusty_viking::SerdeError;

fn main() -> Result<()> {
    rusty_viking::MietteDefaultConfig::init_set_panic_hook(Some(
        "Welcome to the fun and joys of Rust!".into(),
    ))?;
    panic!("test");

    Ok(())
}
