#!/usr/bin/env bash
# SimThing Board sync: the ONE writer of the Board issue body, shared by the clearance
# workflow (every run) and the /handoff command. It is a COSMETIC mirror and never fails
# its caller. Two rules keep it honest:
#   - every degraded input is declared IN the body (board sync notes), never only in a log;
#   - every freshly rendered body carries a `synced:` stamp, so a board that stops
#     updating shows its own age instead of silently freezing (it froze 2026-09-09..26).
#
# usage: bash scripts/ci/board_sync.sh [<handoff>]
#        bash scripts/ci/board_sync.sh --selftest
# env:   GH_TOKEN, GITHUB_REPOSITORY  required for a live sync
#        HD_OPEN_PRS_JSON             normalized open PRs (fetched when unset)
#        BOARD_ISSUES_JSON            open-issues json file (skips the gh read; tests)
#        BOARD_SYNC_DRY_RUN=1         print target + body instead of writing
set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HD="${SCRIPT_DIR}/handoff_dispatch.sh"
PYTHON_BIN="${PYTHON_BIN:-python3}"
command -v "$PYTHON_BIN" >/dev/null 2>&1 || PYTHON_BIN="python"

gh_retry() {
  local attempt rc
  for attempt in 1 2 3; do
    "$@" && return 0
    rc=$?
    [ "$attempt" -lt 3 ] && sleep $((attempt * 2))
  done
  return "$rc"
}

sync_board() {
  local handoff="${1:-}" work notes="" target target_rc render_ok=1 fresh=1
  work="$(mktemp -d "${TMPDIR:-/tmp}/board-sync-XXXXXX")"
  note() { notes="${notes:+${notes}; }$1"; }

  # A handoff must be a bare rung id or an existing file. Anything else is a verdict or
  # diagnostic that leaked into the caller's variable; ignore it, never use it as a path.
  if [ -n "$handoff" ] && ! { [ -f "$handoff" ] || printf '%s' "$handoff" | grep -qxE '[A-Z0-9][A-Z0-9_-]*'; }; then
    note "handoff argument ignored [$(printf '%s' "$handoff" | head -c 80 | tr '\n' ' ')]"
    handoff=""
  fi

  if [ -z "${HD_OPEN_PRS_JSON:-}" ] && [ "${BOARD_SYNC_DRY_RUN:-0}" != "1" ]; then
    if gh_retry gh pr list --state open --json number,title,headRefName,url,isDraft,body > "$work/open-prs.raw.json" 2>/dev/null \
       && bash "$HD" --normalize-open-prs "$work/open-prs.raw.json" > "$work/open-prs.json" 2>/dev/null; then
      export HD_OPEN_PRS_JSON="$(cat "$work/open-prs.json")"
    else
      note "open PRs unreadable"
    fi
  fi

  if ! bash "$HD" --board-json ${handoff:+"$handoff"} > "$work/board.json" 2> "$work/board-json.err"; then
    echo '{"error":"board-json"}' > "$work/board.json"
    note "board-json build [$(tail -n1 "$work/board-json.err" 2>/dev/null | tr -d '\r' | cut -c1-160)]"
  fi
  if ! bash "$HD" --render-board "$work/board.json" > "$work/board.md" 2>/dev/null; then
    render_ok=0
    note "board render failed"
  fi

  if [ -n "${BOARD_ISSUES_JSON:-}" ]; then
    cp "$BOARD_ISSUES_JSON" "$work/issues.json"
  elif ! gh_retry gh api "repos/${GITHUB_REPOSITORY}/issues?state=open&per_page=100" --paginate --slurp > "$work/issues.json" 2>/dev/null; then
    echo '[]' > "$work/issues.json"
  fi
  target="$(bash "$HD" --board-issue-target "$work/issues.json" 2> "$work/target.note")"
  target_rc=$?
  if [ -s "$work/target.note" ]; then
    note "$(tr -d '\r' < "$work/target.note" | head -n 2 | tr '\n' ' ' | sed 's/ *$//')"
  fi
  # Only these two shapes are targets. A refusal verdict on stdout is NOT an issue number:
  # treating it as one is what froze the board for 17 days.
  if [ "$target_rc" -ne 0 ] || ! printf '%s' "$target" | grep -qxE 'create|update [0-9]+'; then
    note "board target unresolved [$(printf '%s' "$target" | head -c 120 | tr '\n' ' ')]"
    target=""
  fi

  if [ "$render_ok" -eq 1 ]; then
    cp "$work/board.md" "$work/body.md"
  elif [ -n "$target" ] && [ "$target" != "create" ] && [ -z "${BOARD_ISSUES_JSON:-}" ] \
       && gh_retry gh api "repos/${GITHUB_REPOSITORY}/issues/${target#update }" --jq .body > "$work/body.md" 2>/dev/null \
       && [ -s "$work/body.md" ]; then
    fresh=0
  else
    printf '<!-- simthing-board -->\n## SimThing Board\n\n_(render unavailable this run)_\n' > "$work/body.md"
  fi

  "$PYTHON_BIN" - "$work/body.md" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "${GITHUB_RUN_ID:-local}" "$fresh" "$notes" <<'PY'
import pathlib
import sys

path, now, run, fresh, notes = sys.argv[1:6]
body = pathlib.Path(path)
lines = body.read_text(encoding="utf-8").splitlines()
lines = [line for line in lines if not line.startswith("> :warning: board sync notes")]
if fresh == "1":
    # Only a fresh render may claim freshness; a carried-forward body keeps its old stamp.
    lines = [line for line in lines if not line.startswith("- synced:")]
    header = [f"- synced: {now} (run {run})"]
else:
    notes = (notes + "; " if notes else "") + f"render failed at {now}; content below is carried forward"
    header = []
if notes:
    header.append(f"> :warning: board sync notes: {notes}")
try:
    at = next(i for i, line in enumerate(lines) if line.strip() == "## SimThing Board") + 1
except StopIteration:
    lines = ["<!-- simthing-board -->", "## SimThing Board"] + lines
    at = 2
if header:
    lines[at:at] = [""] + header
body.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")
PY

  if [ "${BOARD_SYNC_DRY_RUN:-0}" = "1" ]; then
    printf 'BOARD-SYNC-TARGET: %s\n' "${target:-unresolved}"
    cat "$work/body.md"
    rm -rf "$work"
    return 0
  fi

  "$PYTHON_BIN" -c 'import json,sys; print(json.dumps({"title": "SimThing Board", "body": open(sys.argv[1], encoding="utf-8").read()}))' \
    "$work/body.md" > "$work/payload.json"
  if [ -z "$target" ]; then
    echo "::warning::SimThing Board not written: ${notes}"
  elif [ "$target" = "create" ]; then
    gh_retry gh api -X POST "repos/${GITHUB_REPOSITORY}/issues" --input "$work/payload.json" >/dev/null 2>&1 \
      || echo "::warning::SimThing Board create failed after retries"
  else
    gh_retry gh api -X PATCH "repos/${GITHUB_REPOSITORY}/issues/${target#update }" --input "$work/payload.json" >/dev/null 2>&1 \
      || echo "::warning::SimThing Board update failed after retries; its synced stamp now shows the board's age"
  fi
  rm -rf "$work"
  return 0
}

