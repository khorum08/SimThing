#!/usr/bin/env bash
# OH-RELAY-LINT-0 — validate relay/handoff structure; emit RELAY-LINT-VERDICT.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/relay_lint"
readonly COLD_START_FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/cold_start"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

VERDICT=""
SKETCH_TAG="0"
LINT_CLASS="fail"
LINT_TARGET="file"
SELFTEST_FAILURES=0
FIXTURE_DIR=""
PR_NUMBER=""
INPUT_FILE=""
MODE="advisory"

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/relay_lint.sh --selftest
  bash scripts/ci/relay_lint.sh --fixture <name>
  bash scripts/ci/relay_lint.sh --file <path>
  bash scripts/ci/relay_lint.sh --pr <number>
EOF
  exit 1
}

emit_verdict() {
  local kind="$1"
  local detail="${2:-}"
  case "$kind" in
    pass) VERDICT="RELAY-LINT-VERDICT: PASS" ; LINT_CLASS="pass" ;;
    fail) VERDICT="RELAY-LINT-VERDICT: FAIL(${detail})" ; LINT_CLASS="fail" ;;
    inspect) VERDICT="RELAY-LINT-VERDICT: INSPECT(${detail})" ; LINT_CLASS="inspect" ;;
    *) VERDICT="RELAY-LINT-VERDICT: FAIL(harness-error)" ; LINT_CLASS="fail" ;;
  esac
  printf '%s\n' "$VERDICT"
  printf 'relay_lint_class=%s\n' "$LINT_CLASS"
  printf 'sketch=%s\n' "$SKETCH_TAG"
  printf 'target=%s\n' "$LINT_TARGET"
}

