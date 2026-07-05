#!/usr/bin/env bash
# Update one sticky PR comment for relay-lint reports.
set -euo pipefail

PR="${1:-}"
BODY_FILE="${2:-relay-lint-report.txt}"
MARKER="<!-- relay-lint-sticky -->"

[[ -n "$PR" ]] || { echo "missing PR number" >&2; exit 1; }
[[ -f "$BODY_FILE" ]] || { echo "missing body file: $BODY_FILE" >&2; exit 1; }

REPO="${GITHUB_REPOSITORY:?}"
OWNER="${REPO%%/*}"
NAME="${REPO##*/}"

BODY="$(cat "$BODY_FILE")"
FULL_BODY="${MARKER}
## Relay Lint Report

\`\`\`
${BODY}
\`\`\`"

comment_id="$(gh api "repos/${OWNER}/${NAME}/issues/${PR}/comments" --paginate \
  --jq ".[] | select(.body | contains(\"${MARKER}\")) | .id" | head -n 1)"

if [[ -n "$comment_id" ]]; then
  gh api -X PATCH "repos/${OWNER}/${NAME}/issues/comments/${comment_id}" \
    -f body="$FULL_BODY" >/dev/null
  echo "updated sticky comment ${comment_id}"
else
  gh api -X POST "repos/${OWNER}/${NAME}/issues/${PR}/comments" \
    -f body="$FULL_BODY" >/dev/null
  echo "created sticky comment"
fi