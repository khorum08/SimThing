# CI Doctrinal Screening Surface — auditable reference

> **What this is.** The single authoritative map of the `0.0.8.4.6` CI doctrinal-screening layer (Tracks A + C,
> CLOSED): what each file is, how the screening logic works, the **strict rigor** to change the scan and
> allow/block lists (§4), the **triage agent's** role in keeping the surface narrow (§5), the **live carrot**
> — digest / inner-loop / per-track addenda (§6), the **agent onboarding procedure** (§7), and the **per-track
> addendum authoring standards** (§8). It exists so the layer is **auditable** (anyone can read exactly what is
> screened and why), **maintainable** (one correct, low-risk way to change it), and **usable by agents**
> (a clear, ordered onboarding standard, so a low-context agent works with the surface instead of re-deriving it).
>
> **Governing docs:** the track [`design_0_0_8_4_6_ci_scaffolding.md`](design_0_0_8_4_6_ci_scaffolding.md)
> (§0 decisions, §1 verdict contract, §1A triage), the operational [`scripts/ci/README.md`](../scripts/ci/README.md),
> and the doctrine landed in core design §1.2/§1.2.1, constitution §0.x, and the handoff-template spine.
> This file is the reference; those are the source of authority. Keep them consistent — a change to the
> screening surface updates **this** file in the same PR.

---

## 1. Screening logic — how a change gets judged

Every `pull_request` and every `push` runs the **Doctrine Scan** GitHub Actions workflow on `ubuntu-latest`
(~1 min, free — public repo). The pipeline, in order:

```
checkout → ensure rg (preinstalled + apt fallback)
  → digest freshness (gen_digest.sh --check)  # stale sanctioned_surface.md hard-FAILs with regenerate remedy
  → self-test        (doctrine_selftest.sh)      # prove the scanner still catches its known-bads, or the whole run FAILs
  → PR-delta scan    (doctrine_pr_scan.sh)        # on pull_request: RELIABLE whole-tree, HEURISTIC on the diff only
  → spam check       (inspect_spam_check.sh)      # §1A hill-climbing bounds
  → whole-tree scan  (doctrine_scan.sh)           # on push to master: the positive control
  → publish report   (job summary + artifact)
```

**Three verdicts, never two** (residue-as-tripwire applied to the scanner):

| Verdict | Meaning | Blocks? | Routes to |
|---|---|---|---|
| **PASS** | clean; for a RELIABLE/allowlist scan this is **DA-equivalent** ("the DA ran it") | — | nobody — trusted without DA re-verification |
| **FAIL** | a RELIABLE scan hit a definitive violation, **or** the self-test rotted | **yes** (= DA HOLD) | author fixes the code, or adds a *conforming* allowlist record |
| **INSPECT** | a HEURISTIC hit, or a hit in a known false-positive zone — a grep can't adjudicate | no | **§1A triage** (never straight to the DA, never silently merged) |

The machine-parseable footer the orchestrator keys on:
`DOCTRINE-SCAN-VERDICT: PASS|FAIL|INSPECT  failures=N inspect=M selftest=PASS|FAIL`.

**Delta vs whole-tree (binding).** HEURISTIC scans are evaluated **on the PR diff only** in CI; RELIABLE
scans stay **whole-tree** (you want zero of those anywhere, always). A whole-tree HEURISTIC scan re-flags the
pre-existing baseline on every PR (~81 legitimate hits on master) and would drown triage — so per-PR HEURISTIC
is delta-scoped, and the whole-tree run is only the master-push positive control. The §1A spam-bounds count
**branch-introduced (delta)** INSPECTs, never baseline.

---

## 2. The files — the auditable surface

Everything lives under `scripts/ci/`. Heuristics and allowlists are **data**; the engines are thin and carry
**no invariant-specific patterns**.

