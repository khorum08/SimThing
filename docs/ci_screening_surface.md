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

## 0. The Rustification of SimThings — what this apparatus is and how to use it

The three 0.0.8.4.x tracks are one system: **the Rustification of SimThings** — the migration of every
project invariant to the highest rung of the admission ladder (core §1.2: **type boundary > admission
hard-error > guard scan > prose**), so that rigor is *enforced by construction* instead of held in an
agent's context window. Each pillar owns a rung-range:

| Pillar | What it rustified | Enforcement form |
|---|---|---|
| **0.0.8.4 Admission Substrate** (AS-1–8B) | column access, channel/index identity, kind-free tick view, `SimulationFabric`, `StructuralCoord`, `PackedUpload` | **rung 1 — types.** The illegal state does not compile; one `compile_fail` per promoted invariant is its whole proof. |
| **0.0.8.4.5 SimThing-Kernel** | the constitutional spine itself — "the sweep is the only authoritative path to mutate resolved state or emit a decision" | **rung 1 at architecture scale.** Sole owner of authoritative state, sole minter of effects; write/emission/participation seals; the cross-crate seal law; dependency-graph-enforced, zero-cost (ZST tokens, `#[repr(transparent)]`). |
| **0.0.8.4.6 CI Scaffolding** (Tracks A/B/C/D) | everything types cannot yet or can never reach | **rungs 2–3, mechanized.** Allowlist/blocklist scans, the self-testing scanner, test-admission law, the digest/inner-loop carrot, and the clearance ladder below. |

**How it is used — an admissions-based rigor and clearance system.** Nothing in this repo is *believed*;
everything is *admitted*. A change flows through composed admission gates, each cheaper and more total than
review: the **compiler** admits code (the seals make bypass uncompilable); **hydration/spec hard-errors**
admit content (malformed authoring dies at import with a span); the **CI scans** admit the surface (a clean
RELIABLE/allowlist verdict is DA-equivalent — the sanctioned surface is closed and unwidened); **test
admission law** admits proof (a test exists only if it names the regression nothing higher on the ladder
owns); and the **clearance ladder** admits the merge itself (SHA-bound verdicts → §1A triage →
orchestrator merge authority for precedented classes → DA graduation routed by declared risk → owner
supremacy above all). The scarce resources — DA judgment and owner attention — are spent exclusively on
the **residue** the gates cannot reach: sanctioned-door logic, live ontological conformance, taste. That
residue is a *named tripwire catalogue*, never a passive gap.

**The standing directives that keep it alive:**
- **Cite the gate, don't re-derive the rigor.** If an invariant is type-sealed, scanned, or
  admission-error'd, the gate is the proof — re-proving it in tests or prose is kabuki (§H rule 1).
- **New invariant → highest expressible rung**, and record why it could not climb higher.
- **The apparatus is designed to shrink.** Every scan carries a `promotion-blocker`; every surviving test
  carries a permanent-residue class or promotion target; promotion **retires** the lower-rung guard in the
  same PR. A growing guard count is a regression signal, not rigor.
- **The Necessity Test governs every test (2026-07-03; "one representative per boundary" is a retired fossil
  premise).** A test survives **only** if it catches a regression that neither the compiler / a type boundary,
  a production admission hard-error on a live path, nor an existing integration path already catches. If
  deleting it cannot break production and it is not a downstream dependency or required for canonical function,
  **delete it.** The per-boundary floor is **zero**, not one — a per-boundary "representative" for an invariant
  the substrate already enforces is a redundant witness, and curating one guaranteed the corpus never shrinks
  below (number of boundaries). Do **not** run representative-curation waves; run necessity-deletion waves.
- **Trust flows from admission, not authorship.** A green gate is trusted without re-verification
  (DA-equivalence); a relayed claim is not (verify the tree). This is why cheap agents can build the
  substrate safely: the gates, not the agents, carry the rigor.

## 1. Screening logic — how a change gets judged

Every `pull_request` and every `push` runs the **Doctrine Scan** GitHub Actions workflow on `ubuntu-latest`
(~1 min, free — public repo). The pipeline, in order:

