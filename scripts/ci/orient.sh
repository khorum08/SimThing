#!/usr/bin/env bash
# OH-COLD-START-0 — role-keyed orientation landing + ORIENT-RECEIPT emission.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly ORIENT_DOC="${REPO_ROOT}/docs/orchestrator_orientation.md"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

ROLE=""
SINCE_RECEIPT=""
FIXTURE_MODE=""
FIXTURE_DIR=""
SELFTEST_FAILURES=0
readonly COLD_START_FIXTURES="${SCRIPT_DIR}/fixtures/cold_start"

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/orient.sh --role=coding
  bash scripts/ci/orient.sh --role=orchestrator
  bash scripts/ci/orient.sh --role=da
  bash scripts/ci/orient.sh --role=coding --since=<receipt>
  bash scripts/ci/orient.sh --selftest
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --role=*)
        ROLE="${1#--role=}"
        shift
        ;;
      --role)
        ROLE="${2:-}"
        [[ -n "$ROLE" ]] || usage
        shift 2
        ;;
      --since=*)
        SINCE_RECEIPT="${1#--since=}"
        shift
        ;;
      --since)
        SINCE_RECEIPT="${2:-}"
        [[ -n "$SINCE_RECEIPT" ]] || usage
        shift 2
        ;;
      --selftest)
        FIXTURE_MODE="selftest"
        shift
        ;;
      -h|--help)
        usage
        ;;
      *)
        usage
        ;;
    esac
  done
}

