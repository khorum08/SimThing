#!/usr/bin/env bash
# Parse /seal-proof and /triage webchat commands from GitHub event payloads.
set -euo pipefail

EVENT_NAME="${GITHUB_EVENT_NAME:?}"
EVENT_PATH="${GITHUB_EVENT_PATH:?}"

body="$(python3 - <<'PY' "$EVENT_PATH" "$EVENT_NAME"
import json, sys
path, name = sys.argv[1], sys.argv[2]
ev = json.load(open(path, encoding="utf-8"))
if name == "issue_comment":
    print(ev.get("comment", {}).get("body", ""))
elif name == "pull_request_review":
    print(ev.get("review", {}).get("body", ""))
elif name == "pull_request_review_comment":
    print(ev.get("comment", {}).get("body", ""))
else:
    print("")
PY
)"

body="${body//$'\r'/}"

if [[ "$body" =~ ^/seal-proof([[:space:]]|$) ]] || [[ "$body" == /seal-proof* ]]; then
  cmd="seal-proof"
elif [[ "$body" =~ ^/triage[[:space:]] ]]; then
  cmd="triage"
else
  echo "COMMAND: ignore"
  exit 0
fi

if [[ "$cmd" == "seal-proof" ]]; then
  plan="false"
  profile="full-cpu"
  probe=""
  if [[ "$body" =~ plan ]]; then plan="true"; fi
  if [[ "$body" =~ profile=([a-z0-9_-]+) ]]; then profile="${BASH_REMATCH[1]}"; fi
  if [[ "$body" =~ probe=([a-z0-9_-]+) ]]; then probe="${BASH_REMATCH[1]}"; fi
  echo "COMMAND: seal-proof plan=${plan} profile=${profile} probe=${probe}"
  exit 0
fi

if [[ "$cmd" == "triage" ]]; then
  # /triage <scan-id> <delete|green|escalate> <reason>
  if [[ "$body" =~ ^/triage[[:space:]]+([^[:space:]]+)[[:space:]]+(delete|green|escalate)[[:space:]]+(.+)$ ]]; then
    echo "COMMAND: triage scan=${BASH_REMATCH[1]} outcome=${BASH_REMATCH[2]} reason=${BASH_REMATCH[3]}"
    exit 0
  fi
  echo "COMMAND: triage-invalid"
  echo "FORMAT: /triage <scan-id> <delete|green|escalate> <reason>"
  exit 1
fi