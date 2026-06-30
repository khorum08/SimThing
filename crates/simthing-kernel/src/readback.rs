//! Compile-fail proofs: public POD-to-sealed readback bridges removed (KERNEL-CRATE-EXTRACT-0R2).
//!
//! External crates cannot launder forged GPU POD into sealed events via a public bridge:
//!
//! ```compile_fail
//! fn external_pod_bridge_launder() {
//!     let forged = simthing_kernel::ThresholdEventGpu {
//!         slot: 0,
//!         col: 0,
//!         value: 999.0,
//!         event_kind: 7,
//!     };
//!     let _events = simthing_kernel::threshold_events_from_gpu(&[forged]);
//! }
//! ```
//!
//! External crates cannot mint readback authority and launder forged threshold events:
//!
//! ```compile_fail
//! fn external_mint_then_launder_threshold_event() {
//!     let auth = simthing_kernel::ReadbackAuthority::for_kernel_readback();
//!     let forged = simthing_kernel::ThresholdEventGpu {
//!         slot: 0,
//!         col: 0,
//!         value: 999.0,
//!         event_kind: 7,
//!     };
//!     let _ = simthing_kernel::threshold_events_from_gpu(&[forged], auth);
//! }
//! ```
//!
//! External crates cannot launder forged emission POD without readback authority:
//!
//! ```compile_fail
//! fn external_emission_pod_bridge_launder() {
//!     let forged = simthing_kernel::EmissionRecordGpu {
//!         reg_idx: 0,
//!         emit_count: 99,
//!     };
//!     let _records = simthing_kernel::emission_records_from_gpu(&[forged]);
//! }
//! ```
//!
//! External crates cannot mint readback authority and launder forged emission records:
//!
//! ```compile_fail
//! fn external_mint_then_launder_emission_record() {
//!     let auth = simthing_kernel::ReadbackAuthority::for_kernel_readback();
//!     let forged = simthing_kernel::EmissionRecordGpu {
//!         reg_idx: 0,
//!         emit_count: 99,
//!     };
//!     let _ = simthing_kernel::emission_records_from_gpu(&[forged], auth);
//! }
//! ```
