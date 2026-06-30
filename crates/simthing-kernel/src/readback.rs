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
//!
//! External crates cannot obtain a public resolved-values buffer handle for queue writes:
//!
//! ```compile_fail
//! fn external_resolved_queue_write(
//!     queue: &wgpu::Queue,
//!     buffers: &simthing_kernel::ResolvedGpuBuffers,
//! ) {
//!     let bytes = [0u8; 16];
//!     queue.write_buffer(buffers.values(), 0, &bytes);
//! }
//! ```
//!
//! External crates cannot obtain a public write-authority minter:
//!
//! ```compile_fail
//! fn external_write_authority_minter() {
//!     let _ = simthing_kernel::ResolvedWriteAuthority::for_boundary_install();
//! }
//! ```
//!
//! External crates cannot obtain session resolved-values buffer for queue writes:
//!
//! ```compile_fail
//! fn external_session_values_queue_write(
//!     queue: &wgpu::Queue,
//!     session: &simthing_kernel::AccumulatorOpSession,
//! ) {
//!     let bytes = [0u8; 16];
//!     queue.write_buffer(session.values_buffer(), 0, &bytes);
//! }
//! ```
//!
//! External crates cannot obtain EML program node/range buffers for queue writes:
//!
//! ```compile_fail
//! fn external_eml_program_node_write(
//!     queue: &wgpu::Queue,
//!     table: &simthing_kernel::EmlGpuProgramTable,
//! ) {
//!     let bytes = [0u8; 16];
//!     queue.write_buffer(table.node_buffer(), 0, &bytes);
//! }
//! ```
//!
//! ```compile_fail
//! fn external_eml_program_range_write(
//!     queue: &wgpu::Queue,
//!     table: &simthing_kernel::EmlGpuProgramTable,
//! ) {
//!     let bytes = [0u8; 16];
//!     queue.write_buffer(table.range_buffer(), 0, &bytes);
//! }
//! ```
//!
//! External crates cannot obtain input-list buffer for queue writes:
//!
//! ```compile_fail
//! fn external_input_list_buffer_write(
//!     queue: &wgpu::Queue,
//!     table: &simthing_kernel::AccumulatorInputListTable,
//! ) {
//!     let bytes = [0u8; 16];
//!     queue.write_buffer(table.buffer(), 0, &bytes);
//! }
//! ```
//!
//! ```compile_fail
//! fn external_input_list_field_write(
//!     queue: &wgpu::Queue,
//!     table: &simthing_kernel::AccumulatorInputListTable,
//! ) {
//!     let bytes = [0u8; 16];
//!     queue.write_buffer(&table.buffer, 0, &bytes);
//! }
//! ```