emit_orientation() {
  local role="$1"
  local fixture_dir="${2:-}"
  ORIENT_ROLE="$role" \
  ORIENT_DOC_PATH="${ORIENT_DOC_OVERRIDE:-$ORIENT_DOC}" \
  ORIENT_REPO_ROOT="$REPO_ROOT" \
  ORIENT_FIXTURE_DIR="$fixture_dir" \
    exec "$PYTHON_BIN" - <<'PY'
import hashlib
import os
import pathlib
import re
import sys

role = os.environ["ORIENT_ROLE"].lower()
orient_doc = pathlib.Path(os.environ["ORIENT_DOC_PATH"])
repo_root = pathlib.Path(os.environ["ORIENT_REPO_ROOT"])
script_dir = repo_root / "scripts" / "ci"
fixture_dir = os.environ.get("ORIENT_FIXTURE_DIR", "")

if role not in ("coding", "orchestrator", "da"):
    print(f"orient.sh: invalid role: {role}", file=sys.stderr)
    sys.exit(2)


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    text = raw.decode("utf-8")
    return text.replace("\r\n", "\n").replace("\r", "\n")


def file_digest(path: pathlib.Path) -> str:
    return hashlib.sha256(normalize_text(path.read_bytes()).encode("utf-8")).hexdigest()


def rule_source_paths():
    return [
        pathlib.Path(
            os.environ.get(
                "ORIENT_PRECEDENTED_CLASSES_TSV",
                str(script_dir / "precedented_classes.tsv"),
            )
        ),
        pathlib.Path(
            os.environ.get(
                "ORIENT_BINDING_CONDITIONS_TSV",
                str(script_dir / "binding_conditions.tsv"),
            )
        ),
        pathlib.Path(
            os.environ.get(
                "ORIENT_DOCTRINE_ANCHORS_TSV",
                str(script_dir / "doctrine_anchors.tsv"),
            )
        ),
    ]


def orientation_rule_stamp() -> str:
    return hashlib.sha256(
        "|".join(file_digest(p) for p in rule_source_paths()).encode("utf-8")
    ).hexdigest()[:16]


def load_fixture_state():
    if not fixture_dir:
        return None
    fix = pathlib.Path(fixture_dir)
    snap = fix / "orientation_snapshot.md"
    state = fix / "orientation_state.txt"
    if not snap.is_file() or not state.is_file():
        return None
    text = normalize_text(snap.read_bytes())
    digest_sha = hashlib.sha256(text.encode("utf-8")).hexdigest()
    stamps = {}
    for line in state.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, val = line.split("=", 1)
            stamps[key.strip()] = val.strip()
    return text, digest_sha, stamps.get("orientation_rule_stamp", stamps.get("source_stamp", ""))


fixture = load_fixture_state()
if fixture:
    text, digest_sha, rule_stamp = fixture
else:
    if not orient_doc.is_file():
        print(f"orient.sh: missing {orient_doc}", file=sys.stderr)
        sys.exit(1)
    text = normalize_text(orient_doc.read_bytes())
    digest_sha = hashlib.sha256(text.encode("utf-8")).hexdigest()
    rule_stamp = orientation_rule_stamp()

receipt = hashlib.sha256(
    f"ORIENT-RECEIPT|{role}|{rule_stamp}".encode("utf-8")
).hexdigest()[:12]

SECTIONS_CODING = {
    "Source Stamps", "Next Rung Pointer", "Clearance Router Verdict Meanings",
    "Precedented Classes (active)", "tested_code_sha + coverage_basis Rule",
    "Inner Loop (coding agent)", "GHA Comment Commands", "Orientation Receipt (ORIENT-RECEIPT)",
}
SECTIONS_DA = {
    "Source Stamps", "OH Track / Rung Summary (0.0.8.4.7)", "Binding Conditions",
    "Clearance Ledger (recent)", "Escalation / DA-RESERVE Posture",
    "Relay Lint Required Blocks", "Orientation Receipt (ORIENT-RECEIPT)",
}

def split_sections(body):
    lines = body.splitlines()
    sections = {"_header": []}
    current = "_header"
    for line in lines:
        if line.startswith("## "):
            current = line[3:].strip()
            sections[current] = [line]
        else:
            sections.setdefault(current, []).append(line)
    return sections

sections = split_sections(text)
header = "\n".join(sections.get("_header", [])).strip()

if role == "orchestrator":
    body_out = text
else:
    wanted = SECTIONS_CODING if role == "coding" else SECTIONS_DA
    parts = [header, ""]
    for name, content in sections.items():
        if name == "_header":
            continue
        if name in wanted:
            parts.append("\n".join(content).rstrip())
            parts.append("")
    body_out = "\n".join(parts).rstrip() + "\n"

print(f"ORIENT-RECEIPT: {receipt}")
print(f"role: {role}")
print(f"orientation_rule_stamp: {rule_stamp}")
print(f"orientation_digest_sha: {digest_sha}")
print("generated_at: source-bound")
print("--- orientation ---")
print(body_out.rstrip())
PY
}

