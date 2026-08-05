#!/usr/bin/env bash
# EML-LN-PRIMITIVE-0 -- pinned Candidate-F qualification artifact presence +
# freshness. NEVER re-executes the admitted-domain sweep (standing Owner ruling: CI runs
# no cargo tests; certification is a phase-boundary LOCAL act).
#
# Freshness links checked statically (remand 5186492955 authority strip):
#   1. the pinned CPU-twin sequence source (constants + eml_ln_pinned_f32)
#      hashes to the qualified pin -- any algorithm edit invalidates;
#   2. the standalone Candidate-F WGSL artifact
#      (crates/simthing-driver/tests/wgsl/eml_ln_cf_candidate.wgsl) hashes to
#      the qualified pin -- production FieldSweep / AccumulatorOp shaders must
#      NOT carry LN helpers or opcode dispatch while proof is failed;
#   3. the qualification module pins the reference digest placeholder slot, and
#      the results artifact doc records the domain size;
#   4. the recorded wgpu/naga versions still match Cargo.lock (shader-compiler
#      half of the trust chain). Driver identity is re-verified by any local
#      GPU referee run -- CI has no GPU and never claims that link.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="${EML_LN_QUAL_ROOT:-$(cd "${SCRIPT_DIR}/../.." && pwd)}"

# Pinned at LNCF Candidate-F rewrite (2026-08-05, DA 5186354130). Re-pin ONLY
# together with a local exhaustive requalification and a new digest in
# eml_ln_qualification.rs. WGSL pin retargeted to standalone artifact under
# remand 5186492955 (production helpers removed).
readonly QUALIFIED_TWIN_SHA256="46d0937ccdac05e455a377351b0b6c0012adda303b17b0f7923d6109496b068d"
readonly QUALIFIED_CANDIDATE_WGSL_SHA256="e2f1de09f154d1211a374da707f7cf27d3bba30acb7525f9994cc9d320f85164"
readonly QUALIFIED_REFERENCE_DIGEST="0x0"
readonly QUALIFIED_DOMAIN_SIZE="2130706432"
readonly QUALIFIED_WGPU_VERSION="22.1.0"

fail() {
  echo "EML-LN-QUALIFICATION-CHECK: FAIL($1)"
  exit 1
}

twin_region() {
  awk '
    /^pub const EML_LN_SEQUENCE_VERSION/ { printing = 1 }
    /^pub fn eml_ln_pinned_f32/ { in_fn = 1 }
    printing { print }
    in_fn && /^}$/ { exit }
  ' "$1" | strip_prose
}

strip_prose() {
  sed -e 's|[[:space:]]*//.*$||' -e '/^[[:space:]]*$/d'
}

sha_of() {
  tr -d '\r' | sha256sum | cut -d' ' -f1
}

check() {
  local core_twin="${REPO_ROOT}/crates/simthing-core/src/eml_ln.rs"
  local qual_module="${REPO_ROOT}/crates/simthing-kernel/src/eml_ln_qualification.rs"
  local candidate_wgsl="${REPO_ROOT}/crates/simthing-driver/tests/wgsl/eml_ln_cf_candidate.wgsl"
  local field_wgsl="${REPO_ROOT}/crates/simthing-kernel/src/shaders/field_sweep.wgsl"
  local ao_wgsl="${REPO_ROOT}/crates/simthing-kernel/src/shaders/accumulator_op.wgsl"
  local results_doc="${REPO_ROOT}/docs/tests/eml_ln_primitive_0_results.md"
  local cargo_lock="${REPO_ROOT}/Cargo.lock"

  [[ -f "$core_twin" ]] || fail "missing-cpu-twin"
  [[ -f "$qual_module" ]] || fail "missing-qualification-module"
  [[ -f "$candidate_wgsl" ]] || fail "missing-candidate-wgsl"
  [[ -f "$field_wgsl" ]] || fail "missing-field-shader"
  [[ -f "$ao_wgsl" ]] || fail "missing-accumulator-shader"
  [[ -f "$results_doc" ]] || fail "missing-results-artifact"

  local twin_sha
  twin_sha="$(twin_region "$core_twin" | sha_of)"
  [[ "$twin_sha" == "$QUALIFIED_TWIN_SHA256" ]] || fail "cpu-twin-sequence-drift:${twin_sha}"

  local candidate_sha
  candidate_sha="$(strip_prose < "$candidate_wgsl" | sha_of)"
  [[ "$candidate_sha" == "$QUALIFIED_CANDIDATE_WGSL_SHA256" ]] \
    || fail "candidate-wgsl-drift:${candidate_sha}"

  # Remand 5186492955: production EvalEML homes must not host LNCF / OP_LN.
  for shader in "$field_wgsl" "$ao_wgsl"; do
    if grep -q "fn eml_ln_pinned(" "$shader" \
      || grep -qE 'OP_LN|EML_OP_LN' "$shader" \
      || grep -q "eml_ln_snap_exp_domain" "$shader"; then
      fail "production-shader-retains-ln:$(basename "$shader")"
    fi
  done

  grep -q "EML_LN_EXHAUSTIVE_REFERENCE_DIGEST: u64 = 0" "$qual_module" \
    || grep -q "EML_LN_EXHAUSTIVE_REFERENCE_DIGEST: u64 = 0x" "$qual_module" \
    || fail "qualification-module-digest-slot-missing"
  grep -q "2_130_706_432" "$qual_module" || fail "qualification-module-domain-missing"
  grep -qi "${QUALIFIED_REFERENCE_DIGEST}" "$results_doc" || fail "results-doc-digest-missing"
  grep -q "${QUALIFIED_DOMAIN_SIZE}" "$results_doc" || fail "results-doc-domain-missing"

  grep -q "wgpu ${QUALIFIED_WGPU_VERSION}" "$qual_module" || fail "toolchain-record-missing"
  awk '/^name = "wgpu"$/{getline; print; exit}' "$cargo_lock" \
    | grep -q "version = \"${QUALIFIED_WGPU_VERSION}\"" \
    || fail "wgpu-version-drift"

  echo "EML-LN-QUALIFICATION-CHECK: PASS"
}

