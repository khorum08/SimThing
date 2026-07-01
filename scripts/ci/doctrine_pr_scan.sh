#!/usr/bin/env bash
# CI-A-WORKFLOW-0R — PR-delta doctrine scan wrapper (HEURISTIC delta; RELIABLE whole-tree).
set -uo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SCAN_SH="${SCRIPT_DIR}/doctrine_scan.sh"

usage() {
  echo "usage: doctrine_pr_scan.sh <base_sha> <head_sha>" >&2
  echo "       doctrine_pr_scan.sh --prove-delta" >&2
  exit 2
}

run_pr_scan() {
  local base_sha="$1"
  local head_sha="${2:-HEAD}"

  if [[ -z "$base_sha" ]]; then
    echo "doctrine_pr_scan: missing base SHA" >&2
    usage
  fi

  if ! git -C "$REPO_ROOT" rev-parse --verify "${base_sha}^{commit}" >/dev/null 2>&1; then
    echo "doctrine_pr_scan: invalid base SHA: ${base_sha}" >&2
    exit 2
  fi
  if ! git -C "$REPO_ROOT" rev-parse --verify "${head_sha}^{commit}" >/dev/null 2>&1; then
    echo "doctrine_pr_scan: invalid head SHA: ${head_sha}" >&2
    exit 2
  fi

  exec bash "$SCAN_SH" --pr-delta "$base_sha" "$head_sha"
}

prepare_prove_repo() {
  local root="$1"
  mkdir -p "${root}/scripts/ci/allow"
  mkdir -p "${root}/crates/simthing-kernel/src"
  mkdir -p "${root}/crates/simthing-sim/src"
  mkdir -p "${root}/crates/simthing-spec/src"

  cp "${SCRIPT_DIR}/doctrine_scan.sh" \
    "${SCRIPT_DIR}/scans.tsv" \
    "${SCRIPT_DIR}/scan_allowlists.py" \
    "${root}/scripts/ci/"
  cp "${SCRIPT_DIR}/allow/"*.txt "${root}/scripts/ci/allow/"
  cp "${REPO_ROOT}/crates/simthing-kernel/src/lib.rs" \
    "${root}/crates/simthing-kernel/src/lib.rs"
  cp "${REPO_ROOT}/crates/simthing-sim/src/lib.rs" \
    "${root}/crates/simthing-sim/src/lib.rs"
  cat >"${root}/crates/simthing-spec/src/lib.rs" <<'EOF'
// prove sandbox spec stub
EOF

  git -C "$root" init -q
  git -C "$root" config user.email "prove@simthing.local"
  git -C "$root" config user.name "prove"
  git -C "$root" add -A
  git -C "$root" commit -q -m "base"
}

prove_expect() {
  local label="$1"
  local expect_verdict="$2"
  local expect_exit="$3"
  local base_sha="$4"
  local head_sha="$5"
  local root="$6"

  local out="${root}/prove.out"
  local exit_code verdict
  set +e
  (cd "$root" && bash scripts/ci/doctrine_scan.sh --pr-delta "$base_sha" "$head_sha") >"$out" 2>&1
  exit_code=$?
  set -e
  verdict="$(grep 'DOCTRINE-SCAN-VERDICT:' "$out" | tail -1 | sed -n 's/.*DOCTRINE-SCAN-VERDICT: \([A-Z]*\).*/\1/p')"
  if [[ "$verdict" == "$expect_verdict" && "$exit_code" -eq "$expect_exit" ]]; then
    echo "  ${label}  PASS"
    return 0
  fi
  echo "  ${label}  FAIL (verdict=${verdict} exit=${exit_code}, want ${expect_verdict}/${expect_exit})"
  sed -n '1,20p' "$out"
  return 1
}