emit_since() {
  local role="$1"
  local receipt="$2"
  ORIENT_ROLE="$role" \
  ORIENT_SINCE_RECEIPT="$receipt" \
  ORIENT_DOC_PATH="${ORIENT_DOC_OVERRIDE:-$ORIENT_DOC}" \
  ORIENT_REPO_ROOT="$REPO_ROOT" \
    exec "$PYTHON_BIN" - <<'PY'
import hashlib
import os
import pathlib
import sys

role = os.environ["ORIENT_ROLE"].lower()
since = os.environ["ORIENT_SINCE_RECEIPT"].lower()
repo_root = pathlib.Path(os.environ["ORIENT_REPO_ROOT"])
orient_doc = pathlib.Path(os.environ["ORIENT_DOC_PATH"])
script_dir = repo_root / "scripts" / "ci"

if role not in ("coding", "orchestrator", "da"):
    print(f"orient.sh: invalid role: {role}", file=sys.stderr)
    sys.exit(2)
if not since or len(since) != 12 or any(ch not in "0123456789abcdef" for ch in since):
    print("orient.sh: --since requires a 12-hex ORIENT-RECEIPT", file=sys.stderr)
    sys.exit(2)

def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")

def file_digest(path: pathlib.Path) -> str:
    return hashlib.sha256(normalize_text(path.read_bytes()).encode("utf-8")).hexdigest()

rule_paths = [
    pathlib.Path(os.environ.get("ORIENT_PRECEDENTED_CLASSES_TSV", str(script_dir / "precedented_classes.tsv"))),
    pathlib.Path(os.environ.get("ORIENT_BINDING_CONDITIONS_TSV", str(script_dir / "binding_conditions.tsv"))),
    pathlib.Path(os.environ.get("ORIENT_DOCTRINE_ANCHORS_TSV", str(script_dir / "doctrine_anchors.tsv"))),
]
rule_stamp = hashlib.sha256("|".join(file_digest(p) for p in rule_paths).encode("utf-8")).hexdigest()[:16]
digest_sha = file_digest(orient_doc) if orient_doc.is_file() else ""
current = hashlib.sha256(f"ORIENT-RECEIPT|{role}|{rule_stamp}".encode("utf-8")).hexdigest()[:12]

if since == current:
    print("ORIENT-SINCE-VERDICT: CURRENT")
else:
    print("ORIENT-SINCE-VERDICT: STALE(rule-source)")
print(f"supplied_receipt: {since}")
print(f"current_receipt: {current}")
print(f"role: {role}")
print(f"orientation_rule_stamp: {rule_stamp}")
print(f"orientation_digest_sha: {digest_sha}  # informational")
PY
}

