#!/usr/bin/env bash
# Parse /seal-proof and /triage webchat commands from GitHub event payloads.
set -euo pipefail

EVENT_NAME="${GITHUB_EVENT_NAME:?}"
EVENT_PATH="${GITHUB_EVENT_PATH:?}"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

body="$("$PYTHON_BIN" - <<'PY' "$EVENT_PATH" "$EVENT_NAME"
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
elif [[ "$body" =~ ^/clearance([[:space:]]|$) ]] || [[ "$body" == /clearance* ]]; then
  cmd="clearance"
elif [[ "$body" =~ ^/relay-lint([[:space:]]|$) ]] || [[ "$body" == /relay-lint* ]]; then
  cmd="relay-lint"
elif [[ "$body" =~ ^/orient([[:space:]]|$) ]] || [[ "$body" == /orient* ]]; then
  cmd="orient"
elif [[ "$body" =~ ^/anchor([[:space:]]|$) ]] || [[ "$body" == /anchor* ]]; then
  cmd="anchor"
else
  echo "COMMAND: ignore"
  exit 0
fi

if [[ "$cmd" == "seal-proof" ]]; then
  plan="false"
  profile="ci-b-webchat-smoke"
  probe=""
  if [[ "$body" =~ plan ]]; then plan="true"; fi
  if [[ "$body" =~ profile=([a-z0-9_-]+) ]]; then profile="${BASH_REMATCH[1]}"; fi
  if [[ "$body" =~ probe=([a-z0-9_-]+) ]]; then probe="${BASH_REMATCH[1]}"; fi
  if [[ "$profile" == owner-deep-* ]]; then
    echo "COMMAND: seal-proof-rejected reason=owner-deep-comment-path profile=${profile}"
    exit 0
  fi
  echo "COMMAND: seal-proof plan=${plan} profile=${profile} probe=${probe}"
  exit 0
fi

if [[ "$cmd" == "triage" ]]; then
  # /triage <scan-id> <delete|green|escalate> <reason>
  if [[ "$body" =~ ^/triage[[:space:]]+([^[:space:]]+)[[:space:]]+(delete|green|escalate)[[:space:]]+(.+)$ ]]; then
    triage_scan="${BASH_REMATCH[1]}"
    triage_outcome="${BASH_REMATCH[2]}"
    triage_reason="${BASH_REMATCH[3]}"
    triage_reason="${triage_reason#"${triage_reason%%[![:space:]]*}"}"
    triage_reason="${triage_reason%"${triage_reason##*[![:space:]]}"}"
    if bash "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/triage_log_check.sh" \
        --validate-append "$triage_scan" "$triage_outcome" "$triage_reason" >/dev/null 2>&1; then
      echo "COMMAND: triage scan=${triage_scan} outcome=${triage_outcome} reason=${triage_reason}"
      exit 0
    fi
  fi
  echo "COMMAND: triage-invalid"
  echo "FORMAT: /triage <scan-id> <delete|green|escalate> <reason>"
  exit 1
fi

if [[ "$cmd" == "clearance" ]]; then
  echo "COMMAND: clearance"
  exit 0
fi

if [[ "$cmd" == "relay-lint" ]]; then
  echo "COMMAND: relay-lint"
  exit 0
fi

if [[ "$cmd" == "orient" ]]; then
  role="orchestrator"
  if [[ "$body" =~ role=([a-z]+) ]]; then
    role="${BASH_REMATCH[1]}"
  fi
  echo "COMMAND: orient role=${role}"
  exit 0
fi

if [[ "$cmd" == "anchor" ]]; then
  target=""
  if [[ "$body" =~ ^/anchor[[:space:]]+([^[:space:]]+) ]]; then
    target="${BASH_REMATCH[1]}"
  fi
  [[ -n "$target" ]] || { echo "COMMAND: anchor-invalid"; exit 1; }
  echo "COMMAND: anchor target=${target}"
  exit 0
fi