### Data (the screening definitions — edit these, not the engines)
| File | Kind | What it holds |
|---|---|---|
| `scans.tsv` | scan definitions | one scan per line, 7 fields: `id \| severity \| target-glob \| pattern \| exclude \| doctrine-ref \| promotion-blocker` |
| `allow/sealed_producers.txt` | **allowlist** | the sanctioned producer doors for sealed types (`read_*`/`readback_*`/`dispatch_*`/`apply_*`/`cpu_oracle_*`) — anything else that produces a sealed type FAILs |
| `allow/kernel_surface.txt` | **allowlist** | the closed set of `simthing-kernel` `lib.rs` exports, classed `surface-inert` / `authority-export` / `sealed-export` (never the wildcard `inert-util`) |
| `allow/inert_buffer_handles.txt` | **allowlist** | provably-inert public buffer utilities (`inert-util` only) |
| `allow/sealed_types.txt` | data list | the closed set of sealed authority **type names** (bare names). Loaded by `scan_allowlists.py`; missing/empty fails loudly |
| `inspect_justifications.tsv` | triage telemetry | per-INSPECT author justification (an INSPECT with none is `unresolved`) |
| `triage_log.tsv` | triage telemetry | append-only `scan-id \| branch \| outcome(delete/green/escalate) \| reason \| commit` — **also the per-scan promotion telemetry** |

### Engines (thin — change only when the *format/report* changes, never for an invariant)
| File | Role |
|---|---|
| `doctrine_scan.sh` | reads `scans.tsv` + `allow/`, applies each scan (`rg -U`), emits the report + footer; every FAIL prints its sanctioned remedy |
| `scan_allowlists.py` | the closed-set allowlist scans (producers / buffer-handles / kernel-surface); loads `sealed_types.txt` from data |
| `doctrine_pr_scan.sh` | PR-delta wrapper: RELIABLE whole-tree + HEURISTIC on the diff |
| `doctrine_selftest.sh` | the rot-guard: runs every RELIABLE scan against its known-bad (must FAIL) + the trap corpus + clean master (must NOT FAIL); tool-missing emits FAIL, never a false PASS |
| `inspect_spam_check.sh` | the §1A hill-climbing bounds → `INSPECT-SPAM-CHECK: SPAM|OK` |
| `audit_kernel_surface.py` / `verify_kernel_surface.py` | re-derive / diff `kernel_surface.txt` against `lib.rs` (both `pub use` forms) |
| `fixtures/` | known-bad inputs (one per RELIABLE scan) + false-positive traps + HEURISTIC production negative controls; `fixtures/README.md` maps fixture → scan → expected verdict |
| `.github/workflows/doctrine-scan.yml` | the authoritative gate (runs entirely on GitHub) |

---

## 3. Blocklist vs allowlist — the two screening modes

- **RELIABLE blocklist scan** (`scans.tsv`, severity `RELIABLE`, a `pattern`): a hit **is** a violation → FAIL.
  Fast belt-and-suspenders for known holes (e.g. a re-added forge minter, a `&Buffer` escaping the kernel).
- **Closed-set allowlist scan** (`scan_allowlists.py` over `allow/*.txt`): enumerate the **sanctioned** surface;
  *anything outside it FAILs.* This is the strongest form — it catches **novel and subtle** holes a blocklist
  can't name in advance (a `#[doc(hidden)] pub fn -> Self` minter fails with no per-name pattern). A clean
  allowlist result is **trusted without DA re-verification**; the DA's only standing engagement is reviewing an
  allowlist **edit**.
- **HEURISTIC scan** (`scans.tsv`, severity `HEURISTIC`): fuzzy by nature (semantic words, raw indices, `.kind`
  reads) → **INSPECT**, never a hard FAIL. Surfacing-not-blocking is correct; tightening one into a
  type/admission boundary is the §1.2 promotion path.

---

## 4. Strict rigor to add or change a `scans.tsv` entry

A `scans.tsv` line is a **doctrinal claim**, not a convenience. The bar to add one is deliberately high — the
layer is **designed to shrink**, and a growing scan count is a regression signal, not progress.

