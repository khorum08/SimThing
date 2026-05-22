//! Spec → runtime compilation.
//!
//! Compilers turn authored `*Spec` structures into live SimThing primitives:
//!
//! - [`compile_property`] registers a `SimProperty` with a `DimensionRegistry`.
//! - [`compile_overlay`] builds an `Overlay` instance (caller attaches it).
//! - [`CapabilityTreeBuilder`] compiles a full capability tree spec into a
//!   template `SimThing`, a `CapabilityTreeDefinition`, and the unlock
//!   registrations PR 4 will hand to the feeder.
//!
//! [`CompileContext`] threads the registry through batch compilation of multiple
//! specs from the same `DomainPackSpec` / `GameModeSpec`.

pub mod capability;
pub mod context;
pub mod effect;
pub mod event;
pub mod overlay;
pub mod property;
pub mod trigger;

pub use capability::{CapabilityTreeBuildOutput, CapabilityTreeBuilder};
pub use context::CompileContext;
pub use effect::compile_effect;
pub use event::compile_event;
pub use overlay::compile_overlay;
pub use property::compile_property;
pub use trigger::compile_trigger;
