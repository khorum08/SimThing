#!/usr/bin/env bash
# WRITE-DOOR-BAND-DELTA-0 census: fused write-door + structural remap authority.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

fail() {
  echo "FAIL(write-door-band-delta-census): $1" >&2
  shift
  if [[ $# -gt 0 ]]; then
    printf '%s\n' "$@" >&2
  fi
  exit 1
}

normalize() {
  tr '\\' '/'
}

# ── 1. BandCrossingDelta mint only via sealed kernel door ────────────────────
hits="$(
  rg -n --glob 'crates/**/*.rs' 'BandCrossingDelta\s*\{' \
    | normalize \
    | grep -Ev -- '->[[:space:]]*([[:alnum:]_]+::)*BandCrossingDelta[[:space:]]*\{[[:space:]]*$' \
    | grep -Ev 'crates/simthing-kernel/src/sealed/band_crossing_delta\.rs' \
    | grep -Ev 'compile_fail|doctest|docs/' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "BandCrossingDelta struct literal outside sealed mint door:" "${hits}"
fi
echo "PASS: BandCrossingDelta literals confined to sealed door module"

# ── 2. No production CPU post-hoc invent of threshold crossings (hot path) ───
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' 'patrol_threshold_crossed|cpu_invent_band_crossing|infer_threshold_crossing_from_values' \
    | normalize \
    | grep -Ev 'dress_rehearsal|_oracle\.rs|sealed/|cpu_oracle\.rs|default_schedule_0080|gradient_follow_0080' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "production CPU post-hoc crossing invent outside fenced residues:" "${hits}"
fi
echo "PASS: no unexplained production CPU post-hoc crossing invent symbols"

# ── 3. Structural encode gate symbol must exist ──────────────────────────────
if ! rg -q 'validate_anchor_remap_for_encode|validate_exact_anchor_remap_endpoints' \
  crates/simthing-sim/src crates/simthing-core/src; then
  fail "missing exact/key remap encode gate"
fi
if ! rg -q 'fn expected_anchored_remap_keys' crates/simthing-core/src/anchor_remap.rs; then
  fail "missing expected_anchored_remap_keys (independent pre/post completeness)"
fi
# Required keys must not be seeded from a proposed section (self-certify fence).
hits="$(
  rg -n --glob 'crates/simthing-sim/src/anchor_remap_encode.rs' \
    'required_anchored_loci_for_boundary|expected_anchored_remap_keys' \
    | normalize \
    || true
)"
if [[ -z "${hits}" ]]; then
  fail "anchor_remap_encode.rs missing independent pre/post required-key derivation"
fi
hits="$(
  rg -n --glob 'crates/simthing-sim/src/anchor_remap_encode.rs' \
    'section\.remaps\.iter\(\).*key|for .* in &?section\.remaps' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "required remap keys appear seeded from section.remaps (self-certify risk):" "${hits}"
fi
echo "PASS: remap encode gates present (independent pre/post required keys)"

# ── 4. Boundary flush must consult the remap encode gate ─────────────────────
if ! rg -q 'gate_structural_gpu_encode_exact|validate_exact_anchor_remap_endpoints' \
  crates/simthing-sim/src/boundary.rs; then
  fail "boundary.rs does not call exact remap encode gate before GPU sync"
fi
if ! rg -q 'snapshot_anchored_loci' crates/simthing-sim/src/boundary.rs; then
  fail "boundary.rs does not snapshot pre/post Anchored loci"
fi
echo "PASS: boundary.rs gates structural GPU sync on exact anchor remap"

# ── 5. Remap-free relocation doors must not bypass the gate ──────────────────
hits="$(
  rg -n --glob 'crates/simthing-sim/src/{fission,tree_mutation}.rs' 'sync_gpu_buffers\(' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "remap-free relocation: sync_gpu_buffers called outside boundary gate:" "${hits}"
fi
echo "PASS: fission/tree_mutation do not call sync_gpu_buffers directly"

# ── 6. Zero public production band-delta readback doors; sealed apply mint ───
hits="$(
  rg -n --glob 'crates/**/*.rs' 'fn readback_band_crossing_deltas' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "public production band-delta readback door still present:" "${hits}"
fi
if ! rg -q 'fn apply_band_crossing_deltas_from_fused_emissions' \
  crates/simthing-kernel/src/sealed/band_crossing_delta.rs; then
  fail "missing apply_band_crossing_deltas_from_fused_emissions sealed mint door"
fi
if ! rg -q 'fn apply_band_crossing_deltas_from_threshold_events' \
  crates/simthing-kernel/src/sealed/band_crossing_delta.rs; then
  fail "missing apply_band_crossing_deltas_from_threshold_events sealed mint door"
fi
echo "PASS: no public band-delta readback; sealed apply mint doors present"

# ── 7. Zero fabricated/default remap endpoints ───────────────────────────────
hits="$(
  rg -n --glob 'crates/simthing-sim/src/anchor_remap_encode.rs' \
    'unwrap_or\(SlotIndex::new\(0\)\)|unwrap_or\(ColumnIndex::' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "fabricated/default remap endpoints in encode helpers:" "${hits}"
fi
# Post-hoc-only retire helper must not exist (pre/post derive only).
if rg -q 'fn push_retire_remaps' crates/simthing-sim/src/anchor_remap_encode.rs; then
  fail "push_retire_remaps still present — remaps must derive from pre/post snapshots"
fi
echo "PASS: no fabricated remap endpoints / post-hoc retire helper"

# ── 8. Boundary/replay transport for band deltas + remaps ────────────────────
if ! rg -q 'BandCrossingDeltasApplied' crates/simthing-sim/src/delta_log.rs; then
  fail "delta_log missing BandCrossingDeltasApplied transport"
fi
if ! rg -q 'last_band_crossing_deltas' crates/simthing-sim/src/replay.rs; then
  fail "replay missing bit-exact band-delta retention"
fi
if ! rg -q 'last_anchor_remap' crates/simthing-sim/src/replay.rs; then
  fail "replay missing bit-exact remap retention"
fi
echo "PASS: boundary/replay transport retains remaps + band deltas"

# ── 9. Structural encode inventory (load-bearing map; must remain named) ─────
for path in \
  crates/simthing-sim/src/fission.rs \
  crates/simthing-sim/src/tree_mutation.rs \
  crates/simthing-sim/src/boundary.rs \
  crates/simthing-sim/src/gpu_sync.rs
do
  if [[ ! -f "$path" ]]; then
    fail "missing structural encode inventory path: $path"
  fi
done
echo "PASS: structural encode inventory paths present (fission/tree_mutation/boundary/gpu_sync)"

echo "PASS(write-door-band-delta-census): fused write-door + structural remap census green"