```
checkout → ensure rg (preinstalled + apt fallback)
  → digest freshness (gen_digest.sh --check)  # stale sanctioned_surface.md hard-FAILs with regenerate remedy
  → orientation freshness (gen_orientation.sh --check)  # stale orchestrator_orientation.md hard-FAILs with regenerate/open remedy
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

**Delta vs whole-tree (binding).** Enforced by `doctrine_pr_scan.sh` / `doctrine_scan.sh`: HEURISTIC = PR diff;
RELIABLE = whole-tree. §1A spam bounds count branch-introduced INSPECTs via `inspect_spam_check.sh`.

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
| `gen_digest.sh` | regenerates / `--check`-verifies `docs/sanctioned_surface.md` (the global sanctioned-surface digest) against the live scans + allowlists — CI-enforced freshness |
| `gen_orientation.sh` | regenerates / `--check`-verifies `docs/orchestrator_orientation.md` from `active_track.txt`, the active design doc, clearance data, relay lint, and anchors; `--open <track.md>` is the local/operator entrypoint for setting or creating the active orchestration track. **Workplan classifier (binding):** `--open` accepts only production workplans under `docs/` that are *not* under `docs/tests/`, `docs/archive/`, or `docs/workshop/`, that parse ≥1 PR-ladder/rung table, and that carry workplan language (`production track` / `PR ladder` / `workplan` / `design track`). Evidence indexes and other non-workplan markdown hard-fail with `ORIENTATION-OPEN-VERDICT: FAIL(non-workplan)` *before* mutating `active_track.txt` or the orientation digest; an existing invalid `active_track` pointer likewise fails generate/`--check` as `FAIL(invalid-active-track: non-workplan)`. |
| `fixtures/` | known-bad inputs (one per RELIABLE scan) + false-positive traps + HEURISTIC production negative controls; `fixtures/README.md` maps fixture → scan → expected verdict |
| `.github/workflows/doctrine-scan.yml` | the authoritative gate (runs entirely on GitHub) |
### Test-corpus lifecycle & inventory tooling (the Rustified Test Lifecycle surface)

The test corpus is governed as data, not by ad-hoc judgment. These files + checks are the operator surface of the
Rustified Test Lifecycle (§11; CI-scaffolding design §4.1). Most checks are **ledger/text analysis only**; `track_closeout.sh --apply`
mutates and may run `cargo check -p <crate>` for `elevate-code`; GHA runs proof/guard modes unless a PR author has committed the apply result.

| File | Kind | What it holds / does |
|---|---|---|
| `test_inventory.tsv` | inventory ledger | one row per surviving test: `crate \| file \| test_name \| kind \| class \| superseding_boundary \| verdict \| note \| promotion_target \| birth_track \| dsu_survivals`. Every KEEP row must name a permanent-residue class or a promotion target |
| `test_residue_classes.tsv` | data list | the closed set of `permanent-residue:*` classes (`oracle-parity`, `golden-byte`, `seal-proof`, `determinism`, `behavior-regression`, `escaped-bug`, `doc-named-invariant`, `stead-required`, `dependency-floor`) |
| `test_lifecycle_tracks.tsv` | lifecycle ledger | `track_id \| status \| closed_at \| source \| note` — which birth tracks are open vs closed (a test whose birth track has closed is an expiry candidate) |
| `test_lifecycle_dsu_tiers.tsv` | policy ladder | downstream-utility renewal tiers keyed on `dsu_survivals`: `1–2` advisory-audit (PASS), `3–4` rejustify (INSPECT), `5+` presumed-stale (INSPECT — delete-or-promote unless DA affirmatively renews) |
| `track_closeout.sh` / `active_track.txt` / `closeout_autoclear.tsv` / `closeout_artifacts.tsv` | closeout substrate | owns closure: builds/applies manifests, checks freshness, stamps tracks closed, retires the orientation pointer when closing its active source/doc scope, leases artifacts, runs gates; autoclear names safe delete owners; artifact ledger carries the wall-clock reaper queue |
| `test_lifecycle_parked.tsv` / `test_lifecycle_parked_boundary.tsv` | parking ledgers | undecided rows parked out of live inventory/boundary tables with enough data for lossless restore or later decommission |
| `test_lifecycle_expiry_check.sh` | lifecycle tripwire | flags tests surviving past their birth-track closure and applies the DSU ladder. Modes: `--schema`, `--scheduled`, `--track-closeout <track_id>`, `--closure-gate <track_id>`, `--prove`. Emits `LIFECYCLE-EXPIRY-VERDICT: PASS\|INSPECT\|FAIL expired=N audit=N [max_dsu_survivals=N] mode=<mode>` |
| `test_inventory_check.sh` | inventory gate | validates the inventory schema + class/verdict grammar (allows the `dependency-floor` class for non-runnable helpers) |
| `test_inventory_drift_check.sh` | drift gate | the `TEST-INVENTORY-DRIFT` stock gate body: inventory must match discovered tests and every KEEP row must be owned; unledgered runnable tests FAIL. `permanent-residue:dependency-floor` rows are exempt from the stale-drift check only |
| `test_lifecycle_boundaries.tsv` / `test_lifecycle_boundary_rows.tsv` | boundary ledger | survivor boundary ownership (renamed from Track-D `test_pare_*` machinery at `CI-LIFECYCLE-RESIDUE-DELETE-0`) |
| `test_lifecycle_boundary_check.sh` | boundary gate | validates that each survivor KEEP row maps to an owned lifecycle boundary |

`TEST-INVENTORY-DRIFT` and `TEST-BUDGET` run as **stock gates inside `doctrine_scan.sh`**, enforcing inventory truth
and the ≤3-new-`#[test]` budget. `track_closeout.sh` owns closure; expiry checks remain operator/cadence diagnostics (§11).