selftest() {
  local tmp out fails=0
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/board-sync-selftest-XXXXXX")"
  check() {
    if [ "$2" = "ok" ]; then echo "PASS $1"; else echo "FAIL $1"; fails=$((fails + 1)); fi
  }
  has() { printf '%s' "$out" | grep -qF -- "$1" && echo ok || echo no; }
  lacks() { printf '%s' "$out" | grep -qF -- "$1" && echo no || echo ok; }
  run() { out="$(BOARD_SYNC_DRY_RUN=1 BOARD_ISSUES_JSON="$tmp/issues.json" HD_OPEN_PRS_JSON='[]' sync_board "$@" 2>&1)"; }

  printf '[{"number":1332,"title":"SimThing Board"}]\n' > "$tmp/issues.json"
  run
  check "single-board-updates-with-fresh-stamp" "$( [ "$(has 'BOARD-SYNC-TARGET: update 1332')$(has '- synced: ')$(lacks 'board sync notes')" = "okokok" ] && echo ok || echo no)"

  printf '[{"number":2024,"title":"SimThing Board"},{"number":1332,"title":"SimThing Board"},{"number":2023,"title":"SimThing Board"}]\n' > "$tmp/issues.json"
  run
  check "duplicates-write-canonical-and-name-strays" "$( [ "$(has 'BOARD-SYNC-TARGET: update 1332')$(has 'duplicate-board-issues #2023,#2024')" = "okok" ] && echo ok || echo no)"

  printf '[]\n' > "$tmp/issues.json"
  run
  check "empty-read-never-targets-or-creates" "$( [ "$(has 'BOARD-SYNC-TARGET: unresolved')$(has 'board target unresolved')$(lacks 'BOARD-SYNC-TARGET: create')" = "okokok" ] && echo ok || echo no)"

  printf '[{"number":1332,"title":"SimThing Board"}]\n' > "$tmp/issues.json"
  run 'HD-LINT-VERDICT: FAIL(current-handoff-missing)'
  check "leaked-verdict-handoff-is-ignored" "$( [ "$(has 'handoff argument ignored')$(has 'BOARD-SYNC-TARGET: update 1332')" = "okok" ] && echo ok || echo no)"

  rm -rf "$tmp"
  if [ "$fails" -eq 0 ]; then
    echo "BOARD-SYNC-SELFTEST: PASS"
    return 0
  fi
  echo "BOARD-SYNC-SELFTEST: FAIL ($fails)"
  return 1
}

case "${1:-}" in
  --selftest) selftest ;;
  -h|--help) sed -n '2,14p' "$0" ;;
  *) sync_board "${1:-}" ;;
esac