1. **All seven fields present.** Malformed rows are a scanner/data error (loud FAIL), never skipped.
2. **A RELIABLE scan MUST carry a real `promotion-blocker`** — the type/admission boundary that would make it
   redundant (e.g. *"retire when `ColumnIndex` is a kernel-wide newtype"*). An empty promotion-blocker on a
   RELIABLE scan is a flagged anomaly: *why is this prose-guarded instead of typed?*
3. **First ask "should this be a type, not a scan?"** A grep scan is rung 3 of the admission ladder; it exists
   only because the invariant isn't yet a type boundary (rung 1). Prefer promotion. When an invariant *does*
   get promoted, **the same PR deletes the now-redundant scan** (the retirement contract).
4. **HEURISTIC scans are few and budgeted.** Each must have tuned `exclude` patterns (comments, `#[cfg(test)]`,
   string/display lines) so its master baseline is small, and it must be delta-scoped in PR CI. A HEURISTIC that
   fires chronically without ever surfacing a real finding is **promoted or deleted**, not left to erode INSPECT.
5. **No invariant in the engine.** The pattern/target/exclude live in the data line; `doctrine_scan.sh` stays
   pattern-free. (The sealed-type *name set* likewise lives in `allow/sealed_types.txt`, not in Python.)
6. **Prove it or it doesn't land.** A new RELIABLE scan needs a known-bad fixture it FAILs on **and** a trap it
   stays quiet on, both wired into `doctrine_selftest.sh`; a HEURISTIC needs a production negative control
   proving its excludes didn't no-op it. The self-test must stay green on master.
7. **Multiline-robust.** Patterns must survive a declaration split across lines (`rg -U`); prove it with a
   split-declaration fixture. A scan that can't be made robust is downgraded to INSPECT, never a silent RELIABLE.

### Adding a sanctioned-surface (allowlist) entry — the rustified onboarding heuristic
An `allow/*.txt` record is a **typed admission record**, not a list you pad. The scanner enforces its form:
- Format `symbol | door-class | rationale | promotion-blocker` — **every field required** (a missing field FAILs).
- **The `symbol` name must match its `door-class` grammar** (`read`→`read_*`, `apply`→`apply_*`, `cpu_oracle`→`cpu_oracle_*`, …). You **cannot** file `forge_x` under `read`.
- **`inert-util` is reserved** for genuinely-inert constants/helpers. Sealed/authority exports are **never**
  `inert-util` — that wildcard would launder the high-authority surface into "inert" and defeat the legitimacy
  check. Sealed types carry `sealed-export`; their minters are grammar-enforced in `sealed_producers.txt`.
- The **rationale lives in the data** (auditable in the diff) — this replaces "remember to justify it in the PR
  description" with an enforced admission gate. Casual cruft is structurally rejected; a real new feature is
  accommodated by one conforming, self-describing line.

---

## 5. The triage agent's role — keeping the surface narrow and disciplined

INSPECT is a **cost-asymmetry loophole**: free for an agent to trip a HEURISTIC, expensive for the DA to clear.
The **triage agent** (the orchestrator tier — a *free/unmetered* model, deliberately a **different family** from
the coding agent) sits between the scanner and the scarce DA and resolves every INSPECT to one terminal state:

- **DELETE** — a genuine false-positive; logged with its reason; the PR proceeds.
- **GREEN** — the agent fixed the underlying issue; triage **verifies *why* it is legitimate** (correct
  door-class, real rationale, violation actually gone) — *not* that the scanner merely went quiet.
- **ESCALATE** — a real gray zone → the DA, with reasoning attached (the DA verifies a *claim*, not derives a
  finding from nothing).