lint_text() {
  local text="$1"
  local result
  export RELAY_LINT_FIXTURE_DIR="${FIXTURE_DIR:-}"
  export RELAY_LINT_REPO_ROOT="$REPO_ROOT"
  result="$("$PYTHON_BIN" - <<'PY' "$text"
import hashlib
import os
import pathlib
import re
import sys

text = sys.argv[1]
repo_root = pathlib.Path(os.environ.get("RELAY_LINT_REPO_ROOT", "."))
fixture_dir = os.environ.get("RELAY_LINT_FIXTURE_DIR", "")

def has_section(patterns):
    for pat in patterns:
        if re.search(pat, text, re.IGNORECASE | re.MULTILINE):
            return True
    return False

def section_body(patterns):
    for pat in patterns:
        m = re.search(pat, text, re.IGNORECASE | re.MULTILINE)
        if m:
            start = m.end()
            rest = text[start:]
            nxt = re.search(r'\n##\s+', rest)
            body = rest[: nxt.start()] if nxt else rest
            return body.strip()
    return ""

REQUIRED = [
    ("status", [r'^##\s+Status\b', r'^Status:\s*$', r'^Status:\s*\S']),
    ("pr-merge", [r'^##\s+PR\s*/?\s*branch\s*/?\s*merge', r'^PR\s*/\s*Merge:\s*$', r'^PR\s*/\s*Merge:\s*\S']),
    ("what-changed", [r'^##\s+What changed', r'^What changed:\s*$', r'^What changed:\s*\S']),
    ("load-bearing-proofs", [r'^##\s+Load-bearing proofs', r'^Load-bearing proofs']),
    ("scope-ledger", [r'^##\s+Scope Ledger', r'^Scope Ledger:\s*$', r'^Scope Ledger:\s*\S']),
    ("conformance", [r'^##\s+Conformance', r'^Conformance']),
    ("known-gaps", [r'^##\s+Known gaps', r'^Known gaps']),
    ("graduation-routing", [r'^##\s+Graduation routing', r'^Graduation routing']),
]

missing = []
for key, pats in REQUIRED:
    if not has_section(pats):
        missing.append(key)

if missing:
    if "graduation-routing" in missing:
        print("FAIL:missing-graduation-routing")
        sys.exit(0)
    print("FAIL:empty-required-block")
    sys.exit(0)

grad = section_body([r'^##\s+Graduation routing', r'^Graduation routing'])
grad_lower = grad.lower()
for field, aliases in [
    ("ci-verdict", ["ci verdict"]),
    ("triage-entries", ["triage entries", "triage entry"]),
    ("risk-class", ["risk class"]),
    ("falsification-check", ["falsification check"]),
    ("recommended-posture", ["recommended posture"]),
]:
    if not any(a in grad_lower for a in aliases):
        print("FAIL:missing-graduation-routing")
        sys.exit(0)

if not re.search(r'tested_code_sha\s*[:=]\s*[0-9a-f]{8,}', text, re.IGNORECASE):
    print("FAIL:missing-coverage-basis")
    sys.exit(0)
if not re.search(r'coverage_basis\s*[:=]', text, re.IGNORECASE):
    if not re.search(r'coverage_basis', text, re.IGNORECASE):
        print("FAIL:missing-coverage-basis")
        sys.exit(0)

has_homing = bool(re.search(r'homing boundary\s+classification', text, re.IGNORECASE))
has_scope_class = bool(re.search(r'scope ledger', text, re.IGNORECASE)) and bool(
    re.search(r'scope ledger[\s\S]{0,800}classification', text, re.IGNORECASE)
)
has_lifecycle = bool(
    re.search(
        r'\b(PROBATION|DA-GRADUATED|ORCHESTRATOR-GRADUATED|HOLD|DA-OWNER)\b',
        text,
        re.IGNORECASE,
    )
)
if not ((has_homing or has_scope_class) and has_lifecycle):
    print("FAIL:missing-classification")
    sys.exit(0)

KABUKI_PATTERNS = [
    r'^\s*(tbd|n/?a|todo|\.\.\.|—|-)\s*$',
    r'^\s*$',
]
for key, pats in REQUIRED:
    body = section_body(pats)
    if not body:
        print("FAIL:empty-required-block")
        sys.exit(0)
    lines = [ln.strip() for ln in body.splitlines() if ln.strip()]
    substantive = [
        ln
        for ln in lines
        if not any(re.match(p, ln, re.IGNORECASE) for p in KABUKI_PATTERNS)
        and not re.match(r'^\|?\s*[-|]+\s*\|?\s*$', ln)
    ]
    if len(substantive) < 1:
        print("FAIL:empty-required-block")
        sys.exit(0)

sketch = 0
if re.search(r'§5\.1\s+design[- ]space sketch', text, re.IGNORECASE) or re.search(
    r'^##\s+Design[- ]space sketch', text, re.IGNORECASE | re.MULTILINE
):
    sketch = 1

required_role = ""
if fixture_dir:
    req_path = pathlib.Path(fixture_dir) / "required_receipt_role.txt"
    if req_path.is_file():
        required_role = req_path.read_text(encoding="utf-8").strip().lower()

def current_orientation_state():
    orient_doc = repo_root / "docs" / "orchestrator_orientation.md"
    script_dir = repo_root / "scripts" / "ci"
    if not orient_doc.is_file():
        return None, None, None
    digest_sha = hashlib.sha256(orient_doc.read_bytes()).hexdigest()
    sources = [
        script_dir / "precedented_classes.tsv",
        script_dir / "binding_conditions.tsv",
        script_dir / "clearance_ledger.tsv",
        repo_root / "docs" / "design_0_0_8_4_7_orchestration_harness.md",
        script_dir / "relay_lint.sh",
    ]
    source_stamp = hashlib.sha256(
        "|".join(hashlib.sha256(p.read_bytes()).hexdigest() for p in sources if p.is_file()).encode()
    ).hexdigest()[:16]
    return digest_sha, source_stamp, orient_doc

def expected_receipt(role, digest_sha, source_stamp):
    return hashlib.sha256(
        f"ORIENT-RECEIPT|{role}|{digest_sha}|{source_stamp}".encode("utf-8")
    ).hexdigest()[:12]

def validate_receipt():
    global required_role
    if not required_role:
        risk = re.search(r'risk class[:\s|]+([^\n|]+)', text, re.IGNORECASE)
        if not risk or "gate-wiring" not in risk.group(1).lower():
            return None
        required_role = "coding"
    receipt_m = re.search(r'ORIENT-RECEIPT:\s*([0-9a-f]{12})', text, re.IGNORECASE)
    if not receipt_m:
        return "missing-orient-receipt"
    role_m = re.search(r'^role:\s*([a-z]+)\s*$', text, re.IGNORECASE | re.MULTILINE)
    digest_m = re.search(r'orientation_digest_sha:\s*([0-9a-f]{64})', text, re.IGNORECASE)
    if not role_m or not digest_m:
        return "missing-orient-receipt"
    role = role_m.group(1).lower()
    digest_claim = digest_m.group(1).lower()
    live_digest, live_stamp, _ = current_orientation_state()
    if live_digest is None:
        return "missing-orient-receipt"
    if role != required_role:
        return "wrong-orient-role"
    if digest_claim != live_digest:
        return "stale-orient-receipt"
    expected = expected_receipt(role, live_digest, live_stamp)
    if receipt_m.group(1).lower() != expected:
        return "stale-orient-receipt"
    return None

receipt_fail = validate_receipt()
if receipt_fail:
    print(f"FAIL:{receipt_fail}")
    sys.exit(0)

print(f"PASS:sketch={sketch}")
PY
)"
  local status="${result%%:*}"
  local detail="${result#*:}"
  if [[ "$status" == "PASS" ]]; then
    if [[ "$detail" == sketch=* ]]; then
      SKETCH_TAG="${detail#sketch=}"
    fi
    emit_verdict pass
    return 0
  fi
  emit_verdict fail "${detail#FAIL:}"
  return 0
}

