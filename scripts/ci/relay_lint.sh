#!/usr/bin/env bash
# OH-RELAY-LINT-0 — validate relay/handoff structure; emit RELAY-LINT-VERDICT.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/relay_lint"
readonly COLD_START_FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/cold_start"
readonly ANCHOR_FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/anchor_integrity"

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
PR_HEAD_SHA=""
PR_BASE_SHA=""

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
  export RELAY_LINT_PR_HEAD_SHA="${PR_HEAD_SHA:-}"
  export RELAY_LINT_PR_BASE_SHA="${PR_BASE_SHA:-}"
  export RELAY_LINT_CHANGED_FILES="${RELAY_LINT_CHANGED_FILES:-}"
  result="$("$PYTHON_BIN" - <<'PY' "$text"
import hashlib
import os
import pathlib
import re
import subprocess
import sys

text = sys.argv[1]
repo_root = pathlib.Path(os.environ.get("RELAY_LINT_REPO_ROOT", "."))
fixture_dir = os.environ.get("RELAY_LINT_FIXTURE_DIR", "")
changed_files_env = os.environ.get("RELAY_LINT_CHANGED_FILES", "")
diff_env = os.environ.get("RELAY_LINT_DIFF", "")


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    body = raw.decode("utf-8")
    return body.replace("\r\n", "\n").replace("\r", "\n")


def read_normalized(path: pathlib.Path) -> str:
    return normalize_text(path.read_bytes())


def file_digest(path: pathlib.Path) -> str:
    return hashlib.sha256(read_normalized(path).encode("utf-8")).hexdigest()


def validate_live_pointer():
    patterns = [
        (r"(?im)^\s*current_pr_head\s*:", "current_pr_head"),
        (r"(?im)^\s*current\s+pr\s+head\s*:", "current_pr_head"),
        (r"(?i)live/docs-refresh\s+head", "live/docs-refresh head"),
        (r"(?im)^\s*docs-refresh\s+head\s*:", "docs-refresh head"),
        (r"(?im)^\s*docs-only\s+head\s*:", "docs-only head"),
        (r"(?im)^\s*evidence\s+docs\s+head\s*:", "evidence docs head"),
        (r"(?im)^\s*latest[- ]run\s*:", "latest-run"),
        (r"(?i)latest[- ]run\s+as\s+current", "latest-run"),
        (r"(?i)current\s+branch\s+tip", "current branch tip"),
        (r"(?im)^\s*branch\s+tip\s*:", "branch tip"),
        (r"(?i)as[-_]?of\s+sha", "as-of sha"),
        (r"(?i)self-referential\s+sha", "self-referential sha"),
    ]
    for pat, field in patterns:
        if re.search(pat, text):
            return field
    return None


live_pointer = validate_live_pointer()
if live_pointer:
    print(f"FAIL:live-pointer:{live_pointer}")
    sys.exit(0)


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

def lines_slice(path, spec):
    import re as _re
    m = _re.match(r"lines:(\d+)-(\d+)$", spec)
    start, end = int(m.group(1)), int(m.group(2))
    lines = read_normalized(path).splitlines()
    return "\n".join(lines[start - 1 : end]) + "\n"

def heading_section(path, heading):
    h = heading.removeprefix("heading:")
    lines = read_normalized(path).splitlines()
    start = None
    for i, line in enumerate(lines):
        if line.strip() == h or line.strip().startswith(h):
            start = i
            break
    if start is None:
        raise KeyError(heading)
    out = [lines[start]]
    for line in lines[start + 1 :]:
        if line.startswith("## ") and not line.startswith("###"):
            break
        out.append(line)
    return "\n".join(out).rstrip() + "\n"

def extract_anchor_text(doc_rel, section):
    path = repo_root / doc_rel
    if section.startswith("heading:"):
        return heading_section(path, section)
    if section.startswith("lines:"):
        return lines_slice(path, section)
    raise ValueError(section)