Four fences keep triage honest (full protocol: track §1A):
1. **The agent pays first** — a one-line justification per INSPECT before it's triage-eligible.
2. **Bounded loop** — hard cap of 3 attempts, then auto-ESCALATE.
3. **Spam-bounds → FAIL-equivalent** — escalate immediately on any hill-climbing signature: >3 branch-introduced
   INSPECTs; the same symbol tripping ≥2 different HEURISTIC scan-ids (symbol-walking); INSPECT rising while a
   RELIABLE FAIL stays open.
4. **Decorrelated reviewer + DA spot-audit** — the DA samples a % of DELETE/GREEN clearances against the tree
   until triage accuracy is established (a clearance is named residue `triage-cleared-uninspected`).

For webchat-driven executable proof and remote §1A triage commands, see §9.

**Why the triage agent is the discipline mechanism, not just a filter:** its escalation log (`triage_log.tsv`)
*is* the per-scan-id promotion telemetry. A HEURISTIC that keeps reaching ESCALATE is, by construction, a scan
that needs promotion or deletion. So the standing **corpus-maintenance cadence** (per-track-closeout or
scheduled) reads that log and prunes/promotes/retires — turning "keep the lists narrow" from an aspiration into
a data-driven action. The triage agent also gate-keeps **allowlist widenings**: every added sanctioned door is a
deliberate, reviewed, diff-visible edit, and triage flags a widening that isn't grammar-conforming or
self-justifying. Narrow by construction; open only to legitimate, justified growth.

### Graduation routing — corpus + rationale → posture (how the DA spends tokens)

The triage log is the DA's telemetry, but it captures **one** risk axis — INSPECT / heuristic. It is *necessary
but not sufficient* for deciding how deep a graduation review must go. **Track C's corpus proved this
empirically:** it holds a single GREEN row (the C1 inner-loop demo), and `CI-C-DIGEST-0` raised **zero** INSPECTs
— yet C2 needed a *deep* review, because its risk was structural (a machine-parsed digest whose freshness was
ungated), invisible to the triage log. A DA routing graduation from the log alone would have graduated C2 light
and missed it.

So the DA routes graduation depth from **two** inputs: the **triage log** (*what fired*) **and** the
orchestrator's **Graduation-routing block** (handoff §11 — *the structural risk class the scanner can't see, and
the exact falsification check*). The orchestrator never self-marks COMPLETE; it relays PROBATION with that block,
and the DA applies:

| Declared risk class | Posture | What the DA actually does |
|---|---|---|
| `none` + green RELIABLE + no escalation | **light** | confirm the named deliverables exist; graduate |
| `semantic` | targeted | one judgment check (e.g. call-site analysis of a new accessor) |
| `data-deliverable` / `gate-wiring` | **deep** | byte-faithfulness against the source; *prove the guard bites* (perturb → FAIL) |
| `seal-residue` / `allowlist-edit` | **deep** | tree / legitimacy audit (door-class, no laundering) |
| triage `ESCALATE` present | **deep** | the escalation was already headed to the DA |

The **Falsification check** in the block tells the DA *where* to spend on a deep review — never "re-verify
everything." This is what makes graduation cost *decay*: a well-declared PROBATION lets the DA confirm-deliverables
in seconds where CI already vouches, and reserve token-heavy investigation for exactly the residue CI cannot see.

---

## 5A. Orchestrator guidance — the operational contract (constitution §0.9.7 is the authority)

> **A NEW orchestration session is not qualified to route work until it has read:** this document
> (whole), constitution `design_0_0_8_3.md` §0.9, core design §1.2/§1.2.1,
> `design_0_0_8_4_5_simthing_kernel.md` §5.2 (the B1–B8 bypass catalogue), and `handoff_template.md`.
> Skipping this list is how orchestration sessions miss standing rulings, re-derive settled decisions,
> or route gate-state work to the wrong tier — each is a recorded, repeated failure mode.

**Standing responsibilities (every session, every rung):**
1. **Triage-log stewardship.** Every INSPECT routes through the §1A loop and lands a row in
   `scripts/ci/triage_log.tsv` (delete/green/escalate + reason + commit) — never a silent pass, never a
   straight-to-DA relay. The log is the promotion telemetry the corpus-maintenance cadence reads; an
   unlogged clearance is invisible and therefore did not happen.