run_selftest() {
  local fixtures=(
    cold_start_selftest_orient_roles
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_orient_selftest_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if ! run_rule_stamp_selftest; then
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  fi
  if ! run_since_selftest; then
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  fi
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "ORIENT-SELFTEST: PASS ($((${#fixtures[@]} + 2)) fixtures)"
    return 0
  fi
  echo "ORIENT-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

stamp_from_env() {
  "$PYTHON_BIN" - <<'PY'
import hashlib
import os
import pathlib

def norm(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")

def digest(path: str) -> str:
    return hashlib.sha256(norm(pathlib.Path(path).read_bytes()).encode("utf-8")).hexdigest()

paths = [
    os.environ["ORIENT_PRECEDENTED_CLASSES_TSV"],
    os.environ["ORIENT_BINDING_CONDITIONS_TSV"],
    os.environ["ORIENT_DOCTRINE_ANCHORS_TSV"],
]
print(hashlib.sha256("|".join(digest(p) for p in paths).encode("utf-8")).hexdigest()[:16])
PY
}

run_rule_stamp_selftest() {
  local sandbox base changed digest_doc
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-rule-stamp-XXXXXX")"
  cp "${SCRIPT_DIR}/precedented_classes.tsv" "$sandbox/precedented_classes.tsv"
  cp "${SCRIPT_DIR}/binding_conditions.tsv" "$sandbox/binding_conditions.tsv"
  cp "${SCRIPT_DIR}/doctrine_anchors.tsv" "$sandbox/doctrine_anchors.tsv"
  cp "$ORIENT_DOC" "$sandbox/orientation.md"

  export ORIENT_PRECEDENTED_CLASSES_TSV="$sandbox/precedented_classes.tsv"
  export ORIENT_BINDING_CONDITIONS_TSV="$sandbox/binding_conditions.tsv"
  export ORIENT_DOCTRINE_ANCHORS_TSV="$sandbox/doctrine_anchors.tsv"
  base="$(stamp_from_env)"

  printf '\n# selftest change\n' >>"$sandbox/orientation.md"
  digest_doc="$(stamp_from_env)"
  if [[ "$digest_doc" != "$base" ]]; then
    echo "FAIL orient_rule_stamp_prose_digest_excluded"
    unset ORIENT_PRECEDENTED_CLASSES_TSV ORIENT_BINDING_CONDITIONS_TSV ORIENT_DOCTRINE_ANCHORS_TSV
    rm -rf "$sandbox"
    return 1
  fi

  printf '\nselftest-class\tfixture\tfixture\tfixture\tactive\tfixture\n' >>"$sandbox/precedented_classes.tsv"
  changed="$(stamp_from_env)"
  [[ "$changed" != "$base" ]] || { echo "FAIL orient_rule_stamp_precedented_classes"; unset ORIENT_PRECEDENTED_CLASSES_TSV ORIENT_BINDING_CONDITIONS_TSV ORIENT_DOCTRINE_ANCHORS_TSV; rm -rf "$sandbox"; return 1; }
  cp "${SCRIPT_DIR}/precedented_classes.tsv" "$sandbox/precedented_classes.tsv"

  printf '\nSELFTEST\tcondition\tfixture\topen\tfixture\n' >>"$sandbox/binding_conditions.tsv"
  changed="$(stamp_from_env)"
  [[ "$changed" != "$base" ]] || { echo "FAIL orient_rule_stamp_binding_conditions"; unset ORIENT_PRECEDENTED_CLASSES_TSV ORIENT_BINDING_CONDITIONS_TSV ORIENT_DOCTRINE_ANCHORS_TSV; rm -rf "$sandbox"; return 1; }
  cp "${SCRIPT_DIR}/binding_conditions.tsv" "$sandbox/binding_conditions.tsv"

  printf '\nselftest-anchor\tdocs/orchestrator_orientation.md\theading:## Source Stamps\tfixture\t0000000000000000000000000000000000000000000000000000000000000000\n' >>"$sandbox/doctrine_anchors.tsv"
  changed="$(stamp_from_env)"
  [[ "$changed" != "$base" ]] || { echo "FAIL orient_rule_stamp_doctrine_anchors"; unset ORIENT_PRECEDENTED_CLASSES_TSV ORIENT_BINDING_CONDITIONS_TSV ORIENT_DOCTRINE_ANCHORS_TSV; rm -rf "$sandbox"; return 1; }

  unset ORIENT_PRECEDENTED_CLASSES_TSV ORIENT_BINDING_CONDITIONS_TSV ORIENT_DOCTRINE_ANCHORS_TSV
  rm -rf "$sandbox"
  echo "PASS orient_rule_stamp_sources"
  return 0
}

run_since_selftest() {
  local receipt current stale
  unset ORIENT_PRECEDENTED_CLASSES_TSV ORIENT_BINDING_CONDITIONS_TSV ORIENT_DOCTRINE_ANCHORS_TSV
  receipt="$(emit_orientation coding | awk '/^ORIENT-RECEIPT:/ {print $2; exit}')"
  current="$(emit_since coding "$receipt" | head -n 1)"
  stale="$(emit_since coding 000000000000 | head -n 1)"
  if [[ "$current" != "ORIENT-SINCE-VERDICT: CURRENT" ]]; then
    echo "FAIL orient_since_current"
    return 1
  fi
  if [[ "$stale" != "ORIENT-SINCE-VERDICT: STALE(rule-source)" ]]; then
    echo "FAIL orient_since_stale"
    return 1
  fi
  echo "PASS orient_since"
  return 0
}

run_orient_selftest_fixture() {
  local name="$1"
  local fix="${COLD_START_FIXTURES}/${name}"
  [[ -d "$fix" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected
  expected="$(tr -d '\r' <"${fix}/expected_result.txt" | head -n 1)"
  local role
  role="$(tr -d '\r' <"${fix}/role.txt" | head -n 1)"
  local got
  got="$(emit_orientation "$role" "$fix" | head -n 1)"
  if [[ "$got" == "$expected" ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  [[ -n "$ROLE" ]] || usage
  if [[ -n "$SINCE_RECEIPT" ]]; then
    emit_since "$ROLE" "$SINCE_RECEIPT"
    exit $?
  fi
  emit_orientation "$ROLE"
}

main "$@"