def load_anchor_state():
    import csv
    tsv = repo_root / "scripts" / "ci" / "doctrine_anchors.tsv"
    if fixture_dir:
        alt = pathlib.Path(fixture_dir) / "doctrine_anchors.tsv"
        if alt.is_file():
            tsv = alt
    if not tsv.is_file():
        return {}
    state = {}
    with tsv.open(encoding="utf-8", newline="") as fh:
        for row in csv.DictReader(fh, delimiter="\t"):
            if not row.get("anchor_id"):
                continue
            text = extract_anchor_text(row["doc"], row["section"])
            live = hashlib.sha256(text.encode("utf-8")).hexdigest()
            state[row["anchor_id"]] = {
                "live_hash": live,
                "short": live[:12],
                "domains": [d.strip() for d in row["trigger_domains"].split(",") if d.strip()],
            }
    return state

def anchor_stamp(state):
    joined = "|".join(f"{k}:{state[k]['live_hash']}" for k in sorted(state))
    return hashlib.sha256(joined.encode("utf-8")).hexdigest()[:16]

def orientation_state_from_fixture():
    if not fixture_dir:
        return None
    fix = pathlib.Path(fixture_dir)
    snap = fix / "orientation_snapshot.md"
    state = fix / "orientation_state.txt"
    if not snap.is_file() or not state.is_file():
        return None
    stamps = {}
    for line in state.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, val = line.split("=", 1)
            stamps[key.strip()] = val.strip()
    digest_sha = stamps.get("digest_sha") or hashlib.sha256(
        read_normalized(snap).encode("utf-8")
    ).hexdigest()
    return (
        digest_sha,
        stamps.get("orientation_rule_stamp", stamps.get("source_stamp", "")),
        snap,
    )


def orientation_rule_stamp():
    script_dir = repo_root / "scripts" / "ci"
    sources = [
        script_dir / "precedented_classes.tsv",
        script_dir / "binding_conditions.tsv",
        script_dir / "doctrine_anchors.tsv",
    ]
    return hashlib.sha256(
        "|".join(file_digest(p) for p in sources if p.is_file()).encode("utf-8")
    ).hexdigest()[:16]


def current_orientation_state():
    fixture_state = orientation_state_from_fixture()
    if fixture_state:
        return fixture_state
    orient_doc = repo_root / "docs" / "orchestrator_orientation.md"
    if not orient_doc.is_file():
        return None, None, None
    digest_sha = file_digest(orient_doc)
    return digest_sha, orientation_rule_stamp(), orient_doc

