#!/usr/bin/env bash
# CI-B-SURFACE-TRUTH-0: compiler-derived public API diff for simthing-kernel (report-only).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASELINE="${ROOT}/scripts/ci/kernel_public_api_baseline.txt"
CURRENT="${ROOT}/scripts/ci/.doctrine_exec_public_api_current.txt"
PROBE_MODE="${DOCTRINE_SURFACE_TRUTH_PROBE:-}"

cd "$ROOT"

if ! command -v cargo-public-api >/dev/null 2>&1; then
  echo "SURFACE-TRUTH: INSPECT cargo-public-api not installed"
  exit 0
fi

emit_public_api() {
  local dir="$1"
  (
    cd "$dir"
    cargo +nightly public-api -p simthing-kernel 2>/dev/null | grep '^pub ' || true
  )
}

if [[ "$PROBE_MODE" == "invisible-pub-use" ]]; then
  workdir="$(mktemp -d)"
  trap 'rm -rf "$workdir"' EXIT
  cp -r crates/simthing-kernel "$workdir/simthing-kernel"
  echo 'pub use crate::sealed::threshold_event::ThresholdEvent as InvisibleProbeExport;' >> "$workdir/simthing-kernel/src/lib.rs"
  emit_public_api "$workdir/simthing-kernel" > "$CURRENT"
else
  emit_public_api "$ROOT" > "$CURRENT"
fi

if [[ ! -f "$BASELINE" ]]; then
  echo "SURFACE-TRUTH: INSPECT missing baseline $BASELINE"
  exit 0
fi

if diff -u "$BASELINE" "$CURRENT" >/dev/null 2>&1; then
  echo "SURFACE-TRUTH: PASS public API matches baseline"
  exit 0
fi

echo "SURFACE-TRUTH: INSPECT public API diverges from baseline"
diff -u "$BASELINE" "$CURRENT" | head -n 80 || true
exit 0