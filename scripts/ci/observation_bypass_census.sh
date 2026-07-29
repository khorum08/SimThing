#!/usr/bin/env bash
# ANCHOR-TABLE-SURFACE-0 census: sole observation door + no consumer bypass.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

fail() {
  echo "FAIL(observation-bypass-census): $1" >&2
  shift
  if [[ $# -gt 0 ]]; then
    printf '%s\n' "$@" >&2
  fi
  exit 1
}

normalize() {
  tr '\\' '/'
}

# Reuse WRITE-DOOR remand-4 brace-balanced cfg(test) filter (do not reinvent).
# shellcheck source=lib_cfg_test_mod_spans.sh
source "$(cd "$(dirname "$0")" && pwd)/lib_cfg_test_mod_spans.sh"

# ── 1. Production consumers must not call GpuValuesSnapshot::from_session ────
hits="$(
  rg -n --glob 'crates/**/*.rs' 'GpuValuesSnapshot::from_session' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
if [[ -n "${hits}" ]]; then
  fail "production GpuValuesSnapshot::from_session bypass:" "${hits}"
fi
echo "PASS: no production GpuValuesSnapshot::from_session"

# ── 2. Studio / hosted observation must use AnchorTableSnapshot ──────────────
if ! rg -q 'AnchorTableSnapshot::from_session' \
  crates/simthing-mapeditor/src/studio_live_session_bridge.rs \
  crates/simthing-driver/src/hosted_property_observation.rs; then
  fail "missing AnchorTableSnapshot::from_session on Studio/hosted observation door"
fi
hits="$(
  rg -n --glob 'crates/simthing-mapeditor/src/**/*.rs' 'read_values\(\)' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
if [[ -n "${hits}" ]]; then
  fail "mapeditor production read_values observation bypass:" "${hits}"
fi
echo "PASS: Studio/hosted observation uses AnchorTableSnapshot"

# ── 3. No CPU band reconstruction symbols in production consumers ───────────
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' \
    'reconstruct_band_from_values|infer_band_from_raw|cpu_band_from_matrix' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
if [[ -n "${hits}" ]]; then
  fail "CPU band reconstruction in production:" "${hits}"
fi
echo "PASS: no production CPU band reconstruction symbols"

# ── 4. Anchor table must not enter wire/replay delta enum ────────────────────
hits="$(
  rg -n 'AnchorTable|AnchorTableRow|anchor_table_row' \
    crates/simthing-sim/src/delta_log.rs \
    crates/simthing-sim/src/replay.rs \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "anchor table must not enter wire/replay authority:" "${hits}"
fi
echo "PASS: anchor table absent from wire/replay delta authority"

# ── 5. Falloff params must not land in 5.3 table schema (DA sharpening) ──────
hits="$(
  rg -n --glob 'crates/simthing-core/src/anchor_table.rs' \
    'falloff_params|AdmittedFalloff|pub falloff' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "falloff fields must be absent from 5.3 typed table (DA 5120052669):" "${hits}"
fi
if ! rg -q 'Option<BandIndex>' crates/simthing-core/src/anchor_table.rs; then
  fail "core row must carry Option<BandIndex> (sentinel only at POD encode)"
fi
if ! rg -q 'ANCHOR_BAND_NONE_POD' crates/simthing-kernel/src/sealed/anchor_table.rs; then
  fail "POD sentinel ANCHOR_BAND_NONE_POD missing at encode boundary"
fi
echo "PASS: typed Option<BandIndex>; no falloff fields; POD sentinel at encode"

# ── 6. Mapeditor surface present (5.1 surfaces-omission precedent) ───────────
if ! rg -q 'simthing-mapeditor' handoffs/ANCHOR-TABLE-SURFACE-0.hd.md; then
  fail "handoff surfaces must include crates/simthing-mapeditor"
fi
echo "PASS: handoff lists mapeditor surfaces"

# ── 7. Orch remand 5120259758: no production CPU staging observation door ────
# Consumers must not clone BoundaryProtocol writer staging via .anchor_table().
if ! rg -q 'read_anchor_table' crates/simthing-driver/src/hosted_property_observation.rs; then
  fail "AnchorTableSnapshot::from_session must read GPU via WorldGpuState::read_anchor_table"
fi
hits="$(
  rg -n --glob 'crates/**/*.rs' \
    'proto\.anchor_table\(\)|\.anchor_table\(\)\.clone|BoundaryProtocol::anchor_table' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
if [[ -n "${hits}" ]]; then
  fail "production CPU staging observation bypass (proto.anchor_table / clone):" "${hits}"
fi
# Writer-staging mut/accessor is oracle/test-only; production src must not call it.
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' \
    'writer_staging_anchor_table_for_oracle_or_test|writer_staging_anchor_table_mut_for_oracle_or_test' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
# Allow the definition sites in boundary.rs only.
hits="$(
  printf '%s\n' "${hits}" \
    | grep -v 'crates/simthing-sim/src/boundary.rs' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "production consumer reached writer-staging CPU table:" "${hits}"
fi
echo "PASS: observation door is GPU readback; CPU staging fenced"

# ── 8. Orch remand 5120410047: no CPU shadow writer / full-upload substitute ─
# Dynamic band/value updates must live on the GPU fused maintain path.
# `apply_sealed_band_crossings_to_anchor_table` is oracle-only — banned on boundary.
if rg -q 'apply_sealed_band_crossings_to_anchor_table' crates/simthing-sim/src/boundary.rs; then
  fail "boundary must not apply sealed band crossings onto a CPU observation table"
fi
# Persistent CPU table field / writer-staging accessors must stay gone.
if rg -q 'anchor_table:\s*AnchorTable|writer_staging_anchor_table' crates/simthing-sim/src/boundary.rs; then
  fail "persistent CPU AnchorTable / writer_staging accessors must not return"
fi
# Per-boundary hot-path substitute: refresh+upload after minting band deltas.
# Allowed sites remain admission mint + structural-remap fallback only.
hits="$(
  rg -n 'refresh_anchor_table_magnitudes' crates/simthing-sim/src/boundary.rs \
    | normalize \
    || true
)"
# Count call sites (exclude the import line).
call_hits="$(
  printf '%s\n' "${hits}" \
    | grep -v 'use simthing_core::{' \
    | grep -v 'refresh_anchor_table_magnitudes,' \
    || true
)"
call_count="$(
  printf '%s\n' "${call_hits}" \
    | grep -c 'refresh_anchor_table_magnitudes' \
    || true
)"
if [[ "${call_count}" -gt 2 ]]; then
  fail "refresh_anchor_table_magnitudes call sites exceed admission+remap allowance (${call_count}):" "${call_hits}"
