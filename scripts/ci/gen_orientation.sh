#!/usr/bin/env bash
# OH-ORIENTATION-DIGEST-0 — generate orchestrator orientation digest from live harness data.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/orientation_digest"
readonly OUTPUT_PATH="${REPO_ROOT}/docs/orchestrator_orientation.md"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

MODE="generate"
FIXTURE_MODE=""
FIXTURE_DIR=""
SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/gen_orientation.sh
  bash scripts/ci/gen_orientation.sh --check
  bash scripts/ci/gen_orientation.sh --selftest
  bash scripts/ci/gen_orientation.sh --fixture <name>
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --check) MODE="check"; shift ;;
      --selftest) FIXTURE_MODE="selftest"; shift ;;
      --fixture)
        [[ $# -ge 2 ]] || usage
        FIXTURE_MODE="fixture"
        FIXTURE_DIR="${FIXTURES_ROOT}/${2}"
        shift 2
        ;;
      -h|--help) usage ;;
      *) usage ;;
    esac
  done
}

run_selftest_fixture() {
  local name="$1"
  local fix="${FIXTURES_ROOT}/${name}"
  [[ -d "$fix" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected
  expected="$(tr -d '\r' <"${fix}/expected_result.txt" | head -n 1)"
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-selftest-XXXXXX")"
  local classes="${SCRIPT_DIR}/precedented_classes.tsv"
  local binding="${SCRIPT_DIR}/binding_conditions.tsv"
  local ledger="${SCRIPT_DIR}/clearance_ledger.tsv"
  local design="${REPO_ROOT}/docs/design_0_0_8_4_7_orchestration_harness.md"
  local relay="${SCRIPT_DIR}/relay_lint.sh"
  [[ -f "${fix}/precedented_classes.tsv" ]] && classes="${fix}/precedented_classes.tsv"
  [[ -f "${fix}/binding_conditions.tsv" ]] && binding="${fix}/binding_conditions.tsv"
  [[ -f "${fix}/clearance_ledger.tsv" ]] && ledger="${fix}/clearance_ledger.tsv"
  cp "$classes" "$sandbox/precedented_classes.tsv"
  cp "$binding" "$sandbox/binding_conditions.tsv"
  cp "$ledger" "$sandbox/clearance_ledger.tsv"
  cp "$design" "$sandbox/design.md"
  cp "$relay" "$sandbox/relay_lint.sh"
  local out="${sandbox}/orientation.md"
  if [[ "$name" == "orientation_digest_selftest_stale_digest" ]]; then
    if [[ ! -f "${fix}/orientation.md" ]]; then
      echo "FAIL ${name}: missing stale orientation.md"
      return 1
    fi
    cp "${fix}/orientation.md" "$out"
  elif [[ "$name" == "orientation_digest_selftest_live_tsv_change" ]]; then
    ORIENTATION_CLASSES_TSV="${sandbox}/precedented_classes.tsv" \
    ORIENTATION_BINDING_TSV="${sandbox}/binding_conditions.tsv" \
    ORIENTATION_LEDGER_TSV="${sandbox}/clearance_ledger.tsv" \
    ORIENTATION_DESIGN_DOC="${sandbox}/design.md" \
    ORIENTATION_RELAY_LINT="${sandbox}/relay_lint.sh" \
    ORIENTATION_OUTPUT="$out" \
    bash "${SCRIPT_DIR}/gen_orientation.sh" >/dev/null
    printf 'stale-marker-row\n' >>"$sandbox/precedented_classes.tsv"
  else
    echo "FAIL ${name}: unknown fixture"
    return 1
  fi
  set +e
  ORIENTATION_CLASSES_TSV="${sandbox}/precedented_classes.tsv" \
  ORIENTATION_BINDING_TSV="${sandbox}/binding_conditions.tsv" \
  ORIENTATION_LEDGER_TSV="${sandbox}/clearance_ledger.tsv" \
  ORIENTATION_DESIGN_DOC="${sandbox}/design.md" \
  ORIENTATION_RELAY_LINT="${sandbox}/relay_lint.sh" \
  ORIENTATION_OUTPUT="$out" \
  bash "${SCRIPT_DIR}/gen_orientation.sh" --check >/dev/null 2>&1
  local rc=$?
  set -e
  local got="PASS"
  [[ "$rc" -ne 0 ]] && got="FAIL"
  if [[ "$got" == "$expected" ]]; then
    echo "PASS ${name}"
    rm -rf "$sandbox"
    return 0
  fi
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  rm -rf "$sandbox"
  return 1
}

run_selftest() {
  local fixtures=(
    orientation_digest_selftest_stale_digest
    orientation_digest_selftest_live_tsv_change
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_selftest_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "ORIENTATION-DIGEST-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  echo "ORIENTATION-DIGEST-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  if [[ "$FIXTURE_MODE" == "fixture" ]]; then
    [[ -d "$FIXTURE_DIR" ]] || { echo "missing fixture dir" >&2; exit 1; }
    run_selftest_fixture "$(basename "$FIXTURE_DIR")"
    exit $?
  fi

  export ORIENTATION_CLASSES_TSV="${ORIENTATION_CLASSES_TSV:-${SCRIPT_DIR}/precedented_classes.tsv}"
  export ORIENTATION_BINDING_TSV="${ORIENTATION_BINDING_TSV:-${SCRIPT_DIR}/binding_conditions.tsv}"
  export ORIENTATION_LEDGER_TSV="${ORIENTATION_LEDGER_TSV:-${SCRIPT_DIR}/clearance_ledger.tsv}"
  # Active-track pointer: which design doc the Next-Rung / rung-summary is drawn from.
  # A closed track must not keep pointing agents at its own leftover deferred rung —
  # opening a track updates active_track.txt (one line), not this script.
  _default_design="${REPO_ROOT}/docs/design_0_0_8_4_7_orchestration_harness.md"
  if [[ -f "${SCRIPT_DIR}/active_track.txt" ]]; then
    _at_rel="$(tr -d '\r' <"${SCRIPT_DIR}/active_track.txt" | grep -v '^[[:space:]]*#' | grep -v '^[[:space:]]*$' | head -n 1)"
    [[ -n "$_at_rel" && -f "${REPO_ROOT}/${_at_rel}" ]] && _default_design="${REPO_ROOT}/${_at_rel}"
  fi
  export ORIENTATION_DESIGN_DOC="${ORIENTATION_DESIGN_DOC:-${_default_design}}"
  export ORIENTATION_RELAY_LINT="${ORIENTATION_RELAY_LINT:-${SCRIPT_DIR}/relay_lint.sh}"
  export ORIENTATION_ANCHORS_TSV="${ORIENTATION_ANCHORS_TSV:-${SCRIPT_DIR}/doctrine_anchors.tsv}"
  export ORIENTATION_OUTPUT="${ORIENTATION_OUTPUT:-${OUTPUT_PATH}}"
  export ORIENTATION_MODE="$MODE"

  exec "$PYTHON_BIN" - <<'PY'
import hashlib
import csv
import os
import pathlib
import re
import sys
import tempfile

REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]
CLASSES_TSV = pathlib.Path(os.environ["ORIENTATION_CLASSES_TSV"])
BINDING_TSV = pathlib.Path(os.environ["ORIENTATION_BINDING_TSV"])
LEDGER_TSV = pathlib.Path(os.environ["ORIENTATION_LEDGER_TSV"])
DESIGN_DOC = pathlib.Path(os.environ["ORIENTATION_DESIGN_DOC"])
RELAY_LINT = pathlib.Path(os.environ["ORIENTATION_RELAY_LINT"])
OUTPUT = pathlib.Path(os.environ["ORIENTATION_OUTPUT"])
MODE = os.environ.get("ORIENTATION_MODE", "generate")

GENERATED_MARKER = "<!-- GENERATED by scripts/ci/gen_orientation.sh; do not edit by hand. -->"


def fail(msg):
    print(f"gen_orientation: {msg}", file=sys.stderr)
    sys.exit(1)


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    text = raw.decode("utf-8")
    return text.replace("\r\n", "\n").replace("\r", "\n")


def sha256_file(path: pathlib.Path) -> str:
    return hashlib.sha256(normalize_text(path.read_bytes()).encode("utf-8")).hexdigest()


def read_tsv(path: pathlib.Path):
    if not path.is_file():
        fail(f"missing source: {path}")
    rows = []
    with path.open(encoding="utf-8", newline="") as fh:
        for row in csv.reader(fh, delimiter="\t"):
            if not row or row[0] in ("class_id", "rung", "verdict"):
                continue
            rows.append(row)
    return rows


def md_row(values):
    return "| " + " | ".join(v.replace("|", "\\|") for v in values) + " |"


def table(headers, rows):
    lines = [md_row(headers), "| " + " | ".join("---" for _ in headers) + " |"]
    lines.extend(md_row(r) for r in rows)
    return lines


def parse_rungs(design_text: str):
    rows = []
    in_table = False
    for line in design_text.splitlines():
        if line.strip().startswith("| # | Rung |"):
            in_table = True
            continue
        if in_table:
            if not line.strip().startswith("|"):
                break
            if line.strip().startswith("|---"):
                continue
            parts = [p.strip() for p in line.strip().strip("|").split("|")]
            if len(parts) >= 4:
                rows.append((parts[0], parts[1], parts[2], parts[3]))
    return rows


def next_rung_pointer(rungs):
    for num, rung, _deliv, exit_proof in rungs:
        low = exit_proof.lower()
        if "da-graduated" in low or "orchestrator-graduated" in low:
            continue
        if "deferred" in low:
            continue
        parts = rung.split("`")
        return parts[1] if len(parts) >= 3 else rung.strip("`").strip()
    return "none"


def ledger_summary(rows, limit=5):
    if not rows:
        return ["> clearance ledger empty"]
    tail = rows[-limit:]
    out = table(["verdict", "class", "pr", "sha", "date"], [r[:5] for r in tail if len(r) >= 5])
    return out


classes = read_tsv(CLASSES_TSV)
binding = read_tsv(BINDING_TSV)
ledger_rows = read_tsv(LEDGER_TSV)
design_text = DESIGN_DOC.read_text(encoding="utf-8")
rungs = parse_rungs(design_text)
next_rung = next_rung_pointer(rungs)

ANCHORS_TSV = pathlib.Path(os.environ["ORIENTATION_ANCHORS_TSV"])

sources = [
    ("precedented_classes.tsv", CLASSES_TSV),
    ("binding_conditions.tsv", BINDING_TSV),
    ("clearance_ledger.tsv", LEDGER_TSV),
    (DESIGN_DOC.name, DESIGN_DOC),
    ("relay_lint.sh", RELAY_LINT),
    ("doctrine_anchors.tsv", ANCHORS_TSV),
]
manifest = [(name, sha256_file(path)) for name, path in sources]

class_rows = []
for row in classes:
    if len(row) < 6:
        continue
    class_rows.append((row[0], row[1], row[2], row[3], row[4], row[5]))

binding_rows = []
for row in binding:
    if len(row) < 5:
        continue
    binding_rows.append(tuple(row[:5]))

lines = [
    "# Orchestrator Orientation",
    "",
    GENERATED_MARKER,
    "",
    "> Operational orientation generated from live harness TSVs. Not a doctrine anchor summary.",
    "> Regenerate: `bash scripts/ci/gen_orientation.sh`",
    "",
    "## MANDATORY (ORCHESTRATOR burden): run `/clearance` before you relay for DA review",
    "",
    "**This is the orchestrator's job, not the DA's.** Do NOT produce, post, or relay a DA-review /",
    "graduation handoff without first running the clearance router and observing its verdict for the current",
    "PR. It is not optional, and it is NOT satisfied by a verdict quoted in someone else's report.",
    "",
    "1. Run `/clearance` (GHA) or `bash scripts/ci/clearance_check.sh --pr <n>` / `--range <base>..<head>`,",
    "   and read the emitted `CLEARANCE-VERDICT` yourself.",
    "2. `ORCHESTRATOR-CLEARABLE` -> **merge it yourself. Do NOT escalate to DA.**",
    "3. `DA-RESERVE(...)` -> that verdict is the ONLY valid justification for a DA relay; quote it verbatim.",
    "4. No verdict yet -> **STOP. Do not write the relay.** Trigger clearance first.",
    "",
    "`relay_lint` FAILs a DA relay lacking a fresh PR-head-bound verdict (`FAIL(missing-clearance-verdict)`).",
    "A handoff typed into chat is outside CI and cannot be linted -- it is on your honor to the same rule.",
    "Never SHA-match (`tested_code_sha` vs head) in place of running the router: that is the recurring kabuki",
    "that appears whenever the real mechanism is skipped. The router is the routing authority, run first-hand.",
    "",
    "**DA side:** the DA does NOT re-run `/clearance` as a required pass -- a green `relay_lint` is",
    "DA-equivalent for routing (the orchestrator already paid this cost). The DA runs the router only on",
    "spot-audit or when a relay is genuinely suspect. See design 0.0.8.4.8 section 4C.",
    "",
    "## Source Stamps",
    "",
]
lines.extend(table(["source", "sha256"], manifest))
lines.extend([
    "",
    f"## Active Track / Rung Summary (`{DESIGN_DOC.name}`)",
    "",
])
rung_table = []
for num, rung, deliverable, exit_proof in rungs:
    short = exit_proof
    if len(short) > 120:
        short = short[:117] + "..."
    rung_table.append((num, rung.strip("`"), deliverable[:80], short))
lines.extend(table(["#", "rung", "deliverable", "exit proof"], rung_table))
lines.extend([
    "",
    "## Next Rung Pointer",
    "",
    f"Active pointer: `{next_rung}`",
    "",
    "## Cold-Start Entrypoint",
    "",
    "Cold-start entrypoint: run `bash scripts/ci/orient.sh --role=coding|orchestrator|da` and carry the emitted ORIENT-RECEIPT.",
    "",
    "## Clearance Router Verdict Meanings",
    "",
    "| verdict | meaning |",
    "| --- | --- |",
    "| `CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE` | precedented class matched; binding conditions discharged; required proof fields present |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(novelty)` | resolved non-empty diff with no precedented class match |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(harness-error)` | malformed data, ambiguous class, empty/unresolved requested target, or script error |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)` | PR touches router/lint/harness gate surfaces (self-application refusal) |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(binding-conditions)` | open binding condition blocks clearance for matched class |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(class-suspended)` | precedented class row status=suspended |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(triage-missing)` | INSPECT delta without landed /triage row (check 7 live) |",
    "| `CLEARANCE-VERDICT: FAIL(remedy)` | named fix required before re-attempt (CI not green, missing proof fields, etc.) |",
    "",
    "## Precedented Classes (active)",
    "",
])
active_classes = [r for r in class_rows if len(r) > 4 and r[4] != "retired"]
lines.extend(table(["class_id", "envelope", "requirements", "status", "promotion_blocker"], [r[:5] for r in active_classes]))
lines.extend([
    "",
    "## Binding Conditions",
    "",
])
lines.extend(table(["rung", "condition", "set_by", "status", "promotion_blocker"], binding_rows))
lines.extend([
    "",
    "## Clearance Ledger (recent)",
    "",
])
lines.extend(ledger_summary(ledger_rows))
lines.extend([
    "",
    "## Relay Lint Required Blocks",
    "",
    "Required relay/handoff sections (M3): Status; PR/branch/merge; What changed; Load-bearing proofs; Scope Ledger; Conformance; Known gaps; Graduation routing.",
    "",
    "Graduation routing must name: CI verdict, triage entries, risk class, falsification check, recommended posture.",
    "",
    "Proof identity fields required in relay body:",
    "- `tested_code_sha: <8+ hex>`",
    "- `coverage_basis: PASS` (or explicit coverage basis)",
    "",
    f"relay_lint.sh schema stamp: `{sha256_file(RELAY_LINT)[:12]}`",
    "",
    "## tested_code_sha + coverage_basis Rule",
    "",
    "Clearance classes requiring workshop/production proof must carry citable `tested_code_sha` and `coverage_basis` in the PR/relay body.",
    "GPU/desktop/bevy proof is owner-local execution with recorded `DOCTRINE-TESTS-VERDICT: PASS` bound to the same SHA — GHA validates binding, never executes GPU legs.",
    "",
    "## Escalation / DA-RESERVE Posture",
    "",
    "- Novelty, binding-conditions, class-suspended, triage-missing → DA review routing.",
    "- gate-wiring → deep audit; harness surfaces are never self-mergeable.",
    "- harness-error → fix data/target resolution before re-run.",
    "- FAIL(remedy) → apply named remedy and re-run clearance.",
    "",
    "## Orientation Receipt (ORIENT-RECEIPT)",
    "",
    "Run `bash scripts/ci/orient.sh --role=coding|orchestrator|da` to emit a rule-source-bound receipt.",
    "",
    "Schema:",
    "- `ORIENT-RECEIPT: <12-char hash>` - stable hash over role + orientation_rule_stamp",
    "- `role: coding|orchestrator|da`",
    "- `orientation_rule_stamp: <16-char hash>` - hash over `precedented_classes.tsv`, `binding_conditions.tsv`, and `doctrine_anchors.tsv`",
    "- `orientation_digest_sha: <sha256 of docs/orchestrator_orientation.md>` (informational only; prose digest churn does not stale receipts)",
    "- `generated_at: source-bound` (non-authoritative; validation uses the rule stamp)",
    "",
    "Role meanings:",
    "- `coding` — clearance contract, inner-loop commands, precedented classes",
    "- `orchestrator` — full orientation digest",
    "- `da` — rung table, binding conditions, escalation posture",
    "",
    "Receipt freshness: relay-lint compares claimed `orientation_rule_stamp` to the live rule stamp; mismatch -> `FAIL(stale-orient-receipt)`.",
    "Relay-lint receipt rule: gate-wiring handoffs require a valid receipt for the declared role.",
    "Rule-source edits, including `doctrine_anchors.tsv`, stale `ORIENT-RECEIPT` values.",
    "",
    "## Doctrine Anchors (ANCHOR-ACK)",
    "",
    "Table: `scripts/ci/doctrine_anchors.tsv` (`anchor_id | doc | section | trigger_domains | content_hash`).",
    "",
    "ANCHOR-ACK schema: `ANCHOR-ACK: <anchor_id>@<12-char content_hash>`",
    "",
    "Trigger-domain rule: relays touching a domain must ack anchors listing that domain (e.g. `movement-front`, `gate-wiring`).",
    "",
    "Relay-lint failures: `missing-anchor-ack`, `stale-anchor-ack`, `unknown-anchor`.",
    "",
    "Run `bash scripts/ci/anchor_check.sh --check` after anchor table edits.",
    "",
    "## Inner Loop (coding agent)",
    "",
    "```bash",
    "bash scripts/ci/orient.sh --role=coding",
    "bash scripts/ci/anchor_check.sh --check",
    "bash scripts/ci/clearance_check.sh --selftest",
    "bash scripts/ci/relay_lint.sh --selftest",
    "bash scripts/ci/gen_orientation.sh --check",
    "bash scripts/ci/doctrine_selftest.sh",
    "bash scripts/ci/doctrine_scan.sh",
    "```",
    "",
    "## GHA Comment Commands",
    "",
    "- `/clearance` — M1 router verdict",
    "- `/relay-lint` — M3 relay lint verdict",
    "- `/orient` — M2 orientation digest (this page)",
    "- `/orient role=orchestrator|coding|da` — role-filtered subset",
    "- `/anchor <anchor_id|trigger_domain>` — verbatim anchored doctrine text",
    "",
])
generated = "\n".join(lines) + "\n"

if MODE == "check":
    if not OUTPUT.is_file():
        fail(f"{OUTPUT} is missing; remedy: bash scripts/ci/gen_orientation.sh")
    current = OUTPUT.read_text(encoding="utf-8")
    if GENERATED_MARKER not in current:
        fail("orientation digest missing generated marker; do not hand-edit")
    if current != generated:
        with tempfile.NamedTemporaryFile("w", encoding="utf-8", delete=False, suffix=".md") as tmp:
            tmp.write(generated)
            tmp_path = tmp.name
        fail(
            f"{OUTPUT} is stale; expected output written to {tmp_path}; "
            "remedy: run `bash scripts/ci/gen_orientation.sh` and commit docs/orchestrator_orientation.md"
        )
    print("gen_orientation --check: PASS")
else:
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(generated, encoding="utf-8", newline="\n")
    rel = OUTPUT
    try:
        rel = OUTPUT.relative_to(REPO_ROOT)
    except ValueError:
        pass
    print(f"generated {rel}")
PY
}

main "$@"
