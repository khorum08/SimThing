---
rung: REVIEW-EVENT-JSON-TRANSPORT-0
kind: remedial
track: 0.0.8.7
base_sha: 8cbdbd04cf1660c452cca0ea14a7b10d10a3009d
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "DA Fable ruling 5391413241 accepted 11.1c substance but withheld merge solely because review-triggered parse-command is RED. The defect is security-relevant: inline single-quoted toJson(PR) makes apostrophes shell syntax and can become command injection under GH_TOKEN. Fable authorized this bounded harness repair and committed to merge/stamp/rung-close 11.1c once parse-command is green."
surfaces: [".github/workflows/doctrine-exec-commands.yml", ".github/workflows/doctrine-scan.yml", "scripts/ci/doctrine_exec_review_context_selftest.py", "scripts/ci/fixtures/doctrine_exec_review_context/review_events.json", "scripts/ci/test_inventory.tsv", "scripts/ci/anchor_reach_log.tsv"]
forbidden: ["any change to PR #1811 implementation, tests, evidence, or semantics", "changing parse-command command semantics, workflow permissions, collaborator/fork/auth policy, clearance, relay-lint, or gate strictness", "11.1d or later Phase-11 work, #1803/11.2, pointer movement, merge, graduation, or any engine source", "fixing only the observed apostrophe while leaving PR JSON interpolated into shell source"]
required_checks: ["fresh Frontier coding ORIENT receipt at current rule stamp; ACK every rendered REQUIRED-ANCHOR", "replace both pull_request_review and pull_request_review_comment unsafe full-PR JSON shell interpolations with data-only transport; no full PR JSON may become Bash source", "fixture/selftest must execute the actual Resolve PR context block for both review event types with an apostrophe-bearing hostile PR body and a command sentinel", "prove sentinel non-execution and exact preservation of PR number, comment/review id, head/base SHA, head ref, merged status and checkout selection", "restoring either unsafe inline single-quoted JSON assignment must make the proof RED for its own reason", "wire the proof into the existing delta-gated Doctrine selftest/report lane only; inventory and anchor reach rows mechanically updated", "hosted Doctrine Scan and Doctrine Exec PASS at one exact head; return exact base/head, changed paths, fixture result and workflow IDs"]
stop_conditions: ["safe transport appears to require changing command semantics, permissions, auth/fork policy, or another workflow authority", "the hostile fixture can still execute its command sentinel", "the repair requires touching #1811 or any Phase-11 implementation surface"]
---
## BUILD
- Harden the two review-event `Resolve PR context` branches in `.github/workflows/doctrine-exec-commands.yml`. PR JSON must be read as data through runner-provided event transport (for example `$GITHUB_EVENT_PATH` + `jq`) rather than interpolated inline into Bash source.
- Add one focused two-branch selftest fixture that carries an apostrophe plus a command-injection sentinel and exercises the actual workflow block.
- Wire that selftest into the existing delta-gated Doctrine Scan selftest/report path; make only the mechanically required inventory and anchor-reach updates.
## FENCES
- No #1811 byte or semantic change. This is a harness repair only.
- Do not alter parse-command behavior, permissions, collaborator/fork/auth checks, clearance/relay semantics, or gate strictness.
- Do not start 11.1d+, #1803/11.2, or move the pointer.
## EXIT-PROOF
- Both `pull_request_review` and `pull_request_review_comment` hostile fixtures parse and execute the real context block, emit exact expected context, and never execute the sentinel.
- Restoring either branch to inline single-quoted PR JSON REDs the proof.
- Hosted Doctrine Scan and Doctrine Exec are green at the exact returned head; orchestration owns clearance/relay-lint and DA routing.