2. **Closure hygiene.** Temporary `test_edit_scope.tsv` rows re-seal when their wave closes (their
   retirement condition names the wave); stale scope rows left behind are a recurring audit finding.
3. **Verify the tree, never the relayed report.** An implementer's transcript is a claim. Before relaying
   a proof upward or authorizing a merge, confirm the branch state (SHA-bound verdicts exist for exactly
   this — a report is stale unless `head_sha` equals the current PR head).
4. **Never self-mark COMPLETE.** Rungs relay as PROBATION with the Graduation-routing block; graduation is
   the DA's write (or your own merge authority where clause §0.9.7 applies — see below).

**Handoffs:** every rung handoff fills `handoff_template.md` — context spine **verbatim**, recipient by the
routing table (coding → Cursor/Grok; docs → Haiku/Sonnet; DA judgment → Opus/Owner), rung-local reading
≤6 files, one load-bearing proof per regression class, and the §11 response format. A handoff violating §H
(batteries, bespoke guards, triple-docs, inline implementations, inert scaffolding) is **rejected at
review, not implemented** — rejecting it is your job, not the DA's.

**Asserting merge authority (constitution §0.9.7 — the full contract governs; summary):** you MAY merge
without DA escalation only when the rung is a **precedented wave class under a standing ruling** with risk
class `none`/`semantic`/`data-deliverable`, is NOT gate-wiring / seal-residue / allowlist-edit /
protected-corpus / first-of-class, all RELIABLE gates are green on the head with SHA-bound targeted proof
where a profile exists, and you have filed the Graduation-routing block **plus a one-paragraph merge
rationale in the PR thread before merging**. Any doubt, novelty, or precedent-setting element → escalate
(the #1106 escalation is the calibration model: insisting on DA review when a stack smelled wrong was
correct). The DA spot-audits self-authorized merges against the tree; one wrong self-merge suspends the
authority. Owner supremacy sits above everything, visible and recorded.

**Channeling DA token spend (the routing table above is the mechanism — feed it honestly):**
- **Declare risk classes truthfully and completely** — under-declaring to earn a light review is the
  laundering move the spot-audit exists to catch; over-declaring burns the DA turn the regime exists to save.
- **Write the Falsification check as an executable instruction** ("run X, expect Y; perturb Z, expect FAIL")
  — the DA should be able to spend tokens exactly there and nowhere else.
- **Batch escalations** per review cycle; lead every relay with the verdict-relevant facts (what changed,
  what proves it, what the DA must decide); never bury a HOLD-worthy fact mid-report.
- **Never relay an unverified claim as fact** — verify against the tree first, or mark it explicitly
  `unverified`. The DA reconstructing truth from git because a relay obscured it costs more than the
  review it replaced.

---

## 6. Track C — the live carrot (the scanner pulled forward)

Track C (**CLOSED 2026-07-01**) slid this same artifact set into all three pipeline positions. It adds **no new
source of truth** — it consumes the data in §2, so the discipline that keeps CI honest keeps the agent honest.

- **BEFORE generation — the sanctioned-surface digest.** `docs/sanctioned_surface.md`, generated by
  `scripts/ci/gen_digest.sh` from `allow/*.txt` + `scans.tsv`. It is the agent's **pre-computed grep answer**:
  the only kernel doors an agent may call (with door-class + rationale), the sealed types, and the forbidden
  patterns — read it instead of grepping `lib.rs` to rediscover the surface. **Freshness is CI-enforced:** the
  workflow runs `gen_digest.sh --check` (under `set -o pipefail`), so a stale digest hard-FAILs with a
  regenerate remedy — the digest can never silently lie.
- **DURING generation — the inner-loop self-scan.** After each small edit, run `cargo check -p <touched-crate>`
  and `bash scripts/ci/doctrine_scan.sh`. The FAIL-with-remedy is a steering signal that prunes a doomed path in
  your own loop before it reaches a PR / CI / triage / DA. Replaces the *"did I violate a rule"* greps.