### Track B executable-proof tooling (Track B DA-CLOSED 2026-07-04)

Track B is closed; both proof lanes landed. These are its scripts/data, surfaced here for completeness (operator
quick-reference for the GitHub/webchat lane is §9; the owner-local lane is the citation contract in §9).

| File | Lane | Role |
|---|---|---|
| `doctrine_tests.sh` | owner-local | GPU/Bevy/desktop executable harness; `--list` / `--plan` / `--profile <id>` / `--prove-report`; emits `DOCTRINE-TESTS-VERDICT` + `--- tripwire-tags ---`; skipped/unverified → INSPECT, never a silent PASS; refuses GHA execution |
| `doctrine_tests_profiles.tsv` | owner-local | resolves owner-local profiles (e.g. `owner-local-gpu-bevy`) from the live inventory |
| `doctrine_exec.sh` / `doctrine_exec_plan.sh` / `doctrine_exec_probes.sh` | GitHub CPU | the non-blocking CPU proof engine, plan-mode resolver, and known-bad guard-bite probes |
| `doctrine_exec_stale_check.sh` | GitHub CPU | rejects a report whose `head_sha` ≠ current PR head (the verify-the-tree rule, mechanized) |
| `doctrine_exec_comment.sh` | GitHub CPU | one sticky PR comment (`<!-- doctrine-exec-sticky -->`) carrying the verdict footer |
| `doctrine_exec_commands.sh` / `doctrine_exec_triage.sh` | GitHub CPU | the `/seal-proof` + `/triage` command handlers (collaborator-only; `/triage` commits a §1A row to `triage_log.tsv`) |
| `doctrine_exec_profiles.tsv` / `doctrine_exec_profile_lint.sh` | GitHub CPU | the profile taxonomy (`smoke\|targeted\|probe\|owner-deep`) + the lint that forbids casual full-crate `cargo test`, `test-pare-*` IDs, and enforces the GHA proof seal |
| `doctrine_exec_gha_proof_seal.sh` | GitHub CPU | proves owner-local-only commands never appear in a GHA profile (GPU/Bevy/desktop stay off the runner) |
| `doctrine_surface_truth.sh` (+ `_inspect` / `_reason_test`) | GitHub CPU | `cargo public-api` differential of `simthing-kernel` vs `kernel_public_api_baseline.txt` — divergence → `SURFACE-TRUTH: INSPECT` |
| `mapeditor_linux_cargo_check.sh` | owner-local | Studio/`simthing-mapeditor` compile-floor helper |