run_prove_delta() {
  local root failures
  root="$(mktemp -d "${TMPDIR:-/tmp}/simthing-pr-delta-prove-XXXXXX")"
  failures=0

  prepare_prove_repo "$root"
  local base
  base="$(git -C "$root" rev-parse HEAD)"

  echo "PR-delta proof cases"

  # Case 2: PR delta with no HEURISTIC violation -> PASS
  echo '// touch only' >"${root}/crates/simthing-kernel/src/_prove_touch.rs"
  git -C "$root" add crates/simthing-kernel/src/_prove_touch.rs
  git -C "$root" commit -q -m "touch only"
  local head2
  head2="$(git -C "$root" rev-parse HEAD)"
  prove_expect "no heuristic violation -> PASS" "PASS" 0 "$base" "$head2" "$root" || failures=$((failures + 1))

  # Case 4: pre-existing HEURISTIC outside delta must not reappear
  git -C "$root" reset --hard "$base" -q
  cat >"${root}/crates/simthing-kernel/src/_prove_existing.rs" <<'EOF'
pub fn faction_marker() {}
EOF
  git -C "$root" add crates/simthing-kernel/src/_prove_existing.rs
  git -C "$root" commit -q -m "existing heuristic shape"
  local base4
  base4="$(git -C "$root" rev-parse HEAD)"
  echo '// unrelated touch' >"${root}/crates/simthing-kernel/src/_prove_touch.rs"
  git -C "$root" add crates/simthing-kernel/src/_prove_touch.rs
  git -C "$root" commit -q -m "unrelated touch"
  local head4
  head4="$(git -C "$root" rev-parse HEAD)"
  local out4="${root}/prove4.out"
  set +e
  (cd "$root" && bash scripts/ci/doctrine_scan.sh --pr-delta "$base4" "$head4") >"$out4" 2>&1
  set -e
  if grep -E '^  SEMANTIC-WORDS  INSPECT  [1-9]' "$out4" >/dev/null; then
    echo "  baseline heuristic outside delta suppressed  FAIL"
    failures=$((failures + 1))
  else
    echo "  baseline heuristic outside delta suppressed  PASS"
  fi

  # Case 1: HEURISTIC violation in PR delta -> INSPECT exit 0
  git -C "$root" reset --hard "$base" -q
  cat >"${root}/crates/simthing-kernel/src/_prove_heuristic.rs" <<'EOF'
pub struct LaneProbe {
    pub data: [f32; 4],
}

pub fn read_lane(probe: &LaneProbe) -> f32 {
    probe.data[0]
}
EOF
  git -C "$root" add crates/simthing-kernel/src/_prove_heuristic.rs
  git -C "$root" commit -q -m "heuristic violation"
  local head1
  head1="$(git -C "$root" rev-parse HEAD)"
  prove_expect "heuristic violation in delta -> INSPECT" "INSPECT" 0 "$base" "$head1" "$root" || failures=$((failures + 1))

  # Case 3: RELIABLE hard FAIL anywhere -> FAIL exit nonzero
  git -C "$root" reset --hard "$base" -q
  cat >"${root}/crates/simthing-kernel/src/_prove_unsafe.rs" <<'EOF'
pub unsafe fn prove_unsafe() {}
EOF
  git -C "$root" add crates/simthing-kernel/src/_prove_unsafe.rs
  git -C "$root" commit -q -m "reliable violation"
  local head3
  head3="$(git -C "$root" rev-parse HEAD)"
  prove_expect "reliable violation in tree -> FAIL" "FAIL" 1 "$base" "$head3" "$root" || failures=$((failures + 1))

  rm -rf "$root"
  if [[ "$failures" -gt 0 ]]; then
    echo "PR-delta proof: FAIL (${failures} case(s))"
    exit 1
  fi
  echo "PR-delta proof: PASS"
  exit 0
}

main() {
  case "${1:-}" in
    --prove-delta)
      run_prove_delta
      ;;
    -h | --help)
      usage
      ;;
    "")
      usage
      ;;
    *)
      run_pr_scan "$@"
      ;;
  esac
}

main "$@"
