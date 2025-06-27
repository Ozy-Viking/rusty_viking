#![allow(dead_code, unused_imports, unused_variables)]

use std::path::Path;

use rusty_viking::{IntoDiagnosticWithLocation, Number, X, Y, Z, tracing::setup_tracing};
use tracing::{Level, error, info, info_span, instrument, level_filters::LevelFilter, trace};
use tracing_subscriber::{
    EnvFilter,
    filter::{Directive, Targets},
    fmt::time::LocalTime,
    util::SubscriberInitExt,
};
use valuable::Valuable;

fn main() -> miette::Result<()> {
    rusty_viking::MietteDefaultConfig::init_set_panic_hook(Some(
        "Welcome to the fun and joys of Rust!".into(),
    ))?;
    let my_app_level = Level::INFO;
    let other_app_level = Level::WARN;
    let _ = setup_tracing(
        module_path!(),
        my_app_level,
        other_app_level,
        Some(vec![("metrics", Level::TRACE)]),
    )?;
    let span = info_span!("main");
    let _guard = span.enter();
    info!("Initialised tracing");
    let point1 = Point3D::new(X(5), Y(6), Z(6));
    let point2 = Point3D::new(X(2), Y(-5), Z(-6));
    add(point1, point2);
    info!("Finishing");
    dbg!(std::env::current_exe().into_diagnostic()?.file_stem());
    Ok(())
}

#[instrument(level = tracing::Level::INFO,  fields(result))]
fn add(a: Point3D, b: Point3D) -> Point3D {
    info!("computing...");
    let result = Point3D {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    };
    tracing::Span::current().record("result", &result.as_value());
    info!("Completed calculation.");
    info!(target: "not_my_app", "it's a me, mario");
    result
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Valuable)]
struct Magnatude(pub Number);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Valuable)]
struct Point3D {
    x: X,
    y: Y,
    z: Z,
}

impl Point3D {
    #[instrument(name = "Point3D::new")]
    fn new(x: X, y: Y, z: Z) -> Self {
        info!("Creating new instance");
        Self { x, y, z }
    }
}