Workflows: `.github/workflows/doctrine-exec.yml` (`CI-B-GH-CPU-0`) + `.github/workflows/doctrine-exec-commands.yml`
(`CI-B-GH-COMMENT-0` / `CI-B-GH-TRIAGE-0`), both **separate and non-blocking** — the blocking Track A gate is untouched.

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
- **`SPEC-LOWERER-KIND-READ` (CI-SCAN-SPEC-KIND-COVERAGE-0).** HEURISTIC tripwire over
  `crates/simthing-{spec,clausething}/src/**` for drift-shaped `.kind` branching (`match .*\.kind`,
  `.kind ==/!=`, parameterized `match kind { … SimThingKind:: … }`). **Not** a ban on all spec/lowering kind
  reads — legitimate role-resolution may remain (`role-resolution-exclude-site` marker + triage exclude list).
  Closed-lowerer hits carry **higher suspicion** (lowerers are constitutionally closed unless a DA amendment
  names them). Delta-scoped in PR CI; whole-tree baseline is triage backlog only. Promotion blocker: retire when
  spec-layer role resolution is role-keyed by `SubFieldRole`/column admission boundaries, not `SimThingKind`
  branching. Scenario-born candidate engine-shaped code routes to **`simthing-workshop`** (§12), not spec/lowerers.

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

Four fences (mechanized): `triage_log_check.sh` reason strictness; `clearance_check.sh` check 7;
`inspect_spam_check.sh` hill-climbing bounds; `triage_log.tsv` telemetry. Judgment-residue dispositions
(DELETE/GREEN/ESCALATE) and DA spot-audit remain orchestrator/DA practice — see §9 for `/triage`.

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
| `none` + green RELIABLE + no escalation | **light** | confirm named deliverables (relay+CI OK unless load-bearing); graduate |
| `semantic` | targeted | judgment check; tree dig if code-facing / elevatable |
| `data-deliverable` / `gate-wiring` | **deep** | byte-faithfulness on the tree; *prove the guard bites* (perturb → FAIL) |
| `seal-residue` / `allowlist-edit` | **deep** | tree / legitimacy audit (door-class, no laundering) |
| triage `ESCALATE` present | **deep** | the escalation was already headed to the DA |

**Verify-the-tree is weighted** (`agent_onboarding` DA): **require** for code-facing / long-lifecycle / horizontally impactful load-bearing; **relax** for pure policy, stamps, light residual. Advisor: `da_treeverify.sh` + `da_review_profile.tsv` (advisory profile only; non-core rows expire/delete). Falsification check names *where* to spend.

---

## 5A. Orchestrator guidance — the operational contract (constitution §0.9.7 is the authority)

> **Cold-start:** run `bash scripts/ci/orient.sh --role=coding|orchestrator|da` and read generated
> `docs/orchestrator_orientation.md` with ORIENT-RECEIPT — not a static reading list. Freshness:
> `gen_orientation.sh --check`.

**Mechanized responsibilities (enforcing surfaces — do not re-derive from prose):**
1. **Triage-log stewardship** — `clearance_check.sh` check 7 + `triage_log_check.sh` + `/triage`
   (`doctrine_exec_triage.sh`) → `triage_log.tsv`.
2. **Closure hygiene** — birth-track-scoped edit authorization; spent wave replay deleted.
3. **Proof binding** — `relay_lint.sh` tested-code-SHA + `LIVE-POINTER`; design §6 sunset ledger.
4. **Graduation** — `relay_lint.sh` graduation-routing block; never self-mark COMPLETE.

**Handoffs:** `relay_lint.sh` + `handoff_template.md`; §H violations rejected at review.

**Merge authority (judgment-residue — constitution §0.9.7 governs):** precedented-class clearance is
mechanical via `clearance_check.sh` checks 1–8 (`precedented_classes.tsv`). Empty-class split:
`admitted-scope-router-gap` (admitted envelope + proof, missing class — class-harden) vs narrowed
`unclassified-scope`; novelty claim-only; workshop-TP vs admitted ClauseScript API classes.
DA spot-audits self-authorized merges; one wrong self-merge suspends authority. Owner supremacy above all.

**Channeling DA token spend (judgment-residue — feed the routing table honestly):**
- **Declare risk classes truthfully and completely** — under-declaring to earn a light review is the
  laundering move the spot-audit exists to catch; over-declaring burns the DA turn the regime exists to save.
- **Write the Falsification check as an executable instruction** ("run X, expect Y; perturb Z, expect FAIL")
  — the DA should be able to spend tokens exactly there and nowhere else.
- **Batch escalations** per review cycle; lead every relay with the verdict-relevant facts (what changed,
  what proves it, what the DA must decide); never bury a HOLD-worthy fact mid-report.
- **Never relay an unverified claim as fact** — verify against the tree first, or mark it explicitly
  `unverified`. The DA reconstructing truth from git because a relay obscured it costs more than the
  review it replaced.

