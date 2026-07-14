#!/usr/bin/env bash
# Parse doctrine-exec and handoff webchat commands from GitHub event payloads.
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
elif [[ "$body" =~ ^/handoff([[:space:]]|$) ]] || [[ "$body" == /handoff* ]]; then
  cmd="handoff"
elif [[ "$body" =~ ^/triage[[:space:]] ]]; then
  cmd="triage"
elif [[ "$body" =~ ^/clearance([[:space:]]|$) ]] || [[ "$body" == /clearance* ]]; then
  cmd="clearance"
elif [[ "$body" =~ ^/relay-lint([[:space:]]|$) ]] || [[ "$body" == /relay-lint* ]]; then
  cmd="relay-lint"
elif [[ "$body" =~ ^/orient([[:space:]]|$) ]] || [[ "$body" == /orient* ]]; then
  cmd="orient"
elif [[ "$body" =~ ^/librarian([[:space:]]|$) ]] || [[ "$body" == /librarian* ]]; then
  cmd="librarian"
elif [[ "$body" =~ ^/anchor([[:space:]]|$) ]] || [[ "$body" == /anchor* ]]; then
  cmd="anchor"
else
  echo "COMMAND: ignore"
  exit 0
fi

if [[ "$cmd" == "handoff" ]]; then
  body_b64="$(printf '%s' "$body" | base64 | tr -d '\n')"
  COMMAND_BODY_B64="$body_b64" "$PYTHON_BIN" - <<'PY'
import base64
import os
import sys

body = base64.b64decode(os.environ.get("COMMAND_BODY_B64", "").encode("ascii")).decode("utf-8").strip()
if not body.lower().startswith("/handoff"):
    print("COMMAND: handoff-invalid")
    print("FORMAT: /handoff approve | /handoff amend: <text> | /handoff hold | /handoff status")
    sys.exit(1)
rest = body[len("/handoff"):].strip()
if not rest:
    print("COMMAND: handoff-invalid")
    print("FORMAT: /handoff approve | /handoff amend: <text> | /handoff hold | /handoff status")
    sys.exit(1)
head, sep, tail = rest.partition(" ")
action = head.rstrip(":").lower()
text = ""
if action == "amend":
    if head.endswith(":"):
        text = tail.strip()
    elif sep:
        text = tail.strip()
    else:
        text = ""
    if not text:
        print("COMMAND: handoff-invalid")
        print("FORMAT: /handoff amend: <text>")
        sys.exit(1)
else:
    if action not in {"approve", "hold", "status"} or sep or tail.strip():
        print("COMMAND: handoff-invalid")
        print("FORMAT: /handoff approve | /handoff amend: <text> | /handoff hold | /handoff status")
        sys.exit(1)
encoded = base64.urlsafe_b64encode(text.encode("utf-8")).decode("ascii")
print(f"COMMAND: handoff action={action}")
print(f"HANDOFF_TEXT_B64: {encoded}")
PY
  exit $?
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

if [[ "$cmd" == "librarian" ]]; then
  body_b64="$(printf '%s' "$body" | base64 | tr -d '\n')"
  COMMAND_BODY_B64="$body_b64" "$PYTHON_BIN" - <<'PY'
import base64
import os
import shlex
import sys

body = base64.b64decode(os.environ.get("COMMAND_BODY_B64", "").encode("ascii")).decode("utf-8").strip()
try:
    parts = shlex.split(body)
except ValueError:
    print("COMMAND: librarian-invalid")
    print("FORMAT: /librarian staleness | /librarian cull [--confirm] | /librarian catalog [--role coding|orchestrator|da]")
    sys.exit(1)
if not parts or parts[0].lower() != "/librarian" or len(parts) < 2:
    print("COMMAND: librarian-invalid")
    print("FORMAT: /librarian staleness | /librarian cull [--confirm] | /librarian catalog [--role coding|orchestrator|da]")
    sys.exit(1)
action = parts[1].lower()
confirm = "false"
role = ""
rest = parts[2:]
if action == "staleness":
    if rest:
        print("COMMAND: librarian-invalid")
        print("FORMAT: /librarian staleness")
        sys.exit(1)
elif action == "cull":
    if rest == ["--confirm"]:
        confirm = "true"
    elif rest:
        print("COMMAND: librarian-invalid")
        print("FORMAT: /librarian cull [--confirm]")
        sys.exit(1)
elif action == "catalog":
    if not rest:
        role = ""
    elif len(rest) == 2 and rest[0] == "--role" and rest[1] in {"coding", "orchestrator", "da"}:
        role = rest[1]
    else:
        print("COMMAND: librarian-invalid")
        print("FORMAT: /librarian catalog [--role coding|orchestrator|da]")
        sys.exit(1)
else:
    print("COMMAND: librarian-invalid")
    print("FORMAT: /librarian staleness | /librarian cull [--confirm] | /librarian catalog [--role coding|orchestrator|da]")
    sys.exit(1)
print(f"COMMAND: librarian action={action} confirm={confirm} role={role}")
PY
  exit $?
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
