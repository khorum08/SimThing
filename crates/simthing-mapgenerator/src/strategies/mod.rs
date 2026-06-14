//! Registered shape strategy implementations (PR3 seam).

mod elliptical;
mod static_arbitrary;

pub use elliptical::EllipticalStrategy;
pub use static_arbitrary::StaticArbitraryStrategy;

use std::sync::OnceLock;

use crate::strategy::ShapeStrategy;

static ELLIPTICAL: EllipticalStrategy = EllipticalStrategy;
static STATIC: StaticArbitraryStrategy = StaticArbitraryStrategy;

/// Resolve a strategy implementation by registered name (data-driven lookup, not a fixed enum).
pub fn strategy_by_name(name: &str) -> Option<&'static dyn ShapeStrategy> {
    match name {
        "elliptical" => Some(&ELLIPTICAL),
        "static" | "arbitrary_static" => Some(&STATIC),
        _ => None,
    }
}

/// Names with executable PR3 strategies (subset of descriptor registry).
pub fn executable_strategy_names() -> &'static [&'static str] {
    static NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
    NAMES
        .get_or_init(|| vec!["elliptical", "static", "arbitrary_static"])
        .as_slice()
}
