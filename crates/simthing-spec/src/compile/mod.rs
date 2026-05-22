//! Spec → runtime compilation.
//!
//! Compilers turn authored `*Spec` structures into live SimThing primitives:
//!
//! - [`compile_property`] registers a `SimProperty` with a `DimensionRegistry`.
//! - [`compile_overlay`] builds an `Overlay` instance (caller attaches it).
//!
//! [`CompileContext`] threads the registry through batch compilation of multiple
//! specs from the same `DomainPackSpec` / `GameModeSpec`.

pub mod context;
pub mod overlay;
pub mod property;

pub use context::CompileContext;
pub use overlay::compile_overlay;
pub use property::compile_property;
