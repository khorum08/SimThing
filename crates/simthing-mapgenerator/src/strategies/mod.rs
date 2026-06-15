//! Registered shape strategy implementations (PR3 seam, PR8 single-source registry).

mod bar;
mod cartwheel;
mod common;
mod elliptical;
mod registry;
mod ring;
mod spiral;
mod spoked;
mod starburst;
mod static_arbitrary;

pub use elliptical::EllipticalStrategy;
pub use registry::vanilla_entries;
pub use static_arbitrary::StaticArbitraryStrategy;
