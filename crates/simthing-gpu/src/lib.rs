//! GPU foundation for SimThing.
//!
//! Owns the wgpu device/queue and every persistent GPU buffer the simulation
//! reads or writes. CPU-side preparation, compute pipelines, and the GPU pass
//! sequencer build on top of `WorldGpuState`.

pub mod context;
pub mod passes;
pub mod projection;
pub mod slot;
pub mod world_state;

pub use context::{GpuContext, GpuInitError};
pub use passes::Pipelines;
pub use projection::project_tree_to_values;
pub use slot::SlotAllocator;
pub use world_state::{
    build_governed_pairs, build_intensity_params, GovernedPair, IntensityParams, WorldGpuState,
    CLAMP_BOUNDED, CLAMP_FLOORED, CLAMP_UNBOUNDED,
};