fi
# Forbid the old every-boundary sequence: apply_sealed (already banned) +
# unconditional upload_anchor_table immediately after band_crossing_deltas mint.
if rg -n 'band_crossing_deltas\s*=' crates/simthing-sim/src/boundary.rs \
  | normalize \
  | grep -q .; then
  # Within ~25 lines after the mint, upload_anchor_table must not appear as the
  # dynamic-writer substitute (admission mint when n_anchor_rows==0 is OK).
  python - <<'PY'
from pathlib import Path
text = Path("crates/simthing-sim/src/boundary.rs").read_text(encoding="utf-8")
lines = text.splitlines()
fail = False
for i, line in enumerate(lines):
    if "band_crossing_deltas" in line and "=" in line and "out." in line:
        window = "\n".join(lines[i : i + 25])
        if "upload_anchor_table" in window and "n_anchor_rows == 0" not in window:
            print(f"FAIL near line {i+1}: upload_anchor_table after band_crossing mint without empty-table guard")
            fail = True
if fail:
    raise SystemExit(1)
print("PASS: no per-boundary full-upload substitute after band_crossing mint")
PY
fi
echo "PASS: GPU fused writer authority; no CPU shadow refresh/full-upload hot path"

echo "PASS(observation-bypass-census): all arms green"
