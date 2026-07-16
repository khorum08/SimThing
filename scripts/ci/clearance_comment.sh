#!/usr/bin/env bash
# Update one sticky PR comment for clearance_check.sh output.
#
# This is a COSMETIC mirror: the authoritative clearance verdict is carried by the
# job step-summary and the uploaded clearance-report.txt artifact. The PR comment is
# a convenience surface only. GitHub's write path (POST/PATCH issue comments) returns
# HTML 5xx pages intermittently in-runner ("invalid character '<'..." from gh's JSON
# decoder); a transient comment-post hiccup must NOT red the clearance gate. So this
# script retries transient failures and, if it still cannot post, soft-fails (warns,
# exits 0) — matching the non-fatal treatment the handoff-ingress sticky already gets.
set -uo pipefail

PR="${1:-}"
BODY_FILE="${2:-clearance-report.txt}"
MARKER="<!-- clearance-sticky -->"

[[ -n "$PR" ]] || { echo "missing PR number" >&2; exit 1; }
[[ -f "$BODY_FILE" ]] || { echo "missing body file: $BODY_FILE" >&2; exit 1; }

REPO="${GITHUB_REPOSITORY:?}"
OWNER="${REPO%%/*}"
NAME="${REPO##*/}"

BODY="$(cat "$BODY_FILE")"
FULL_BODY="${MARKER}
## Clearance Report

\`\`\`
${BODY}
\`\`\`"

# Retry a gh invocation up to 3 times over transient failures (5xx HTML pages, network
# blips). Returns the command's final rc; caller decides fatality.
gh_retry() {
  local attempt rc
  for attempt in 1 2 3; do
    "$@" && return 0
    rc=$?
    [[ $attempt -lt 3 ]] && sleep $((attempt * 2))
  done
  return "$rc"
}

# Look up an existing sticky (tolerate a flaky read: empty id just means "post fresh").
comment_id="$(gh_retry gh api "repos/${OWNER}/${NAME}/issues/${PR}/comments" --paginate \
  --jq ".[] | select(.body | contains(\"${MARKER}\")) | .id" 2>/dev/null | head -n 1 || true)"

if [[ -n "$comment_id" ]]; then
  if gh_retry gh api -X PATCH "repos/${OWNER}/${NAME}/issues/comments/${comment_id}" \
      -f body="$FULL_BODY" >/dev/null 2>&1; then
    echo "updated sticky comment ${comment_id}"
    exit 0
  fi
else
  if gh_retry gh api -X POST "repos/${OWNER}/${NAME}/issues/${PR}/comments" \
      -f body="$FULL_BODY" >/dev/null 2>&1; then
    echo "created sticky comment"
    exit 0
  fi
fi

# Could not post the cosmetic mirror after retries. The verdict is authoritative in the
# step summary + clearance-report.txt artifact; do not fail the gate on a comment blip.
echo "::warning::clearance sticky comment could not be posted (transient GitHub write-path failure); verdict is in the step summary and clearance-report.txt artifact"
exit 0