- **AFTER generation — the CI gate.** The GitHub `Doctrine Scan` (§1). FAIL-as-teacher prints `file:line` + the
  remedy, so you don't grep to *locate* a violation.
- **Introspection — the data is the interface.** The `DOCTRINE-SCAN-VERDICT:` footer, `triage_log.tsv`, and the
  closed-set `allow/*.txt` answer *"what is screened / fire-rate per scan / retirement candidates / how wide is
  the surface"* — greppable/parseable, no dashboard.

**The through-line:** one artifact set (`scans.tsv` + `allow/*.txt` + `triage_log`) serves three positions
(digest **before**, inner-loop **during**, CI gate **after**). Keeping it narrow and honest is what makes all
three trustworthy.

---

## 7. Agent onboarding procedure — do this, in order, every rung (the standard)

1. **Read the digest first; don't grep for the surface.** If your rung touches `simthing-kernel` or a consumer of
   it, read `docs/sanctioned_surface.md` — the authoritative, freshness-gated list of doors you may call. It is
   the pre-computed answer; do not rediscover the surface by grepping `lib.rs`.
2. **Run the inner loop as you edit.** After each small edit: `cargo check -p <touched-crate>`, then
   `bash scripts/ci/doctrine_scan.sh`. Fix a FAIL immediately from its printed remedy; do not accumulate.
3. **On a FAIL:** fix the violation, **or** — only if it is a legitimately new sanctioned door — add a conforming
   `allow/*.txt` record per §4. **Never edit the scanner to dodge a valid finding.** Match repair posture to
   failure class (error-adaptive repair, arXiv:2606.31706): a scanner/allowlist FAIL is token-cheap — apply the
   printed remedy; a **kernel seal breach** (`compile_fail` / private-field / visibility error at a sealed
   boundary) means the *design* is wrong — step back, re-derive the type boundary, or route through a sanctioned
   door (`docs/sanctioned_surface.md`), and **never** patch-append lifetimes, clones, or `unsafe` to force past
   a seal; a CPU-oracle **parity mismatch** is behavioral — debug oracle-first before touching the GPU leg.
4. **On an INSPECT:** it does not block, but it is **not done**. Attach a one-line justification and route it to
   the triage agent (§5); never silently merge a green-with-INSPECT PR.
5. **If you edit `allow/*.txt` or `scans.tsv`:** regenerate the digest (`bash scripts/ci/gen_digest.sh`) and
   commit it **in the same PR** — otherwise CI's `--check` FAILs.
6. **Do not merge before DA clearance** on any authority / gate / PROBATION rung. **Verify the tree, paste real
   output — never assert.**

> This is the binding floor for a coding rung; the handoff-template §H is the authority (do not restate or dilute
> it). The digest (step 1) + the inner loop (step 2) are what let you *skip the exploratory greps* — that is the
> token economy of the carrot.

---

## 8. Authoring a per-track CI addendum (opt-in; standards)

Most tracks need **none** — the global floor applies to every PR. A production track authors an addendum **only**
when its own anti-patterns keep reaching ESCALATE (triage-driven, never speculative). When it does:

- **Co-locate it with the track doc** — a sibling `<track>.ci.tsv` / `<track>.ci.allow/` (or a fenced block the
  track doc references). It travels with, and archives with, the track — no central registry.
- **Opt-in + auto-detach:** `doctrine_scan.sh --track-doc docs/<track>.md` loads the global floor **+ that
  track's addendum only**. No `--track-doc` = global floor only. When the track archives, its addendum stops
  applying automatically.
- **Additive-only:** an addendum may ADD a scan, TIGHTEN, or define a track digest surface. It may **never
  remove, loosen, or redefine a global scan-id** — that hard-FAILs.