read_input() {
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/relay.md" ]]; then
    LINT_TARGET="file"
    cat "${FIXTURE_DIR}/relay.md"
    return 0
  fi
  if [[ -n "$INPUT_FILE" && -f "$INPUT_FILE" ]]; then
    LINT_TARGET="file"
    cat "$INPUT_FILE"
    return 0
  fi
  if [[ -n "$PR_NUMBER" ]]; then
    if ! command -v gh >/dev/null 2>&1; then
      emit_verdict inspect "gh-unavailable"
      return 1
    fi
    local body evidence combined
    body="$(gh pr view "$PR_NUMBER" --json body -q .body 2>/dev/null || true)"
    evidence=""
    local files
    files="$(gh pr view "$PR_NUMBER" --json files -q '.files[].path' 2>/dev/null || true)"
    local f
    for f in $files; do
      if [[ "$f" == docs/tests/*_results.md ]]; then
        if [[ -f "${REPO_ROOT}/${f}" ]]; then
          evidence="${REPO_ROOT}/${f}"
        fi
      fi
    done
    if [[ -z "$evidence" ]]; then
      for f in docs/tests/oh_relay_lint_0_results.md docs/tests/oh_clearance_router_0_results.md; do
        [[ -f "${REPO_ROOT}/${f}" ]] && evidence="${REPO_ROOT}/${f}" && break
      done
    fi
    combined="$body"
    if [[ -n "$evidence" && -f "$evidence" ]]; then
      combined="${body}

---
EVIDENCE: ${evidence}
---
$(cat "$evidence")"
      LINT_TARGET="results-doc"
    else
      LINT_TARGET="pr-body"
    fi
    printf '%s' "$combined"
    return 0
  fi
  emit_verdict inspect "no-input"
  return 1
}

run_fixture() {
  local root="$1"
  local name="$2"
  FIXTURE_DIR="${root}/${name}"
  [[ -d "$FIXTURE_DIR" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected got text
  expected="$(tr -d '\r' < "${FIXTURE_DIR}/expected_verdict.txt" | head -n 1)"
  text="$(read_input)"
  got="$(lint_text "$text" | head -n 1)"
  if [[ "$got" == "$expected" ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  return 1
}

run_selftest() {
  local relay_fixtures=(
    relay_lint_selftest_pass_1154_shape
    relay_lint_selftest_fail_missing_coverage_basis
    relay_lint_selftest_fail_missing_classification
    relay_lint_selftest_fail_missing_graduation_routing
    relay_lint_selftest_pass_optional_5_1_sketch
    relay_lint_selftest_fail_empty_kabuki_sections
  )
  local cold_fixtures=(
    cold_start_selftest_valid_coding_receipt
    cold_start_selftest_valid_orchestrator_receipt
    cold_start_selftest_fail_missing_receipt
    cold_start_selftest_fail_stale_receipt
    cold_start_selftest_fail_wrong_role
  )
  local name total
  total=$((${#relay_fixtures[@]} + ${#cold_fixtures[@]}))
  for name in "${relay_fixtures[@]}"; do
    FIXTURE_DIR=""
    if ! run_fixture "$FIXTURES_ROOT" "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  for name in "${cold_fixtures[@]}"; do
    FIXTURE_DIR=""
    if ! run_fixture "$COLD_START_FIXTURES_ROOT" "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    SKETCH_TAG="0"
    LINT_TARGET="file"
    emit_verdict pass >/dev/null
    echo "RELAY-LINT-SELFTEST: PASS (${total} fixtures)"
    return 0
  fi
  emit_verdict fail "selftest" >/dev/null
  echo "RELAY-LINT-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

parse_args() {
  [[ $# -gt 0 ]] || usage
  case "${1:-}" in
    --selftest) ;;
    --fixture)
      [[ $# -ge 2 ]] || usage
      FIXTURE_DIR="${FIXTURES_ROOT}/${2}"
      ;;
    --file)
      [[ $# -ge 2 ]] || usage
      INPUT_FILE="$2"
      ;;
    --pr)
      [[ $# -ge 2 ]] || usage
      PR_NUMBER="$2"
      ;;
    --mode)
      [[ $# -ge 2 ]] || usage
      MODE="$2"
      shift
      ;;
    -h|--help) usage ;;
    *)
      if [[ "$1" =~ ^[0-9]+$ ]]; then PR_NUMBER="$1"; else usage; fi
      ;;
  esac
}

main() {
  parse_args "$@"
  if [[ "${1:-}" == "--selftest" ]]; then
    run_selftest
    exit $?
  fi
  local text
  text="$(read_input)" || exit $?
  lint_text "$text"
}

main "$@"