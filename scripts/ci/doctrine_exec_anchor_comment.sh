#!/usr/bin/env bash
# Post the /anchor report as sticky PR comments. Anchors have no size cap (Owner, 2026-09-28),
# but one GitHub comment holds 65,536 characters, so a longer report is paged: part 1 keeps
# the original sticky marker, parts 2..n carry their own, and parts left over from an earlier,
# longer report are marked superseded instead of deleted. Nothing is ever truncated.
#
# usage: doctrine_exec_anchor_comment.sh <pr> [report-file]
#        doctrine_exec_anchor_comment.sh --selftest
set -euo pipefail

PART_CHARS="${ANCHOR_COMMENT_PART_CHARS:-60000}"
PYTHON_BIN="${PYTHON_BIN:-python3}"
command -v "$PYTHON_BIN" >/dev/null 2>&1 || PYTHON_BIN="python"

# Split <file> into <file>.part-1..n of at most PART_CHARS characters, at line boundaries
# (a single longer line is cut losslessly); prints n.
split_parts() {
  "$PYTHON_BIN" - "$1" "$PART_CHARS" <<'PY'
import sys

path, limit = sys.argv[1], int(sys.argv[2])
with open(path, encoding="utf-8", newline="") as fh:
    text = fh.read()
parts, current = [], ""
for line in text.splitlines(keepends=True):
    while len(line) > limit:
        if current:
            parts.append(current)
            current = ""
        parts.append(line[:limit])
        line = line[limit:]
    if len(current) + len(line) > limit:
        parts.append(current)
        current = ""
    current += line
if current or not parts:
    parts.append(current)
for i, part in enumerate(parts, 1):
    with open(f"{path}.part-{i}", "w", encoding="utf-8", newline="") as fh:
        fh.write(part)
print(len(parts))
PY
}

marker_for() {
  if [[ "$1" -eq 1 ]]; then printf '<!-- anchor-sticky -->'; else printf '<!-- anchor-sticky part %s -->' "$1"; fi
}

selftest() {
  local tmp n rebuilt fails=0
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/anchor-comment-XXXXXX")"
  "$PYTHON_BIN" -c 'import sys; open(sys.argv[1], "w", encoding="utf-8", newline="").write("head\n" + "ä" * 70000 + "\n" + "row\n" * 30000)' "$tmp/report.txt"
  n="$(PART_CHARS=60000 split_parts "$tmp/report.txt")"
  rebuilt="$("$PYTHON_BIN" -c '
import sys
parts = [open(f"{sys.argv[1]}.part-{i}", encoding="utf-8", newline="").read() for i in range(1, int(sys.argv[2]) + 1)]
whole = open(sys.argv[1], encoding="utf-8", newline="").read()
print("lossless" if "".join(parts) == whole else "lossy", "fits" if all(len(p) <= 60000 for p in parts) else "overflow")
' "$tmp/report.txt" "$n")"
  if [[ "$rebuilt" == "lossless fits" && "$n" -ge 3 ]]; then echo "PASS anchor_comment_pages_losslessly"; else echo "FAIL anchor_comment_pages_losslessly ($rebuilt, parts=$n)"; fails=1; fi
  printf 'short report\n' >"$tmp/small.txt"
  n="$(split_parts "$tmp/small.txt")"
  if [[ "$n" -eq 1 && "$(cat "$tmp/small.txt.part-1")" == "short report" ]]; then echo "PASS anchor_comment_small_is_one_part"; else echo "FAIL anchor_comment_small_is_one_part"; fails=1; fi
  if [[ "$(marker_for 1)" == "<!-- anchor-sticky -->" && "$(marker_for 12)" != *"$(marker_for 1)"* ]]; then echo "PASS anchor_comment_markers_distinct"; else echo "FAIL anchor_comment_markers_distinct"; fails=1; fi
  rm -rf "$tmp"
  if [[ "$fails" -eq 0 ]]; then echo "ANCHOR-COMMENT-SELFTEST: PASS"; return 0; fi
  echo "ANCHOR-COMMENT-SELFTEST: FAIL"
  return 1
}

if [[ "${1:-}" == "--selftest" ]]; then
  selftest
  exit $?
fi

PR="${1:-}"
BODY_FILE="${2:-anchor-report.txt}"
[[ -n "$PR" ]] || { echo "missing PR number" >&2; exit 1; }
[[ -f "$BODY_FILE" ]] || { echo "missing body file: $BODY_FILE" >&2; exit 1; }
REPO="${GITHUB_REPOSITORY:?}"
COMMENTS="repos/${REPO}/issues/${PR}/comments"

n="$(split_parts "$BODY_FILE")"
for ((k = 1; k <= n; k++)); do
  marker="$(marker_for "$k")"
  footer=""
  [[ "$n" -gt 1 ]] && footer=$'\n'"part ${k}/${n} of this /anchor report"
  { printf '%s\n```\n' "$marker"; cat "${BODY_FILE}.part-${k}"; printf '\n```%s\n' "$footer"; } >"${BODY_FILE}.comment-${k}"
  comment_id="$(gh api "$COMMENTS" --paginate --jq ".[] | select(.body | contains(\"${marker}\")) | .id" | head -n 1)"
  if [[ -n "$comment_id" ]]; then
    gh api -X PATCH "repos/${REPO}/issues/comments/${comment_id}" -F body=@"${BODY_FILE}.comment-${k}" >/dev/null
    echo "updated sticky comment ${comment_id} (part ${k}/${n})"
  else
    gh api -X POST "$COMMENTS" -F body=@"${BODY_FILE}.comment-${k}" >/dev/null
    echo "created sticky comment (part ${k}/${n})"
  fi
done

# Parts beyond n belong to an earlier, longer report: mark them superseded, never delete them.
gh api "$COMMENTS" --paginate \
  --jq '.[] | select(.body | test("<!-- anchor-sticky part [0-9]+ -->")) | "\(.id) \(.body | capture("<!-- anchor-sticky part (?<k>[0-9]+) -->").k)"' |
while read -r comment_id k; do
  if [[ "$k" -gt "$n" ]]; then
    printf '%s\n_(superseded: the current /anchor report has %s part(s))_\n' "$(marker_for "$k")" "$n" >"${BODY_FILE}.superseded"
    gh api -X PATCH "repos/${REPO}/issues/comments/${comment_id}" -F body=@"${BODY_FILE}.superseded" >/dev/null
    echo "marked sticky part ${k} superseded (${comment_id})"
  fi
done