**GHA-side commands:** `doctrine-exec-commands.yml` — `/triage` (`doctrine_exec_triage.sh` +
`triage_log_check.sh`), `/seal-proof`, collaborator-only. Scan picture from checks UI + INSPECT lines +
`/triage`; descoped scan sticky comment is not a proof gate (§9).

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

Track B is **DA-CLOSED (2026-07-04)** with both lanes landed and reconciled: the GitHub-side non-blocking CPU executable-proof surfaces for the webchat orchestrator (below), and the owner-local GPU/Bevy/desktop executable-proof lane (owner-local PASS citation rule above). Track A remains the blocking no-toolchain grep gate.

Use `/seal-proof` to initiate a GitHub-side CPU proof run. Use `/seal-proof plan [profile=<id>]` to print resolved commands without spending runner time. Use `/seal-proof profile=<id>` for a rung-class proof battery. Use `/seal-proof probe=<probe-id>` for known-bad guard-bite probes; a green known-bad probe is FAIL.

**Owner-local PASS is citable proof (`CI-B-CLOSEOUT-0`, Track B closed 2026-07-04).** GPU/Bevy/Studio-mapeditor/desktop-typeface/real-adapter-parity proof is owner-local-only — GitHub must not run those binaries or install desktop/GPU deps. When a GHA check, Doctrine Exec report, orchestrator, or DA review needs proof of an owner-local-only class, a **fresh** owner-local `DOCTRINE-TESTS-VERDICT: PASS` report (from `scripts/ci/doctrine_tests.sh`) is citable validation **iff** it matches the current PR `head_sha`, names the tested profile, carries `owner_local=true`, preserves the strict footer, and emits the relevant PASS tripwire tag (`COMPILE_FAIL_PROVEN` / `PARITY_BIT_EXACT` / `OWNER_LOCAL_PASS`). `INSPECT` is not validation; stale or `head_sha`-mismatched reports are rejected; and citing owner-local PASS never licenses GHA-side execution of GPU/Bevy/Desktop probes. GitHub-side CPU Doctrine Exec remains the citable path for CPU proof classes; the two lanes never merge.

Owner edict on full batteries: Track B exists to avoid hygiene-theater test sweeps. Bare full-crate `cargo test -p <crate>` is forbidden in automatic PR-triggered, comment-triggered, and default doctrine-exec paths. Broad full-crate batteries are quarantined behind owner-deep `workflow_dispatch` only and must never be the default proof path for a small-edit handoff.

The orchestrator must reject any doctrine-exec report whose default or comment-triggered path ran a casual full-crate cargo test battery. Use plan mode to inspect commands before execution. Prefer exact targeted profiles and guard-bite probes. Full-cpu / owner-deep batteries are exceptional owner-dispatch artillery, not routine validation.

**Executable Doctrine Exec profiles (`CI-PROOF-PROFILE-TAXONOMY-0`):** Track-D `test-pare-*` / `test-deletion-*` profiles are retired from `scripts/ci/doctrine_exec_profiles.tsv`. Executable profiles must reference current proof surfaces only — not historical deletion batteries. `doctrine_exec_profile_lint.sh` forbids `test-pare-*` profile IDs and `test-deletion-*` risk classes. Closure-certificate `cargo test --workspace --all-targets` is not a profile-default proof path.

