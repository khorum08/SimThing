//! The Feeder — bridge between authoritative gameplay state and the GPU
//! evaluation pipeline.
//!
//! Per design_v4.md §11 the feeder is three sub-roles:
//!
//! - [`TransformPatcher`] — continuous within-day. Drains `PatchTransform`
//!   work items, resolves `SubFieldRole → col` via `DimensionRegistry`,
//!   writes into a CPU shadow of the `values` buffer.
//! - [`DispatchCoordinator`] — continuous. Uploads dirty rows, sequences
//!   GPU passes 0/1/2/3/7, reads threshold events, advances the
//!   tick/day counters. Signals boundary completion.
//! - [`TreeMaintainer`] — day-boundary only. Owns structural mutation
//!   (slot alloc, reparenting, `AddDimension`). Scaffolded today;
//!   execution lands in `simthing-sim`.
//!
//! ## Topology
//!
//! ```text
//!   gameplay / AI / events
//!            │
//!     FeederSender (Clone, mpsc)
//!            │
//!   ─────────┼──── feeder thread ─────────────────────────────
//!            │
//!     FeederReceiver
//!            │
//!     ┌──────┴──────────────┐
//!     │ TransformPatcher    │  drains → mutates shadow
//!     │  ├─ patches → shadow│
//!     │  └─ boundary → park │
//!     └──────┬──────────────┘
//!            │ (each tick)
//!     ┌──────┴──────────────┐
//!     │ DispatchCoordinator │  uploads dirty rows, runs passes
//!     │  └─ tick(): 0→1→2→3→7  + readback events
//!     └──────┬──────────────┘
//!            │ (each day boundary)
//!     ┌──────┴──────────────┐
//!     │ TreeMaintainer      │  executes parked BoundaryRequests
//!     └─────────────────────┘
//! ```
//!
//! ## What this crate deliberately does *not* do
//!
//! - **Does not own the SimThing tree.** That lives in the upcoming
//!   `simthing-sim` crate, which is also where the §10 day-boundary
//!   protocol orchestration will live (overlay lifecycle, property expiry,
//!   fission/fusion execution).
//! - **Does not build Pass 3 overlay deltas.** `overlay_prep::build_overlay_deltas`
//!   is called by the day-boundary driver in `simthing-sim` (because that's
//!   when the tree changes shape) and the resulting buffer is reused across
//!   ticks within a day.
//! - **Does not spawn OS threads.** The struct names use "thread" terminology
//!   to match the design doc, but the actual `std::thread::spawn` happens in
//!   the top-level driver. This crate is the data-plane logic; thread
//!   placement is a policy decision the driver makes.

pub mod dispatcher;
pub mod maintainer;
pub mod patcher;
pub mod work;

pub use dispatcher::{DispatchCoordinator, TickOutcome};
pub use maintainer::{MaintainerOutcome, TreeMaintainer};
pub use patcher::{PatcherStats, TransformPatcher};
pub use work::{
    feeder_channel, BoundaryRequest, FeederError, FeederReceiver, FeederSender, FeederWork,
    PatchTransform, PlayerIntentOverlay,
};
