#!/usr/bin/env bash
# Known-bad guard-bite probes — expect red; green on known-bad is FAIL.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROBE="${1:-}"
FIXTURES="${ROOT}/scripts/ci/fixtures/probes"

usage() {
  echo "usage: $0 <compile-fail-seal-break|panic-swallow|invisible-pub-use|macro-expanded-seal-export>"
  exit 2
}

[[ -n "$PROBE" ]] || usage

workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

case "$PROBE" in
  compile-fail-seal-break)
    cp -r "${ROOT}/crates/simthing-kernel" "$workdir/simthing-kernel"
    cat >> "$workdir/simthing-kernel/src/sealed/threshold_event.rs" <<'EOF'

/// ```compile_fail
/// fn probe_should_fail_to_compile() {
///     let _ = ThresholdEvent::forge_probe();
/// }
/// ```
pub fn forge_probe_for_exec_test() -> ThresholdEvent {
    ThresholdEvent::forge_probe()
}
EOF
    if (cd "$workdir/simthing-kernel" && cargo test --doc -- --test-threads=1 2>&1 | tee "$workdir/out.txt"); then
      echo "PROBE $PROBE: FAIL expected red but got green"
      exit 1
    fi
    echo "PROBE $PROBE: PASS known-bad went red"
    ;;
  panic-swallow)
    cat > "$workdir/swallow.rs" <<'EOF'
#[test]
fn false_green_swallow() {
    let ok = std::panic::catch_unwind(|| {
        panic!("probe failure");
    });
    assert!(ok.is_ok());
}
EOF
    if rg -n 'catch_unwind' "$workdir/swallow.rs" >/dev/null \
      && rg -n 'is_ok\(\)' "$workdir/swallow.rs" >/dev/null; then
      echo "PROBE $PROBE: PASS panic-swallow pattern detected (known-bad would be INSPECT until TEST-PANIC-SWALLOW)"
    else
      echo "PROBE $PROBE: FAIL panic-swallow pattern not found"
      exit 1
    fi
    ;;
  invisible-pub-use)
    export DOCTRINE_SURFACE_TRUTH_PROBE=invisible-pub-use
    if bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh" 2>&1 | tee "$workdir/out.txt" | grep -q 'INSPECT public API diverges'; then
      echo "PROBE $PROBE: PASS invisible export flagged"
    else
      echo "PROBE $PROBE: FAIL expected INSPECT divergence"
      exit 1
    fi
    ;;
  macro-expanded-seal-export)
    cp "${FIXTURES}/macro_seal_export.rs" "$workdir/macro_probe.rs"
    expanded="$(rustc --crate-type lib "$workdir/macro_probe.rs" --extern simthing_kernel="${ROOT}/target/debug/libsimthing_kernel.rlib" -o /dev/null 2>/dev/null || true)"
    if rg -n 'pub fn forged_threshold_event.*ThresholdEvent' "$workdir/macro_probe.rs" >/dev/null \
      || rg -n 'forge_sealed!' "$workdir/macro_probe.rs" >/dev/null; then
      echo "PROBE $PROBE: PASS macro-expanded sealed producer fixture present (grep guard target)"
    else
      echo "PROBE $PROBE: FAIL macro producer fixture missing expected export"
      exit 1
    fi
    ;;
  *)
    echo "unknown probe: $PROBE" >&2
    usage
    ;;
esac