- **Same rigor as the floor (§4):** every scan carries a `promotion-blocker`; every allowlist record matches its
  door-class grammar + carries a rationale; DA-reviewed like an allowlist edit.
- **Per-track digest:** `bash scripts/ci/gen_digest.sh --track-doc docs/<track>.md --output
  docs/tests/<track>_digest.md` (add `--check` to gate its freshness). The track digest = global surface + that
  track's addendum only.
- **Prove it:** `bash scripts/ci/doctrine_scan.sh --prove-addendum` asserts opt-in, auto-detach, additive-only
  rejection, and digest scope.

**Boundary:** an addendum extends *screening + the sanctioned surface* for one track — it is **not a code index
and must not grow into one** (§6). General code navigation stays the agent's own greps.

For webchat-driven executable proof and remote §1A triage commands, see §9.

---

## 9. Webchat orchestration with Track B executable proof

Track B adds non-blocking executable proof surfaces for the webchat orchestrator. Track A remains the blocking no-toolchain grep gate.

Use `/seal-proof` to initiate a GitHub-side CPU proof run. Use `/seal-proof plan [profile=<id>]` to print resolved commands without spending runner time. Use `/seal-proof profile=<id>` for a rung-class proof battery. Use `/seal-proof probe=<probe-id>` for known-bad guard-bite probes; a green known-bad probe is FAIL.

Owner edict on full batteries: Track B exists to avoid hygiene-theater test sweeps. Bare full-crate `cargo test -p <crate>` is forbidden in automatic PR-triggered, comment-triggered, and default doctrine-exec paths. Broad full-crate batteries are quarantined behind owner-deep `workflow_dispatch` only and must never be the default proof path for a small-edit handoff.

The orchestrator must reject any doctrine-exec report whose default or comment-triggered path ran a casual full-crate cargo test battery. Use plan mode to inspect commands before execution. Prefer exact targeted profiles and guard-bite probes. Full-cpu / owner-deep batteries are exceptional owner-dispatch artillery, not routine validation.

Track D note: owner-deep full batteries remain quarantined artillery. Smoke PASS is mechanics-only and not seal-proof. Seal-residue rungs still require targeted profile/probe proof. `DA-RULING: ADMISSION-BOUNDARY-COLLAPSE` supersedes the old same-family audit ledger: deletion authority is now `scripts/ci/test_pare_boundaries.tsv` plus `scripts/ci/test_pare_boundary_rows.tsv`, checked by `scripts/ci/test_pare_boundary_check.sh`. `TEST-ADMISSION-REGIME-0` makes this standing admission law: every KEEP inventory row names a permanent-residue class or promotion target, unledgered tests fail `test_inventory_drift_check.sh`, kernel/sim non-permanent KEEP rows fail, and `TEST-BUDGET` flags delta PRs that add more than three `#[test]` functions to one file without table-driven form. Until material reduction lands, weekly scheduled sentinel means sentinel-core only, and full quarantined battery remains workflow_dispatch-only. Do not implement scheduled workflow changes from Track D without an explicit cadence rung.

Use `/triage <scan-id> <delete|green|escalate> <reason>` to append a §1A row to `scripts/ci/triage_log.tsv` on the PR branch. Malformed commands must be rejected with the expected format. Commands are collaborator-only and accepted from issue comments and PR review/review-comment events. Never run untrusted fork code under a write token.

A doctrine-exec report is accepted only if it is fresh:

- `head_sha` equals the current PR head
- `base_sha` is recorded
- `tested_ref` is recorded
- `workflow_run_id` and `job_id` are recorded
- `merge_ref_status` is PASS, or UNAVAILABLE is treated as INSPECT for merge-sensitive rungs

The authoritative one-line verdict is:

`DOCTRINE-EXEC-VERDICT: PASS|FAIL|INSPECT ...`

`doctrine_exec_report.json` is a generated mirror of the same run, not a second truth. The sticky PR comment and job summary must agree. Labels are not verdicts and must not be used as proof.
