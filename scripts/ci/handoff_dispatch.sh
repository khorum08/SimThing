#!/usr/bin/env bash
# HD-DISPATCH-SUBSTRATE-0 -- lint/render one .hd.md handoff object.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/handoff_dispatch.sh --lint <handoff-file>
  bash scripts/ci/handoff_dispatch.sh --render coding|orchestrator|da <handoff-file>
  bash scripts/ci/handoff_dispatch.sh --render-ingress <pr-number> <handoff-file>
  bash scripts/ci/handoff_dispatch.sh --receipt <handoff-file>
  bash scripts/ci/handoff_dispatch.sh --board-json [<handoff-file>]
  bash scripts/ci/handoff_dispatch.sh --render-board <board-json-file>
  bash scripts/ci/handoff_dispatch.sh --resolve-handoff <pr-body-file> <changed-files-file>
  bash scripts/ci/handoff_dispatch.sh --normalize-open-prs <prs-json-file>
  bash scripts/ci/handoff_dispatch.sh --board-issue-target <issues-json-file>
  bash scripts/ci/handoff_dispatch.sh --selftest
EOF
  exit 2
}

[[ $# -ge 1 ]] || usage

MODE="$1"; shift || true
case "$MODE" in
  --lint|--receipt|--board-json|--selftest) ;;
  --render-board|--normalize-open-prs|--board-issue-target) [[ $# -ge 1 ]] || usage ;;
  --render-ingress) [[ $# -ge 2 ]] || usage ;;
  --resolve-handoff) [[ $# -ge 2 ]] || usage ;;
  --render) [[ $# -ge 1 ]] || usage ;;
  -h|--help) usage ;;
  *) usage ;;
esac

HD_MODE="$MODE" \
HD_REPO_ROOT="$REPO_ROOT" \
HD_SCRIPT_PATH="${SCRIPT_DIR}/handoff_dispatch.sh" \
HD_ARGS="$*" \
  exec "$PYTHON_BIN" - "$@" <<'PY'
import csv
import datetime as dt
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
import tempfile

MODE = os.environ["HD_MODE"]
ROOT = Path(os.environ["HD_REPO_ROOT"])
SCRIPT = Path(os.environ["HD_SCRIPT_PATH"])
ARGS = sys.argv[1:]

REQUIRED_KEYS = [
    "rung",
    "kind",
    "track",
    "base_sha",
    "audience",
    "model_tier",
    "expected_route",
    "owner_approved",
    "owner_notes",
    "surfaces",
    "forbidden",
    "required_checks",
    "stop_conditions",
]
KIND_VALUES = {"rung", "transport", "remedial", "stamp"}
AUDIENCE_VALUES = {"coding", "orchestrator", "da"}
APPROVAL_VALUES = {"true", "false"}
LIST_KEYS = {"surfaces", "forbidden", "required_checks", "stop_conditions"}
SECTION_HEADINGS = ["## BUILD", "## FENCES", "## EXIT-PROOF"]


class HDError(Exception):
    def __init__(self, detail):
        super().__init__(detail)
        self.detail = detail


def fail(detail, code=1):
    print(f"HD-LINT-VERDICT: FAIL({detail})")
    return code


def normalize_bytes(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as exc:
        raise HDError("invalid-utf8") from exc
    text = text.replace("\r\n", "\n").replace("\r", "\n")
    if not text.endswith("\n"):
        raise HDError("terminal-newline")
    if text.endswith("\n\n"):
        raise HDError("terminal-newline")
    return text


def read_norm(path: Path) -> str:
    return normalize_bytes(path.read_bytes())


def read_json_file(path: Path):
    raw = path.read_bytes()
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return json.loads(raw.decode("utf-8"))


def repo_rel(path: Path) -> str:
    try:
        return path.resolve().relative_to(ROOT.resolve()).as_posix()
    except ValueError:
        return path.as_posix()


def parse_scalar(raw: str):
    value = raw.strip()
    if value.startswith("[") or value.startswith('"'):
        try:
            return json.loads(value)
        except json.JSONDecodeError as exc:
            raise HDError("frontmatter-value") from exc
    return value


def path_safe(rel: str) -> bool:
    rel = rel.replace("\\", "/")
    if not rel or rel.startswith("/") or ":" in rel:
        return False
    parts = PurePosixPath(rel).parts
    return bool(parts) and all(part not in ("", ".", "..") for part in parts)


def parse_handoff(path: Path):
    text = read_norm(path)
    lines = text.splitlines()
    if not lines or lines[0] != "---":
        raise HDError("frontmatter-missing")
    try:
        close = lines.index("---", 1)
    except ValueError as exc:
        raise HDError("frontmatter-unclosed") from exc
    if close == 1:
        raise HDError("frontmatter-empty")

    data = {}
    for raw in lines[1:close]:
        if ":" not in raw:
            raise HDError("frontmatter-line")
        key, value = raw.split(":", 1)
        key = key.strip()
        if key not in REQUIRED_KEYS:
            raise HDError("unknown-key")
        if key in data:
            raise HDError("duplicate-key")
        data[key] = parse_scalar(value)

    missing = [key for key in REQUIRED_KEYS if key not in data]
    if missing:
        raise HDError("missing-key")

    if set(data) != set(REQUIRED_KEYS):
        raise HDError("unknown-key")
    if data["kind"] not in KIND_VALUES:
        raise HDError("invalid-kind")
    if data["audience"] not in AUDIENCE_VALUES:
        raise HDError("invalid-audience")
    if str(data["owner_approved"]).lower() not in APPROVAL_VALUES:
        raise HDError("invalid-approval")
    data["owner_approved"] = str(data["owner_approved"]).lower()
    if not isinstance(data["owner_notes"], str):
        raise HDError("owner-notes")

    for key in LIST_KEYS:
        if not isinstance(data[key], list) or not all(isinstance(v, str) for v in data[key]):
            raise HDError("frontmatter-list")

    if path.name != f"{data['rung']}.hd.md":
        raise HDError("basename-rung-mismatch")
    for surface in data["surfaces"]:
        if not path_safe(surface):
            raise HDError("unsafe-surface")

    body_lines = lines[close + 1 :]
    headings = [line for line in body_lines if line.startswith("## ")]
    if headings != SECTION_HEADINGS:
        raise HDError("body-section-order")
    if len(body_lines) > 80:
        raise HDError("body-line-cap")

    sections = {}
    current = None
    for line in body_lines:
        if line in SECTION_HEADINGS:
            current = line.removeprefix("## ").lower()
            sections[current] = [line]
        elif line.startswith("## "):
            raise HDError("body-section-order")
        elif current:
            sections[current].append(line)
        else:
            if line.strip():
                raise HDError("body-section-order")

    return {"path": path, "text": text, "frontmatter": data, "body": body_lines, "sections": sections}


def hd_receipt(obj) -> str:
    return hashlib.sha256(obj["text"].encode("utf-8")).hexdigest()[:12]


def git_bash_cmd():
    if os.name != "nt":
        return "bash"
    for candidate in (
        Path(os.environ.get("ProgramFiles", r"C:\Program Files")) / "Git/bin/bash.exe",
        Path(os.environ.get("ProgramFiles", r"C:\Program Files")) / "Git/usr/bin/bash.exe",
    ):
        if candidate.exists():
            return str(candidate)
    return "bash"


def read_tsv(path: Path):
    if not path.exists():
        return [], []
    text = normalize_bytes(path.read_bytes())
    reader = csv.DictReader(io.StringIO(text), delimiter="\t")
    return reader.fieldnames or [], list(reader)


def anchor_ids_for_surfaces(surfaces):
    script = ROOT / "scripts/ci/anchor_query.sh"
    if not script.is_file():
        raise HDError("anchor-query-missing")
    with tempfile.TemporaryDirectory() as tmp:
        env = {**os.environ, "ANCHOR_REACH_LOG_PATH": str(Path(tmp) / "anchor_reach_log.tsv")}
        proc = subprocess.run(
            [git_bash_cmd(), repo_rel(script), "--paths", *surfaces],
            cwd=str(ROOT),
            capture_output=True,
            text=True,
            env=env,
        )
    if proc.returncode != 0:
        raise HDError("anchor-query-failed")
    domains = ""
    ids = ""
    for line in proc.stdout.splitlines():
        if line.startswith("domains:"):
            domains = line.split(":", 1)[1].strip()
        elif line.startswith("anchors:"):
            ids = line.split(":", 1)[1].strip()
    if domains and domains != "none" and (not ids or ids == "none"):
        raise HDError("anchor-unresolved")
    if not ids or ids == "none":
        return []
    return sorted({item.strip() for item in ids.split(",") if item.strip()})


def owner_directives(path=None):
    table = Path(os.environ.get("HD_OWNER_DIRECTIVES", "")) if os.environ.get("HD_OWNER_DIRECTIVES") else ROOT / "scripts/ci/owner_directives.tsv"
    fields, rows = read_tsv(table)
    expected = ["directive", "scope", "status", "set_by"]
    if fields != expected:
        raise HDError("owner-directives-schema")
    out = []
    active_pairs = set()
    for row in rows:
        status = (row.get("status") or "").strip()
        if status not in {"active", "retired"}:
            raise HDError("owner-directives-status")
        directive = (row.get("directive") or "").strip()
        scope = (row.get("scope") or "").strip()
        if not directive or not scope or not (row.get("set_by") or "").strip():
            raise HDError("owner-directives-row")
        pair = (directive, scope)
        if status == "active":
            if pair in active_pairs:
                raise HDError("owner-directives-duplicate")
            active_pairs.add(pair)
            out.append(row)
    return out


def shared_lines(obj, role):
    fm = obj["frontmatter"]
    anchors = ",".join(anchor_ids_for_surfaces(fm["surfaces"])) or "none"
    receipt = hd_receipt(obj)
    lines = [
        f"# HD Projection: {role}",
        f"rung: {fm['rung']}",
        f"track: {fm['track']}",
        f"kind: {fm['kind']}",
        f"audience: {fm['audience']}",
        f"model_tier: {fm['model_tier']}",
        f"base_sha: {fm['base_sha']}",
        f"expected_route: {fm['expected_route']}",
        f"owner_approved: {fm['owner_approved']}",
        f"HD-RECEIPT: {receipt}",
        f"REQUIRED-ANCHORS: {anchors}",
        "owner_notes:",
    ]
    if fm["owner_notes"]:
        lines.extend(fm["owner_notes"].split("\n"))
    else:
        lines.append("(none)")
    directives = owner_directives()
    lines.append("owner_directives:")
    if directives:
        for row in directives:
            lines.append(f"- {row['directive']} | scope={row['scope']} | set_by={row['set_by']}")
    else:
        lines.append("- none")
    lines.append("stop_conditions:")
    for item in fm["stop_conditions"]:
        lines.append(f"- {item}")
    return lines


def render_projection(role, obj):
    fm = obj["frontmatter"]
    if role not in {"coding", "orchestrator", "da"}:
        raise HDError("invalid-render-role")
    if role == "coding" and fm["owner_approved"] != "true":
        raise HDError("owner-approval-required")

    lines = shared_lines(obj, role)
    if role == "coding":
        lines.extend(["required_checks:"] + [f"- {x}" for x in fm["required_checks"]])
        lines.extend(["forbidden_surfaces:"] + [f"- {x}" for x in fm["forbidden"]])
        for heading in SECTION_HEADINGS:
            key = heading.removeprefix("## ").lower()
            lines.extend(obj["sections"][key])
    elif role == "orchestrator":
        lines.extend([
            "routing:",
            f"- expected_clearance: {fm['expected_route']}",
            "- merge_authority: DA after deep audit for gate-wiring",
            "- proof_intake: require matching HD-RECEIPT and clearance sticky",
            "- sticky_obligation: render handoff ingress and board state",
        ])
    else:
        lines.extend([
            "audit_targets:",
            "- strict frontmatter and receipt determinism",
            "- owner approval, notes, and directive delivery",
            "- relay receipt drift and bootstrap behavior",
            "- handoff sticky and SimThing Board issue",
            "risk_class: gate-wiring",
            "expected_residue: DA deep audit only",
        ])
        lines.extend(["forbidden_surfaces:"] + [f"- {x}" for x in fm["forbidden"]])

    out = "\n".join(lines).rstrip() + "\n"
    if role == "coding" and len(out.splitlines()) > 60:
        raise HDError("projection-line-cap")
    return out


def active_pointer():
    orientation = ROOT / "docs/orchestrator_orientation.md"
    if orientation.exists():
        text = normalize_bytes(orientation.read_bytes())
        m = re.search(r"Active pointer:\s*`([^`]+)`", text)
        if m:
            return m.group(1)
    return ""


def master_head():
    env = os.environ.get("HD_MASTER_HEAD", "").strip()
    if env:
        return env
    proc = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "origin/master"], capture_output=True, text=True)
    return proc.stdout.strip() if proc.returncode == 0 else ""


def expected_route_from_body(body: str) -> str:
    patterns = [
        r"(?im)^\s*expected_route\s*:\s*`?([^`\n|]+?)`?\s*$",
        r"(?im)^\s*CLEARANCE-VERDICT:\s*(ORCHESTRATOR-CLEARABLE|DA-RESERVE\([^)]+\)|FAIL\([^)]+\))\s*$",
        r"(?im)^\s*\|\s*Recommended posture\s*\|\s*`?([^`|\n]+?)`?\s*\|",
    ]
    for pattern in patterns:
        m = re.search(pattern, body or "")
        if not m:
            continue
        route = m.group(1).strip()
        if re.fullmatch(r"(ORCHESTRATOR-CLEARABLE|DA-RESERVE\([^)]+\)|FAIL\([^)]+\))", route):
            return route
        return f"MALFORMED_ROUTE:{route}"
    return "MISSING_ROUTE"


def normalize_open_prs(data):
    out = []
    for item in data:
        draft = item.get("draft")
        if draft is None:
            draft = item.get("isDraft", False)
        route = (item.get("route") or "").strip()
        if not route:
            route = expected_route_from_body(item.get("body", ""))
        out.append({
            "number": item.get("number"),
            "title": item.get("title", ""),
            "head": item.get("head", item.get("headRefName", "")),
            "url": item.get("url", ""),
            "route": route,
            "draft": bool(draft),
        })
    return sorted(out, key=lambda x: x.get("number") or 0)


def open_prs():
    raw = os.environ.get("HD_OPEN_PRS_JSON", "").strip()
    if not raw:
        return []
    try:
        data = json.loads(raw)
    except json.JSONDecodeError:
        return []
    return normalize_open_prs(data)


def active_bindings():
    fields, rows = read_tsv(ROOT / "scripts/ci/binding_conditions.tsv")
    out = []
    for row in rows:
        if (row.get("status") or "").strip() == "active":
            out.append({"rung": row.get("rung", ""), "condition": row.get("condition", "")})
    return sorted(out, key=lambda x: (x["rung"], x["condition"]))


def lease_summary():
    table = Path(os.environ.get("HD_CLOSEOUT_ARTIFACTS", "")) if os.environ.get("HD_CLOSEOUT_ARTIFACTS") else ROOT / "scripts/ci/closeout_artifacts.tsv"
    fields, rows = read_tsv(table)
    today = dt.date.fromisoformat(os.environ.get("HD_TODAY", dt.datetime.now(dt.timezone.utc).date().isoformat()))
    leases = []
    for row in rows:
        path = row.get("path", "")
        leased_at = row.get("leased_at", "")
        age = None
        try:
            age = (today - dt.date.fromisoformat(leased_at)).days
        except Exception:
            age = None
        leases.append({
            "path": path,
            "leased_at": leased_at,
            "age_days": age,
            "disposition": row.get("disposition", ""),
            "closeout_track": row.get("closeout_track", ""),
        })
    leases.sort(key=lambda x: x["path"])
    return {"count": len(leases), "leases": leases}


def ladder_states():
    design = ROOT / "docs/design_0_0_8_4_8_4_hd_board.md"
    if not design.exists():
        return []
    text = normalize_bytes(design.read_bytes())
    out = []
    for line in text.splitlines():
        if not line.startswith("| HD-"):
            continue
        cells = [c.strip() for c in line.strip("|").split("|")]
        if len(cells) >= 4 and cells[1] != "ID":
            out.append({"rung": cells[1].strip("`"), "exit_proof": cells[3]})
    return out


def board_json(path_arg=None):
    obj = parse_handoff(Path(path_arg)) if path_arg else None
    handoff = {}
    if obj:
        fm = obj["frontmatter"]
        handoff = {
            "path": repo_rel(obj["path"]),
            "rung": fm["rung"],
            "kind": fm["kind"],
            "owner_approved": fm["owner_approved"] == "true",
            "hd_receipt": hd_receipt(obj),
            "expected_route": fm["expected_route"],
        }
    return {
        "track": "0.0.8.4.8.4",
        "active_pointer": active_pointer(),
        "master_head": master_head(),
        "current_handoff": handoff,
        "open_prs": open_prs(),
        "binding_conditions": active_bindings(),
        "owner_directives": owner_directives(),
        "leases": lease_summary(),
        "ladder": ladder_states(),
    }


def command_lint(path):
    try:
        obj = parse_handoff(Path(path))
        anchor_ids_for_surfaces(obj["frontmatter"]["surfaces"])
        owner_directives()
    except HDError as exc:
        return fail(exc.detail)
    print("HD-LINT-VERDICT: PASS")
    return 0


def command_receipt(path):
    try:
        obj = parse_handoff(Path(path))
    except HDError as exc:
        return fail(exc.detail)
    print(f"HD-RECEIPT: {hd_receipt(obj)}")
    return 0


def command_render(role, path):
    try:
        obj = parse_handoff(Path(path))
        sys.stdout.write(render_projection(role, obj))
    except HDError as exc:
        return fail(exc.detail)
    return 0


def command_render_ingress(pr_number, path):
    try:
        obj = parse_handoff(Path(path))
        projection = render_projection("coding", obj)
    except HDError as exc:
        return fail(exc.detail)
    rel = repo_rel(obj["path"])
    lines = [
        "<!-- handoff-ingress-sticky -->",
        "## Handoff Ingress",
        "",
        f"- PR: #{pr_number}",
        f"- handoff: `{rel}`",
        "",
        "```",
        "HD-LINT-VERDICT: PASS",
        "```",
        "",
        "```",
    ]
    lines.extend(projection.rstrip().splitlines())
    lines.append("```")
    if len(lines) > 60:
        return fail("ingress-line-cap")
    sys.stdout.write("\n".join(lines).rstrip() + "\n")
    return 0


def command_board(path=None):
    try:
        data = board_json(path)
    except HDError as exc:
        print(json.dumps({"error": exc.detail}, sort_keys=True))
        return 1
    print(json.dumps(data, sort_keys=True, separators=(",", ":")))
    return 0


def clip(value, limit=160):
    text = str(value)
    return text if len(text) <= limit else text[: limit - 3] + "..."


def render_board_markdown(data):
    lines = ["<!-- simthing-board -->", "## SimThing Board", ""]
    lines.append(f"- track: `{data.get('track', '')}`")
    lines.append(f"- active_pointer: `{data.get('active_pointer', '')}`")
    lines.append(f"- master_head: `{str(data.get('master_head', ''))[:12]}`")
    handoff = data.get("current_handoff") or {}
    if handoff:
        lines.append(f"- current_handoff: `{handoff.get('rung', '')}` `{handoff.get('hd_receipt', '')}`")
        lines.append(f"- expected_route: `{handoff.get('expected_route', '')}`")
    else:
        lines.append("- current_handoff: none")
    lines.extend(["", "### Open PRs"])
    prs = data.get("open_prs") or []
    if prs:
        for pr in prs:
            lines.append(
                f"- #{pr.get('number')} `{pr.get('head', '')}` "
                f"draft={str(bool(pr.get('draft'))).lower()} "
                f"route=`{pr.get('route', 'MISSING_ROUTE')}` "
                f"{clip(pr.get('title', ''), 90)}"
            )
    else:
        lines.append("- none")
    lines.extend(["", "### Binding Conditions"])
    bindings = data.get("binding_conditions") or []
    if bindings:
        for item in bindings:
            lines.append(f"- `{item.get('rung', '')}` {clip(item.get('condition', ''), 100)}")
    else:
        lines.append("- none")
    lines.extend(["", "### Owner Directives"])
    directives = data.get("owner_directives") or []
    if directives:
        for item in directives:
            lines.append(f"- `{item.get('scope', '')}` {clip(item.get('directive', ''), 100)}")
    else:
        lines.append("- none")
    lines.extend(["", "### Leases"])
    leases = (data.get("leases") or {}).get("leases") or []
    if leases:
        for item in leases:
            lines.append(f"- `{item.get('path', '')}` age_days={item.get('age_days')}")
    else:
        lines.append("- none")
    lines.extend(["", "### Ladder"])
    for item in (data.get("ladder") or []):
        lines.append(f"- `{item.get('rung', '')}` {clip(item.get('exit_proof', ''), 150)}")
    if len(lines) > 60:
        raise HDError("board-line-cap")
    return "\n".join(lines).rstrip() + "\n"


def command_render_board(path):
    try:
        data = read_json_file(Path(path))
        sys.stdout.write(render_board_markdown(data))
    except HDError as exc:
        return fail(exc.detail)
    except Exception:
        return fail("board-json-read")
    return 0


def explicit_rung_from_pr_body(body: str):
    matches = re.findall(r"(?im)^\s*(?:[-*]\s*)?Rung\s*:\s*`?([A-Z0-9][A-Z0-9_-]+)`?\s*$", body)
    if not matches:
        raise HDError("missing-rung-identity")
    if len(matches) != 1:
        raise HDError("multiple-rung-identities")
    return matches[0]


def changed_handoff_files(path):
    lines = Path(path).read_text(encoding="utf-8").replace("\r\n", "\n").replace("\r", "\n").splitlines()
    out = []
    for line in lines:
        rel = line.strip().replace("\\", "/")
        if re.fullmatch(r"handoffs/[^/]+\.hd\.md", rel):
            out.append(rel)
    return sorted(set(out))


def command_resolve_handoff(body_file, changed_files):
    try:
        body = Path(body_file).read_text(encoding="utf-8")
        rung = explicit_rung_from_pr_body(body)
        rel = f"handoffs/{rung}.hd.md"
        changed = changed_handoff_files(changed_files)
        if not changed:
            raise HDError("missing-changed-handoff")
        if len(changed) > 1:
            raise HDError("multiple-changed-handoffs")
        if changed[0] != rel:
            raise HDError("rung-handoff-mismatch")
        path = ROOT / rel
        if not path.is_file():
            raise HDError("missing-handoff-file")
        obj = parse_handoff(path)
        if obj["frontmatter"]["rung"] != rung:
            raise HDError("frontmatter-rung-mismatch")
    except HDError as exc:
        return fail(exc.detail)
    print(rel)
    return 0


def command_normalize_open_prs(path):
    try:
        data = read_json_file(Path(path))
    except Exception:
        return fail("open-pr-json-read")
    print(json.dumps(normalize_open_prs(data), sort_keys=True, separators=(",", ":")))
    return 0


def command_board_issue_target(path):
    try:
        data = read_json_file(Path(path))
    except Exception:
        return fail("board-issues-json-read")
    if isinstance(data, list) and any(isinstance(item, list) for item in data):
        flattened = []
        for item in data:
            if isinstance(item, list):
                flattened.extend(item)
            else:
                flattened.append(item)
        data = flattened
    matches = [
        item for item in data
        if item.get("title") == "SimThing Board" and not item.get("pull_request")
    ]
    if len(matches) == 0:
        print("create")
        return 0
    if len(matches) == 1:
        print(f"update {matches[0].get('number')}")
        return 0
    return fail("duplicate-board-issues")


def write(path: Path, text: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8", newline="\n")


def run_cmd(args, env=None):
    return subprocess.run(args, cwd=str(ROOT), capture_output=True, text=True, env={**os.environ, **(env or {})})


def selftest_script_arg():
    return repo_rel(SCRIPT)


def selftest_bash_cmd():
    return git_bash_cmd()


def command_selftest():
    fixtures = ROOT / "scripts/ci/fixtures/handoff_dispatch"
    valid = fixtures / "VALID-HANDOFF-DISPATCH-FIXTURE-0.hd.md"
    draft = fixtures / "DRAFT-HANDOFF-DISPATCH-FIXTURE-0.hd.md"
    body_cap = fixtures / "BODY-CAP-HANDOFF-DISPATCH-FIXTURE-0.hd.md"
    missing = fixtures / "MISSING-KEY-HANDOFF-DISPATCH-FIXTURE-0.hd.md"
    unknown = fixtures / "UNKNOWN-KEY-HANDOFF-DISPATCH-FIXTURE-0.hd.md"
    owner = fixtures / "OWNER-NOTES-HANDOFF-DISPATCH-FIXTURE-0.hd.md"
    valid_arg = repo_rel(valid)
    draft_arg = repo_rel(draft)
    body_cap_arg = repo_rel(body_cap)
    missing_arg = repo_rel(missing)
    unknown_arg = repo_rel(unknown)
    owner_arg = repo_rel(owner)
    failures = []

    def check(name, ok, detail=""):
        if ok:
            print(f"PASS {name}")
        else:
            print(f"FAIL {name} {detail}".rstrip())
            failures.append(name)

    script_arg = selftest_script_arg()
    bash_cmd = selftest_bash_cmd()
    lint = run_cmd([bash_cmd, script_arg, "--lint", valid_arg])
    check("strict-frontmatter-valid", lint.stdout.splitlines()[:1] == ["HD-LINT-VERDICT: PASS"])
    check("missing-key-fails", "HD-LINT-VERDICT: FAIL(missing-key)" in run_cmd([bash_cmd, script_arg, "--lint", missing_arg]).stdout)
    check("unknown-key-fails", "HD-LINT-VERDICT: FAIL(unknown-key)" in run_cmd([bash_cmd, script_arg, "--lint", unknown_arg]).stdout)
    check("body-cap-81", "HD-LINT-VERDICT: FAIL(body-line-cap)" in run_cmd([bash_cmd, script_arg, "--lint", body_cap_arg]).stdout)

    draft_lint = run_cmd([bash_cmd, script_arg, "--lint", draft_arg])
    draft_coding = run_cmd([bash_cmd, script_arg, "--render", "coding", draft_arg])
    draft_orch = run_cmd([bash_cmd, script_arg, "--render", "orchestrator", draft_arg])
    draft_da = run_cmd([bash_cmd, script_arg, "--render", "da", draft_arg])
    check("draft-structural-lint", "HD-LINT-VERDICT: PASS" in draft_lint.stdout)
    check("draft-blocks-dispatch", draft_coding.returncode != 0 and "HD-LINT-VERDICT: FAIL(owner-approval-required)" in draft_coding.stdout and not draft_coding.stdout.startswith("# HD Projection"))
    check("draft-orchestrator-visible", draft_orch.returncode == 0 and "owner_approved: false" in draft_orch.stdout)
    check("draft-da-visible", draft_da.returncode == 0 and "owner_approved: false" in draft_da.stdout)

    receipts = []
    stable = True
    for role in ("coding", "orchestrator", "da"):
        one = run_cmd([bash_cmd, script_arg, "--render", role, valid_arg])
        two = run_cmd([bash_cmd, script_arg, "--render", role, valid_arg])
        stable = stable and one.stdout == two.stdout and one.returncode == 0 and two.returncode == 0
        m = re.search(r"HD-RECEIPT:\s*([0-9a-f]{12})", one.stdout)
        receipts.append(m.group(1) if m else "")
    rec = run_cmd([bash_cmd, script_arg, "--receipt", valid_arg])
    m = re.search(r"HD-RECEIPT:\s*([0-9a-f]{12})", rec.stdout)
    receipts.append(m.group(1) if m else "")
    check("projections-byte-stable", stable)
    check("projection-receipt-equality", len(set(receipts)) == 1 and receipts[0])

    owner_outputs = [run_cmd([bash_cmd, script_arg, "--render", role, owner_arg]).stdout for role in ("coding", "orchestrator", "da")]
    active = "Studio remains parked until Owner resumption"
    retired = "Retired fixture directive must stay hidden"
    note = "Owner note exact words: do not paraphrase."
    check("owner-notes-render", all(note in out for out in owner_outputs))
    check("active-directives-render", all(active in out for out in owner_outputs))
    check("retired-directives-hidden", all(retired not in out for out in owner_outputs))

    with tempfile.TemporaryDirectory() as tmp:
        ledger = Path(tmp) / "closeout_artifacts.tsv"
        write(ledger, "path\tleased_at\tdisposition\tcloseout_track\tnote\nhandoffs/OLD.hd.md\t2026-07-01\tlease\thd-fixture\tfixture lease\n")
        board = run_cmd(
            [bash_cmd, script_arg, "--board-json", valid_arg],
            env={
                "HD_CLOSEOUT_ARTIFACTS": str(ledger),
                "HD_TODAY": "2026-07-12",
                "HD_OPEN_PRS_JSON": '[{"number":1,"title":"fixture","headRefName":"h","url":"u","isDraft":true,"body":"expected_route: DA-RESERVE(fixture)"}]',
            },
        )
        data = json.loads(board.stdout)
        check("board-json-valid", data["current_handoff"]["rung"] == "VALID-HANDOFF-DISPATCH-FIXTURE-0")
        check("board-json-lease", data["leases"]["count"] == 1 and data["leases"]["leases"][0]["age_days"] == 11)
        check("board-json-open-pr-normalized", data["open_prs"][0]["head"] == "h" and data["open_prs"][0]["draft"] is True and data["open_prs"][0]["route"] == "DA-RESERVE(fixture)")

        board_json_path = Path(tmp) / "board.json"
        write(board_json_path, json.dumps(data))
        board_md = run_cmd([bash_cmd, script_arg, "--render-board", str(board_json_path)])
        check("board-render-open-pr-route", "#1 `h` draft=true route=`DA-RESERVE(fixture)`" in board_md.stdout)

        oversized = dict(data)
        oversized["open_prs"] = [
            {"number": n, "title": f"fixture {n}", "head": f"h{n}", "url": "", "route": "DA-RESERVE(fixture)", "draft": False}
            for n in range(1, 70)
        ]
        write(board_json_path, json.dumps(oversized))
        board_big = run_cmd([bash_cmd, script_arg, "--render-board", str(board_json_path)])
        check("board-line-cap-fails", "HD-LINT-VERDICT: FAIL(board-line-cap)" in board_big.stdout)

        body = Path(tmp) / "pr_body.md"
        changed = Path(tmp) / "changed_files.txt"
        live_handoff_arg = "handoffs/HD-DISPATCH-SUBSTRATE-0.hd.md"
        write(body, "## Status\n\nRung: HD-DISPATCH-SUBSTRATE-0\n")
        write(changed, f"{live_handoff_arg}\n")
        resolve = run_cmd([bash_cmd, script_arg, "--resolve-handoff", str(body), str(changed)])
        check("resolve-handoff-explicit-rung", resolve.returncode == 0 and resolve.stdout.strip() == live_handoff_arg)

        write(changed, "handoffs/OTHER-HANDOFF.hd.md\n")
        mismatch = run_cmd([bash_cmd, script_arg, "--resolve-handoff", str(body), str(changed)])
        check("resolve-handoff-mismatch-fails", "HD-LINT-VERDICT: FAIL(rung-handoff-mismatch)" in mismatch.stdout)

        issues = Path(tmp) / "issues.json"
        write(issues, '[{"number":7,"title":"SimThing Board"}]\n')
        target = run_cmd([bash_cmd, script_arg, "--board-issue-target", str(issues)])
        check("board-issue-single-update", target.returncode == 0 and target.stdout.strip() == "update 7")

        write(issues, '[{"number":7,"title":"SimThing Board"},{"number":8,"title":"SimThing Board"}]\n')
        dup = run_cmd([bash_cmd, script_arg, "--board-issue-target", str(issues)])
        check("board-issue-duplicate-fails", "HD-LINT-VERDICT: FAIL(duplicate-board-issues)" in dup.stdout)

        write(issues, '[[{"number":7,"title":"SimThing Board"}],[]]\n')
        paged = run_cmd([bash_cmd, script_arg, "--board-issue-target", str(issues)])
        check("board-issue-paginated-update", paged.returncode == 0 and paged.stdout.strip() == "update 7")

        ingress_handoff = Path(tmp) / "INGRESS-CAP-FIXTURE-0.hd.md"
        build_lines = "\n".join(f"- wrapper cap line {i}" for i in range(1, 23))
        write(ingress_handoff, f"""---
rung: INGRESS-CAP-FIXTURE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: fd022256b82c30c42da7d51e041128494bf3dd0a
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: ""
surfaces: ["scripts/ci/handoff_dispatch.sh"]
forbidden: ["crates/**"]
required_checks: ["handoff-dispatch-selftest"]
stop_conditions: ["scope-widening"]
---
## BUILD
{build_lines}
## FENCES
- Fixture only.
## EXIT-PROOF
- Wrapper line cap fails.
""")
        ingress_big = run_cmd([bash_cmd, script_arg, "--render-ingress", "1", str(ingress_handoff)])
        check("ingress-line-cap-fails", "HD-LINT-VERDICT: FAIL(ingress-line-cap)" in ingress_big.stdout)

    if failures:
        print(f"HANDOFF-DISPATCH-SELFTEST: FAIL ({len(failures)})")
        return 1
    print("HANDOFF-DISPATCH-SELFTEST: PASS")
    return 0


if MODE == "--selftest":
    sys.exit(command_selftest())
if MODE == "--lint":
    if len(ARGS) != 1:
        sys.exit(2)
    sys.exit(command_lint(ARGS[0]))
if MODE == "--receipt":
    if len(ARGS) != 1:
        sys.exit(2)
    sys.exit(command_receipt(ARGS[0]))
if MODE == "--render":
    if len(ARGS) != 2:
        sys.exit(2)
    sys.exit(command_render(ARGS[0], ARGS[1]))
if MODE == "--render-ingress":
    if len(ARGS) != 2:
        sys.exit(2)
    sys.exit(command_render_ingress(ARGS[0], ARGS[1]))
if MODE == "--board-json":
    if len(ARGS) > 1:
        sys.exit(2)
    sys.exit(command_board(ARGS[0] if ARGS else None))
if MODE == "--render-board":
    if len(ARGS) != 1:
        sys.exit(2)
    sys.exit(command_render_board(ARGS[0]))
if MODE == "--resolve-handoff":
    if len(ARGS) != 2:
        sys.exit(2)
    sys.exit(command_resolve_handoff(ARGS[0], ARGS[1]))
if MODE == "--normalize-open-prs":
    if len(ARGS) != 1:
        sys.exit(2)
    sys.exit(command_normalize_open_prs(ARGS[0]))
if MODE == "--board-issue-target":
    if len(ARGS) != 1:
        sys.exit(2)
    sys.exit(command_board_issue_target(ARGS[0]))
sys.exit(2)
PY