Track D note: owner-deep full batteries remain quarantined artillery. Smoke PASS is mechanics-only and not seal-proof. Seal-residue rungs still require targeted profile/probe proof. Survivor boundary ownership is enforced by `scripts/ci/test_lifecycle_boundaries.tsv` plus `scripts/ci/test_lifecycle_boundary_rows.tsv`, checked by `scripts/ci/test_lifecycle_boundary_check.sh` (renamed from Track-D `test_pare_*` machinery at `CI-LIFECYCLE-RESIDUE-DELETE-0`; historical `test_pare_audit.tsv` deleted). **The Necessity Test (2026-07-03) supersedes "one representative per boundary," which is retired as a fossil premise:** a test is admissible ONLY if it catches a regression that neither the compiler/a type boundary, a production admission hard-error on a live path, nor an existing integration path already catches. If deleting it cannot break production and it is not a downstream dependency or required for canonical function, **delete it** — the per-boundary floor is **zero**, not one, for invariants the substrate already enforces. Keeping one per boundary guaranteed the corpus never shrinks below (number of boundaries) regardless of redundancy; that is the compromise that stalled paring, now removed. `TEST-ADMISSION-REGIME-0` makes this standing admission law: every KEEP inventory row names a permanent-residue class or promotion target, unledgered tests fail `test_inventory_drift_check.sh`, kernel/sim non-permanent KEEP rows fail, and `TEST-BUDGET` flags delta PRs that add more than three `#[test]` functions to one file without table-driven form. Until material reduction lands, weekly scheduled sentinel means sentinel-core only, and full quarantined battery remains workflow_dispatch-only. Do not implement scheduled workflow changes from Track D without an explicit cadence rung. **Track D is CLOSED (2026-07-04, `TRACK-D-CLOSEOUT-0`):** `TEST-NECESSITY-SWEEP-0` (merged #1122 @ `3ef232506f`) deleted 3,478 tests in one default-DELETE pass, inventory 4,070 → 731 (592 explicit keep + 137 `cfg_test_mod` markers + 2 non-runnable `dependency-floor` helpers), binding proof `cargo check --workspace --all-targets` PASS (DA-re-run). The **Rustified Test Lifecycle** (CI-scaffolding design §4.1) is now standing law: every test is **assumed DELETED at its birth track's closure** unless it (a) carries a canonical notion — then promote it into a `simthing-kernel` type/seal or an EML opcode-stack construct and delete the test, (b) is a `TIER7` terminal proof class with a `catches:` note, or (c) is a non-runnable `dependency-floor` helper. This is how the corpus can never re-propagate. **Closure-certificate exception to the full-battery ban:** a track/PR-ladder closeout may run `cargo test --workspace --all-targets` **once** as a closure certificate (or to satisfy a deferred DA review) — it certifies the survivor set as a whole and is never a routine or comment-triggered validation path; the `dependency-floor` residue class (`permanent-residue:dependency-floor`) is exempt from the stale-drift check only, never from the unledgered-runnable-test check.

Use `/triage <scan-id> <delete|green|escalate> <reason>` to append a §1A row to `scripts/ci/triage_log.tsv` on the PR branch. Malformed commands must be rejected with the expected format. Commands are collaborator-only and accepted from issue comments and PR review/review-comment events. Never run untrusted fork code under a write token.

A doctrine-exec report is accepted only if it is fresh:

- `head_sha` equals the current PR head
- `base_sha` is recorded
- `tested_ref` is recorded
- `workflow_run_id` and `job_id` are recorded
- `merge_ref_status` is PASS, or UNAVAILABLE is treated as INSPECT for merge-sensitive rungs

The authoritative one-line verdict is:

`DOCTRINE-EXEC-VERDICT: PASS|FAIL|INSPECT ...`

---

## 10. Deletion never licenses a desktop/GPU probe (owner ruling, 2026-07-03)

**The insight (owner):** the *only* reason to run a Linux-side desktop/GPU/Bevy binary in CI is to check it
still passes — and a test you are **deleting** never needs to pass anywhere. Therefore a deletion decision
**never** justifies installing ALSA / X / Wayland / winit / wgpu / mapeditor / typeface dependencies or
`apt-get` on a GHA runner. Any such probe during a paring wave is a confused instinct to "verify before
deleting," and it is forbidden. (This is what produced the invalid `simthing-driver`/`alsa-sys` probe;
`TESTS-COMPILE-FLOOR-NON-BEVY-0`'s forbidden-token lint now blocks it structurally, across the `tests`,
`doc_tests`, **and `crate_checks`** columns — the last was the smuggling lane 0R2 closed.)

**The doctrine — how a deletion is proven (all platform-portable; none requires the deleted thing to run):**
1. **Coverage map** — the surface the deleted test claimed is owned by a *compiling* representative
   (kernel-internal preferred). Platform-independent: it is a fact about the corpus, not an execution.
2. **Compile floor** — the surviving code still compiles (`cargo check -p --tests`, the standing GHA floor
   for non-Bevy crates; owner-deep local for Bevy/desktop crates). This is the only "does it build" check a
   deletion needs, and it never runs the deleted binary.