selftest() {
  SELFTEST_TMP="$(mktemp -d)"
  local tmp="$SELFTEST_TMP"
  trap 'rm -rf "${SELFTEST_TMP:-}"' EXIT
  mkdir -p "$tmp/crates/simthing-core/src" \
    "$tmp/crates/simthing-kernel/src/shaders" \
    "$tmp/crates/simthing-driver/tests/wgsl" \
    "$tmp/docs/tests" "$tmp/scripts/ci"
  cp "${REPO_ROOT}/crates/simthing-core/src/eml_ln.rs" "$tmp/crates/simthing-core/src/"
  cp "${REPO_ROOT}/crates/simthing-kernel/src/eml_ln_qualification.rs" \
    "$tmp/crates/simthing-kernel/src/"
  cp "${REPO_ROOT}/crates/simthing-kernel/src/shaders/field_sweep.wgsl" \
    "${REPO_ROOT}/crates/simthing-kernel/src/shaders/accumulator_op.wgsl" \
    "$tmp/crates/simthing-kernel/src/shaders/"
  cp "${REPO_ROOT}/crates/simthing-driver/tests/wgsl/eml_ln_cf_candidate.wgsl" \
    "$tmp/crates/simthing-driver/tests/wgsl/"
  cp "${REPO_ROOT}/docs/tests/eml_ln_primitive_0_results.md" "$tmp/docs/tests/"
  cp "${REPO_ROOT}/Cargo.lock" "$tmp/"

  local verdict
  verdict="$(EML_LN_QUAL_ROOT="$tmp" bash "${BASH_SOURCE[0]}" --check)" \
    || { echo "FAIL clean-fixture-passes"; exit 1; }
  [[ "$verdict" == *PASS* ]] || { echo "FAIL clean-fixture-passes"; exit 1; }
  echo "PASS clean-fixture-passes"

  sed -i 's/0x4C4E_4346/0x4C4E_4347/' "$tmp/crates/simthing-core/src/eml_ln.rs"
  if EML_LN_QUAL_ROOT="$tmp" bash "${BASH_SOURCE[0]}" --check >/dev/null 2>&1; then
    echo "FAIL planted-twin-drift-bites"
    exit 1
  fi
  echo "PASS planted-twin-drift-bites"
  cp "${REPO_ROOT}/crates/simthing-core/src/eml_ln.rs" "$tmp/crates/simthing-core/src/"

  sed -i 's/let y0 = log(x);/let y0 = log2(x);/' \
    "$tmp/crates/simthing-driver/tests/wgsl/eml_ln_cf_candidate.wgsl"
  if EML_LN_QUAL_ROOT="$tmp" bash "${BASH_SOURCE[0]}" --check >/dev/null 2>&1; then
    echo "FAIL planted-candidate-wgsl-drift-bites"
    exit 1
  fi
  echo "PASS planted-candidate-wgsl-drift-bites"
  cp "${REPO_ROOT}/crates/simthing-driver/tests/wgsl/eml_ln_cf_candidate.wgsl" \
    "$tmp/crates/simthing-driver/tests/wgsl/"

  printf '\nfn eml_ln_pinned(x: f32) -> f32 { return log(x); }\n' \
    >>"$tmp/crates/simthing-kernel/src/shaders/field_sweep.wgsl"
  if EML_LN_QUAL_ROOT="$tmp" bash "${BASH_SOURCE[0]}" --check >/dev/null 2>&1; then
    echo "FAIL planted-production-ln-retention-bites"
    exit 1
  fi
  echo "PASS planted-production-ln-retention-bites"

  echo "EML-LN-QUALIFICATION-SELFTEST: PASS"
}

case "${1:---check}" in
  --check) check ;;
  --selftest) selftest ;;
  *)
    echo "usage: bash scripts/ci/eml_ln_qualification_check.sh [--check|--selftest]"
    exit 2
    ;;
esac
