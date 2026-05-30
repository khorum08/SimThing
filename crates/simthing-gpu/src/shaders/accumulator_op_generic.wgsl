// AO-WGSL-0 — Generic AccumulatorOp WGSL performance gate (semantic-free contract)
//
// Allowed shader concepts:
//   slot indices, dimension indices, band indices, role ids, combine mode ids,
//   gate mode ids, consume mode ids, input/output buffers, strides, clamps,
//   weights, masks, reductions.
//
// Forbidden shader concepts:
//   faction, planet, star, map, AI intent, economy meaning,
//   ClauseThing / ClauseScript semantics, named scenario semantics.
//
// Implementation: the `execute_orderband_bands` entry point in accumulator_op.wgsl
// is the authoritative AO-WGSL-0 kernel (shared helpers with execute_ops).
// Pipeline wiring: crates/simthing-gpu/src/accumulator_op/wgsl_path.rs

struct AoWgsl0GateMarker {
    gate_id: u32,
}