3. **Owner's local run (Windows) is authoritative for the delete decision.** If the owner's local machine
   flags a test as fossil/redundant/dead, that determination is **sufficient** — there is **no** obligation
   to re-verify it with a Linux-side run, and for a desktop/GPU-linked test such re-verification is exactly
   the forbidden probe. Local-flags-for-deletion → delete; do not escalate to a Linux execution to "confirm."

**Corollary:** a non-compiling or platform-unavailable binary is a *stronger* delete signal, never a
preservation reason (this extends `OWNER-DEEP-RESIDUE-PARE`: "a stale test binary failing to compile is not
a reason to preserve it"). The desktop/GPU dependency graph belongs to owner-deep local execution only; the
non-owner-deep GHA floor proves *compilation of the survivors*, never *execution of the departed*.

`doctrine_exec_report.json` is a generated mirror of the same run, not a second truth. The sticky PR comment and job summary must agree. Labels are not verdicts and must not be used as proof.

---

## 11. Test-corpus lifecycle tooling cycle (Rustified Test Lifecycle — operator surface)

The doctrine is in the CI-scaffolding design §4.1; this is the **operator walk-through** of the tooling that
enforces it. The governing law (see §9 Track D note): **every test is assumed DELETED at its birth track's
closure** unless it (a) carries a canonical notion — then promote it into a `simthing-kernel` type/seal or an EML
opcode-stack construct and delete the test; (b) is a `TIER7` terminal-proof / permanent-residue class with a
`catches:` note; or (c) is a non-runnable `dependency-floor` helper. All lifecycle tooling is **ledger/text
analysis only — no toolchain, no build, no cargo** — so it is safe to run anywhere, including inside a docs rung.

**The cycle:**

1. **Birth.** A new test is added with its inventory row in `test_inventory.tsv`, naming its `class`, its
   `birth_track`, a `promotion_target` (or a permanent-residue class), and `dsu_survivals=0`. `test_inventory_check.sh`
   validates the schema; the `TEST-INVENTORY-DRIFT` stock gate (in `doctrine_scan.sh`) fails any unledgered runnable
   test on the PR, and `TEST-BUDGET` flags a delta adding more than three `#[test]` fns to one file without
   table-driven form. This is the blocking floor — it runs on every PR.

2. **Track closure → scripted closeout.** Use `track_closeout.sh`: build manifest, resolve every
   test/doc disposition, `--check-eval`, then `--apply`. Apply flips the track to `closed`, mutates
   inventory/boundary rows in lockstep, leases artifacts, and runs gates. Same-PR closeout deletions
   require both report and manifest; closeout-substrate changes also need a disposable two-row sample.

```bash
bash scripts/ci/track_closeout.sh --build-manifest <workplan|--track <track_id>> [--docs <glob>]...
bash scripts/ci/track_closeout.sh --check-eval <manifest>
bash scripts/ci/track_closeout.sh --apply <manifest>
```

   The expiry commands emit `LIFECYCLE-EXPIRY-VERDICT: PASS|INSPECT|FAIL expired=N audit=N [max_dsu_survivals=N] mode=<mode>`.
   A test whose birth track has closed and which is **not** promoted, permanent-residue, or a dependency-floor
   helper is an **expired candidate** → delete-or-promote.

3. **Downstream-utility renewal ladder.** A test that legitimately outlives its birth track earns a
   `dsu_survivals` increment each time it is affirmatively renewed. `test_lifecycle_dsu_tiers.tsv` sets the
   escalating burden: `1–2` = advisory-audit (PASS, renewal burden begins); `3–4` = rejustify (INSPECT — must
   re-justify with a fresh, named, verified downstream consumer); `5+` = presumed-stale (INSPECT — mandatory
   delete-or-promote candidate unless the DA affirmatively renews). **Promotion pressure is deliberate:** the
   sanctioned exit from rising DSU debt is promotion into a kernel type / EML construct, **not** perpetual renewal.

4. **Triage every INSPECT.** Lifecycle INSPECTs route through the §1A loop like any other: the orchestrator lands
   a `delete/green/escalate` row via `/triage` (§5A) into `triage_log.tsv`. An unlogged clearance is invisible and
   therefore did not happen.

5. **Boundary ownership.** `test_lifecycle_boundary_check.sh` (over `test_lifecycle_boundaries.tsv` +
   `test_lifecycle_boundary_rows.tsv`) validates that each survivor KEEP row maps to an owned lifecycle boundary —
   the successor to the retired Track-D `test_pare_*` machinery.

**Wiring status:** the inventory/budget gates are already blocking (inside `doctrine_scan.sh`). The
`test_lifecycle_expiry_check.sh` tripwire is **operator/cadence-run, not yet a workflow** — a scheduled expiry
workflow requires an explicit cadence rung (do not add scheduled workflow changes without one). Until a material
reduction cadence lands, the expiry sweep is an orchestrator closeout duty, not an automated gate.

## 12. Workshop is the scenario candidate-code sandbox (owner ruling, 2026-07-04)

Feature-proofing *scenario* tracks (e.g. 0.0.8.5 Terran-Pirate) are exploratory expeditions that surface
consumer-driven capability needs. Their candidate code (services/structs/fns/heuristics beyond authored data)
lives in **`simthing-workshop`** — never in a sealed engine crate. **Containment** (workshop code can't leak
*up*) is structural: `simthing-workshop` is a **verified leaf** (nothing depends on it), so the seal law makes
game-semantic candidate code there unable to leak upward by linkage, and workshop is already outside every
fence-scan target. **Homing** (new scenario code must be *written into* workshop, not into a sealed crate) is
**not** structural — the arrow does not fence `simthing-clausething`/`spec` — so it is enforced by
classify-before-merge plus the scan tripwires below. See `design_0_0_8_5…§0A.1` for the binding statement; this
section is the operator surface.

- **The exit is re-fenced.** Elevation = moving code `simthing-workshop` → an engine crate. The elevation PR's
  diff **re-applies the full engine-crate scan battery to the outbound hunk** (which now covers `simthing-spec`
  + lowerers — `CI-SCAN-SPEC-KIND-COVERAGE-0`): it must be generic-namespaced and game-semantic-free, or it does
  not climb. The fence isn't removed by living in workshop; it is relocated to workshop's *exit*.
- **Default-delete at closeout, no registry.** Scenario candidate code is expirable by default — deleted at
  track close via the existing lifecycle expiry sweep (orchestrator closeout duty, §11). Keeping a candidate is
  an explicit move into standing workshop code; there is **no registry and no lease** (the envelope's
  default-delete disposition already expresses each asset's value). **Do not add a registry.**
- **The Homing Boundary — classify before merge.** The classifier for any engine-crate addition in a scenario
  PR: *"would this code exist if this scenario didn't?"* If **no** → scenario candidate code → `simthing-workshop`.
  If **yes** — a genuinely generic, semantic-free ClauseScript language/lowering surface any scenario would want
  (e.g. extending a generic decoder family with a new generic form) — an engine crate is fine. **Not** allowed
  in a sealed crate: any scenario-specific service/struct/fn/heuristic (HP/Damage resolver, fleet-contact logic,
  owner-bonus combat helper, zero-HP removal, RF-child-depth workaround, Terran/Pirate/Fleet/Cohort branching).
  *"Generic lowering, as prior TP rungs did it"* is **not** a licence — prior rungs predate this doctrine.
- **Substrate widening is DA-authorized only — not an agent's escape hatch.** Genuine generic substrate widening
  (a reusable, semantic-free API a crate needs) is admissible as future utility, but the route flows **top-down
  from DA/Owner approval**. An agent may **propose/appeal** it to the orchestrator when it is the most
  performant/logical path (*surface, never self-grant*, core §1.2.1). "Future utility" / "downstream value" is a
  **request the DA adjudicates**, never a self-issued verdict — it is nearly unfalsifiable and self-serving, so
  the burden is on the appeal and the **default is deny → workshop-home it**. A self-classified "generic
  widening" landed in an engine crate is drift, rejected at review.
- **Orchestrator note (judgment-residue).** Classify-before-merge per symbol; widening requires DA/Owner approval.
  Tripwire: `SPEC-LOWERER-KIND-READ` in `scans.tsv` (kind-branching); non-kind residue is live review control.

> **Deferred elaboration (not in force).** A per-production `simthing-workshop/src/testthing/<production>/`
> sub-taxonomy with a scan carve-out and a mechanical `--track-closeout` emptiness gate is the natural next step
> *when* workshop fills and needs per-expedition sub-organization. Deferred — until then, "candidate code in the
> `simthing-workshop` leaf, default-delete at closeout" is the whole mechanism. Do not scaffold `testthing/` yet.