def expected_receipt(role, rule_stamp):
    return hashlib.sha256(
        f"ORIENT-RECEIPT|{role}|{rule_stamp}".encode("utf-8")
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
    rule_m = re.search(r'orientation_rule_stamp:\s*([0-9a-f]{16})', text, re.IGNORECASE)
    if not role_m or not rule_m:
        return "missing-orient-receipt"
    role = role_m.group(1).lower()
    rule_claim = rule_m.group(1).lower()
    live_digest, live_rule_stamp, _ = current_orientation_state()
    if live_digest is None:
        return "missing-orient-receipt"
    if role != required_role:
        return "wrong-orient-role"
    if rule_claim != live_rule_stamp:
        return "stale-orient-receipt"
    expected = expected_receipt(role, live_rule_stamp)
    if receipt_m.group(1).lower() != expected:
        return "stale-orient-receipt"
    return None

def required_trigger_domains():
    import csv
    import fnmatch
    from pathlib import PurePosixPath

    domains = set()
    if fixture_dir:
        req = pathlib.Path(fixture_dir) / "required_trigger_domains.txt"
        if req.is_file():
            for line in req.read_text(encoding="utf-8").splitlines():
                line = line.strip().lower()
                if line:
                    domains.add(line)
            return domains
        cf = pathlib.Path(fixture_dir) / "changed_files.txt"
        if not cf.is_file():
            # Legacy fixtures without path/domain overrides keep empty domain set.
            return domains
        files = [ln.strip().replace("\\", "/") for ln in cf.read_text(encoding="utf-8").splitlines() if ln.strip()]
    else:
        files = []
        if changed_files_env.strip():
            files = [ln.strip().replace("\\", "/") for ln in changed_files_env.splitlines() if ln.strip()]

    def glob_match(path: str, pattern: str) -> bool:
        g = pattern.strip().replace("\\", "/")
        if not g:
            return False
        p = PurePosixPath(path)
        if p.match(g):
            return True
        if "**" in g:
            prefix = g.split("**", 1)[0]
            if prefix and path.startswith(prefix):
                return True
        return fnmatch.fnmatch(path, g.replace("**", "*"))

    triggers = repo_root / "scripts" / "ci" / "anchor_triggers.tsv"
    if fixture_dir:
        alt = pathlib.Path(fixture_dir) / "anchor_triggers.tsv"
        if alt.is_file():
            triggers = alt
    if files and triggers.is_file():
        with triggers.open(encoding="utf-8", newline="") as fh:
            for row in csv.DictReader(fh, delimiter="\t"):
                glob_pat = (row.get("glob") or "").strip()
                if not glob_pat:
                    continue
                if any(glob_match(path, glob_pat) for path in files):
                    for d in (row.get("trigger_domains") or "").split(","):
                        d = d.strip()
                        if d:
                            domains.add(d)

    # Legacy prose regex remains secondary only.
    if re.search(r"gate-wiring", text, re.IGNORECASE):
        domains.update({"gate-wiring", "receipt-admission"})
    if re.search(r"movement-front|Movement-Front|map-domain|PALMA", text, re.IGNORECASE):
        domains.add("movement-front")
    return domains

def validate_anchor_ack():
    domains = required_trigger_domains()
    if not domains:
        return None
    state = load_anchor_state()
    if not state:
        return "missing-anchor-ack: remedy=bash scripts/ci/anchor_query.sh --paths <files...>"
    required_ids = set()
    for domain in domains:
        for aid, meta in state.items():
            if domain in meta["domains"]:
                required_ids.add(aid)
    if not required_ids:
        return None
    acks = {}
    for m in re.finditer(r"ANCHOR-ACK:\s*([a-z0-9-]+)@([0-9a-f]{12})", text, re.IGNORECASE):
        acks[m.group(1).lower()] = m.group(2).lower()
    for ack_id, short in acks.items():
        if ack_id not in state:
            return "unknown-anchor: remedy=bash scripts/ci/anchor_query.sh --domain <domain> or --grep <term>"
    for aid in sorted(required_ids):
        if aid not in acks:
            return "missing-anchor-ack: remedy=bash scripts/ci/anchor_query.sh --paths <files...>"
        if acks[aid] != state[aid]["short"]:
            return "stale-anchor-ack: remedy=bash scripts/ci/anchor_check.sh --resync then re-ack"
    return None

receipt_fail = validate_receipt()
if receipt_fail:
    print(f"FAIL:{receipt_fail}")
    sys.exit(0)

anchor_fail = validate_anchor_ack()
if anchor_fail:
    print(f"FAIL:{anchor_fail}")
    sys.exit(0)

def clearance_gate_required():
    patterns = [
        r"\bDA-review\b",
        r"\bDA review\b",
        r"\bDA-review-pending\b",
        r"\bDA-RESERVE\b",
        r"(?im)^\s*\|?\s*Recommended posture\s*\|?\s*deep\b",
        r"(?im)^\s*\|?\s*Risk class\s*\|?[^\n]*\bgate-wiring\b",
        r"(?im)^\s*Risk class\s*:[^\n]*\bgate-wiring\b",
    ]
    return any(re.search(p, text, re.IGNORECASE) for p in patterns)


def short_matches(longer, shorter):
    longer = (longer or "").lower()
    shorter = (shorter or "").lower()
    if len(longer) < len(shorter):
        longer, shorter = shorter, longer
    return len(shorter) >= 8 and longer.startswith(shorter)


def validate_clearance_verdict():
    if not clearance_gate_required():
        return None
    m = re.search(
        r"(?im)^\s*CLEARANCE-VERDICT:\s*(ORCHESTRATOR-CLEARABLE|DA-RESERVE\([^)]+\)|FAIL\([^)]+\))\s*$",
        text,
    )
    if not m:
        return "missing-clearance-verdict"
    verdict = m.group(1)
    if verdict.upper().startswith("ORCHESTRATOR-CLEARABLE"):
        return "clearable-not-da-relay"
    if verdict.upper().startswith("FAIL("):
        return "clearance-fail-remedy"
    if not verdict.upper().startswith("DA-RESERVE("):
        return "missing-clearance-verdict"

    tested = re.search(r"(?im)^\s*tested_code_sha\s*[:=]\s*([0-9a-f]{8,})\s*$", text)
    clearance_head = re.search(r"(?im)^\s*clearance_pr_head\s*[:=]\s*([0-9a-f]{8,})\s*$", text)
    tested_sha = tested.group(1).lower() if tested else ""
    clearance_sha = clearance_head.group(1).lower() if clearance_head else ""
    pr_head = os.environ.get("RELAY_LINT_PR_HEAD_SHA", "").lower()

    if pr_head:
        if short_matches(pr_head, clearance_sha) or short_matches(pr_head, tested_sha):
            return None
        return "missing-clearance-verdict"
    if clearance_sha and tested_sha and not short_matches(clearance_sha, tested_sha):
        return "missing-clearance-verdict"
    if clearance_sha or tested_sha:
        return None
    return "missing-clearance-verdict"


clearance_fail = validate_clearance_verdict()
if clearance_fail:
    print(f"FAIL:{clearance_fail}")
    sys.exit(0)

SHA_RE = re.compile(r"\b[0-9a-fA-F]{8,40}\b")
GRADUATION_RE = re.compile(
    r"\b(DA-GRADUATED|ORCHESTRATOR-GRADUATED|graduated|graduation|exit[- ]stamp|closed as graduated|merge commit|merged\s+#\d+\s+@)\b",
    re.IGNORECASE,
)
NON_CLAIM_SHA_FIELDS = re.compile(
    r"\b(tested_code_sha|clearance_pr_head|current_pr_head|base sha|base:|branch head|head sha|current head)\b",
    re.IGNORECASE,
)


def fixture_claimed_merge_state(commit: str):
    if not fixture_dir:
        return None
    path = pathlib.Path(fixture_dir) / "claimed_merge_commits.tsv"
    if not path.is_file():
        return None
    wanted = commit.lower()
    for raw in path.read_text(encoding="utf-8").splitlines():
        raw = raw.strip()
        if not raw or raw.startswith("#") or "\t" not in raw:
            continue
        sha, state = raw.split("\t", 1)
        if sha.lower() == wanted:
            return state.strip().lower()
    return None


def claimed_graduation_merge_commits():
    commits = []
    for line in text.splitlines():
        if NON_CLAIM_SHA_FIELDS.search(line):
            continue
        if not GRADUATION_RE.search(line):
            continue
        for m in SHA_RE.finditer(line):
            token = m.group(0).lower()
            if token not in commits:
                commits.append(token)
    return commits


def claimed_merge_is_on_master(commit: str):
    state = fixture_claimed_merge_state(commit)
    if state == "ancestor":
        return None
    if state == "not-ancestor":
        return "claimed-merge-not-on-master"
    if state == "unresolvable":
        return "unresolvable-claimed-merge"

    origin_master = subprocess.run(
        ["git", "-C", str(repo_root), "rev-parse", "--verify", "origin/master^{commit}"],
        capture_output=True,
        text=True,
    )
    if origin_master.returncode != 0:
        return "unresolvable-origin-master"
    resolved = subprocess.run(
        ["git", "-C", str(repo_root), "rev-parse", "--verify", f"{commit}^{{commit}}"],
        capture_output=True,
        text=True,
    )
    if resolved.returncode != 0:
        return "unresolvable-claimed-merge"
    ancestry = subprocess.run(
        ["git", "-C", str(repo_root), "merge-base", "--is-ancestor", resolved.stdout.strip(), "origin/master"],
        capture_output=True,
        text=True,
    )
    if ancestry.returncode != 0:
        return "claimed-merge-not-on-master"
    return None


for claimed_commit in claimed_graduation_merge_commits():
    claimed_merge_fail = claimed_merge_is_on_master(claimed_commit)
    if claimed_merge_fail:
        print(f"FAIL:{claimed_merge_fail}")
        sys.exit(0)


def explicit_rung_id():
    patterns = [
        r"(?im)^\s*(?:[-*]\s*)?Rung\s*:\s*`?([A-Z0-9][A-Z0-9_-]+)`?",
        r"(?im)^\s*Rung ID\s*/\s*title\s*:\s*`?([A-Z0-9][A-Z0-9_-]+)`?",
        r"(?im)^\s*\|\s*Rung\s*\|\s*`?([A-Z0-9][A-Z0-9_-]+)`?\s*\|",
    ]
    for pat in patterns:
        m = re.search(pat, text)
        if m:
            return m.group(1)
    return ""


def hd_claimed_receipt():
    m = re.search(r'(?im)^\s*HD-RECEIPT:\s*([0-9a-f]{12})\s*$', text)
    return m.group(1).lower() if m else ""


def changed_files_for_hd():
    if fixture_dir:
        cf = pathlib.Path(fixture_dir) / "changed_files.txt"
        if cf.is_file():
            return [ln.strip().replace("\\", "/") for ln in cf.read_text(encoding="utf-8").splitlines() if ln.strip()]
    if changed_files_env.strip():
        return [ln.strip().replace("\\", "/") for ln in changed_files_env.splitlines() if ln.strip()]
    base = os.environ.get("RELAY_LINT_PR_BASE_SHA", "").strip()
    if not base:
        mb = subprocess.run(
            ["git", "-C", str(repo_root), "merge-base", "HEAD", "origin/master"],
            capture_output=True,
            text=True,
        )
        base = mb.stdout.strip() if mb.returncode == 0 else ""
    if not base:
        return []
    files = []
    for args in (
        ["git", "-C", str(repo_root), "diff", "--name-only", f"{base}...HEAD"],
        ["git", "-C", str(repo_root), "diff", "--name-only"],
    ):
        proc = subprocess.run(args, capture_output=True, text=True)
        if proc.returncode == 0:
            for line in proc.stdout.splitlines():
                rel = line.strip().replace("\\", "/")
                if rel and rel not in files:
                    files.append(rel)
    return files


def hd_normalized_receipt(raw: bytes):
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    body = raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")
    if not body.endswith("\n"):
        body += "\n"
    return hashlib.sha256(body.encode("utf-8")).hexdigest()[:12], body


def fixture_file_bytes(root_name: str, rel: str):
    if not fixture_dir:
        return None
    candidate = pathlib.Path(fixture_dir) / root_name / rel
    if candidate.is_file():
        return candidate.read_bytes()
    return None


def hd_base_bytes(rel: str):
    fixture_bytes = fixture_file_bytes("base", rel)
    if fixture_bytes is not None:
        return fixture_bytes
    if fixture_dir:
        return None
    base = os.environ.get("RELAY_LINT_PR_BASE_SHA", "").strip()
    if not base:
        mb = subprocess.run(
            ["git", "-C", str(repo_root), "merge-base", "HEAD", "origin/master"],
            capture_output=True,
            text=True,
        )
        base = mb.stdout.strip() if mb.returncode == 0 else ""
    if not base:
        return None
    proc = subprocess.run(
        ["git", "-C", str(repo_root), "show", f"{base}:{rel}"],
        capture_output=True,
    )
    if proc.returncode != 0:
        return None
    return proc.stdout


def hd_head_bytes(rel: str):
    fixture_bytes = fixture_file_bytes("head", rel)
    if fixture_bytes is not None:
        return fixture_bytes
    candidate = repo_root / rel
    if candidate.is_file():
        return candidate.read_bytes()
    return None


def hd_frontmatter_rung(source: str):
    lines = source.splitlines()
    if not lines or lines[0] != "---":
        return ""
    for line in lines[1:]:
        if line == "---":
            return ""
        if line.startswith("rung:"):
            return line.split(":", 1)[1].strip().strip('"')
    return ""


def diff_marks_new_file(rel: str):
    patch = relay_diff_text()
    if not patch:
        return True
    marker = f"diff --git a/{rel} b/{rel}"
    start = patch.find(marker)
    if start < 0:
        return False
    nxt = patch.find("\ndiff --git ", start + 1)
    block = patch[start:] if nxt < 0 else patch[start:nxt]
    return "new file mode" in block or f"--- /dev/null" in block


def validate_hd_receipt():
    claimed = hd_claimed_receipt()
    if not claimed:
        return None
    rung_id = explicit_rung_id()
    if not rung_id:
        return "hd-receipt-missing-rung"
    rel = f"handoffs/{rung_id}.hd.md"
    base = hd_base_bytes(rel)
    if base is not None:
        base_receipt, _ = hd_normalized_receipt(base)
        if claimed != base_receipt:
            return "hd-receipt-drift"
        return None

    changed_handoffs = [
        path for path in changed_files_for_hd()
        if re.match(r"^handoffs/[^/]+\.hd\.md$", path)
    ]
    if changed_handoffs != [rel] or not diff_marks_new_file(rel):
        return "hd-bootstrap-shape"
    head = hd_head_bytes(rel)
    if head is None:
        return "hd-bootstrap-shape"
    head_receipt, head_text = hd_normalized_receipt(head)
    if pathlib.PurePosixPath(rel).name != f"{rung_id}.hd.md":
        return "hd-bootstrap-shape"
    if hd_frontmatter_rung(head_text) != rung_id:
        return "hd-bootstrap-shape"
    if claimed != head_receipt:
        return "hd-receipt-drift"
    return None


def claims_self_graduation(rung_id: str):
    if not rung_id:
        return False
    status_body = section_body([r'^##\s+Status\b', r'^Status:\s*$', r'^Status:\s*\S'])
    if re.search(r"\b(DA-GRADUATED|ORCHESTRATOR-GRADUATED|COMPLETE)\b", status_body):
        return True
    claim_patterns = [
        rf"(?is)\b{re.escape(rung_id)}\b[^\n]{{0,240}}\b(DA-GRADUATED|ORCHESTRATOR-GRADUATED|COMPLETE|graduated)\b",
        rf"(?is)\b(DA-GRADUATED|ORCHESTRATOR-GRADUATED|COMPLETE|graduated)\b[^\n]{{0,240}}\b{re.escape(rung_id)}\b",
        r"(?im)^\s*Status\s*:\s*(DA-GRADUATED|ORCHESTRATOR-GRADUATED|COMPLETE)\b",
    ]
    return any(re.search(p, text) for p in claim_patterns)


def relay_diff_text():
    if diff_env:
        return diff_env
    if fixture_dir:
        patch = pathlib.Path(fixture_dir) / "diff.patch"
        if patch.is_file():
            return patch.read_text(encoding="utf-8")
    return ""


def diff_contains_own_rung_stamp(rung_id: str):
    patch = relay_diff_text()
    if not patch:
        return False
    in_design = False
    for raw in patch.splitlines():
        if raw.startswith("diff --git "):
            in_design = "docs/design_" in raw and raw.endswith(".md")
            continue
        if not in_design or not raw.startswith("+") or raw.startswith("+++"):
            continue
        line = raw[1:]
        if rung_id not in line:
            continue
        if not re.search(r"\b(DA-GRADUATED|ORCHESTRATOR-GRADUATED|COMPLETE|merged\s+#\d+\s+@\s*[0-9a-fA-F]{8,40})\b", line):
            continue
        if re.search(r"\b(NOT STARTED|TBD|TODO|PLACEHOLDER|DA review pending|PROBATION)\b", line, re.IGNORECASE):
            continue
        return True
    return False


self_rung = explicit_rung_id()
hd_receipt_fail = validate_hd_receipt()
if hd_receipt_fail:
    print(f"FAIL:{hd_receipt_fail}")
    sys.exit(0)

if claims_self_graduation(self_rung) and not diff_contains_own_rung_stamp(self_rung):
    print("FAIL:self-rung-stamp-missing")
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
    PR_HEAD_SHA="$(gh pr view "$PR_NUMBER" --json headRefOid -q .headRefOid 2>/dev/null || true)"
    PR_BASE_SHA="$(gh pr view "$PR_NUMBER" --json baseRefOid -q .baseRefOid 2>/dev/null || true)"
    evidence=""
    local files
    files="$(gh pr view "$PR_NUMBER" --json files -q '.files[].path' 2>/dev/null || true)"
    export RELAY_LINT_CHANGED_FILES="$files"
    export RELAY_LINT_DIFF="$(gh pr diff "$PR_NUMBER" 2>/dev/null || true)"
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
  PR_HEAD_SHA=""
  PR_BASE_SHA=""
  [[ -d "$FIXTURE_DIR" ]] || { echo "missing fixture: $name" >&2; return 1; }
  if [[ -f "${FIXTURE_DIR}/current_pr_head.txt" ]]; then
    PR_HEAD_SHA="$(tr -d '\r\n' < "${FIXTURE_DIR}/current_pr_head.txt")"
  fi
  if [[ -f "${FIXTURE_DIR}/base_sha.txt" ]]; then
    PR_BASE_SHA="$(tr -d '\r\n' < "${FIXTURE_DIR}/base_sha.txt")"
  fi
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
    relay_lint_selftest_fail_live_pointer_current_pr_head
    relay_lint_selftest_fail_live_pointer_docs_refresh_head
    relay_lint_selftest_fail_live_pointer_latest_run
    relay_lint_selftest_fail_missing_clearance_verdict
    relay_lint_selftest_fail_stale_clearance_verdict
    relay_lint_selftest_pass_fresh_da_reserve_clearance
    relay_lint_selftest_fail_clearable_da_relay
    relay_lint_selftest_pass_non_da_without_clearance
    relay_lint_selftest_fail_claimed_merge_not_master
    relay_lint_selftest_fail_unresolvable_claimed_merge
    relay_lint_selftest_fail_self_graduation_stampless
    relay_lint_selftest_pass_ancestral_merge_self_stamp
    relay_lint_selftest_pass_transport_stamp_record
    relay_lint_selftest_pass_hd_bootstrap
    relay_lint_selftest_fail_hd_receipt_drift
    relay_lint_selftest_path_kernel_missing_ack
    relay_lint_selftest_path_docs_only_no_anchors
    relay_lint_selftest_path_sim_field_policy_missing_ack
    relay_lint_selftest_path_map_stead_missing_ack
    relay_lint_selftest_path_gpu_driver_convergence_missing_ack
  )
  local cold_fixtures=(
    cold_start_selftest_valid_coding_receipt
    cold_start_selftest_pass_prose_digest_churn
    cold_start_selftest_valid_orchestrator_receipt
    cold_start_selftest_fail_missing_receipt
    cold_start_selftest_fail_stale_receipt
    cold_start_selftest_fail_wrong_role
  )
  local anchor_fixtures=(
    anchor_integrity_selftest_pass_gate_wiring_ack
    anchor_integrity_selftest_fail_missing_ack
    anchor_integrity_selftest_fail_stale_ack
    anchor_integrity_selftest_fail_unknown_anchor
  )
  local name total
  total=$((${#relay_fixtures[@]} + ${#cold_fixtures[@]} + ${#anchor_fixtures[@]}))
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
  for name in "${anchor_fixtures[@]}"; do
    FIXTURE_DIR=""
    if ! run_fixture "$ANCHOR_FIXTURES_ROOT" "$name"; then
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
  if [[ -n "$PR_NUMBER" && -z "${PR_HEAD_SHA:-}" ]] && command -v gh >/dev/null 2>&1; then
    PR_HEAD_SHA="$(gh pr view "$PR_NUMBER" --json headRefOid -q .headRefOid 2>/dev/null || true)"
  fi
  if [[ -n "$PR_NUMBER" && -z "${PR_BASE_SHA:-}" ]] && command -v gh >/dev/null 2>&1; then
    PR_BASE_SHA="$(gh pr view "$PR_NUMBER" --json baseRefOid -q .baseRefOid 2>/dev/null || true)"
  fi
  lint_text "$text"
}

main "$@"
