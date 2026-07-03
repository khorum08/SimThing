# 0.0.8.4.6 — CI Scaffolding: the Doctrinal Tripwire Layer

> **Status: Tracks A + C DA-CLOSED (2026-07-01); Track B OPEN (DA-OPENED 2026-07-02, consumer named — §3).** An infrastructure sub-track in the
> 0.0.8.4.x lane (after the closed `simthing-kernel` track 0.0.8.4.5), sequenced **before** the 0.0.8.5
> Terran-Pirate track. *(Owner said "0.0.8.5"; that number is held by Terran-Pirate, so this is numbered
> 0.0.8.4.6 to avoid collision — bump TP to 0.0.8.6 and renumber this to 0.0.8.5 on owner request.)*
>
> **Purpose.** Automate the repeated DA doctrinal scans into a free, fast, public-repo GitHub Actions layer
> so that **agents and the orchestrator may treat a clean CI result as "the DA ran these scans"** — and a
> flagged result routes to DA for the judgment a grep cannot make. The executable tests (builds, GPU parity,
> Studio) stay **local**, by architecture. Completed CI scaffolding now has Track A closed as the grep-only
> tripwire layer and Track C closed as the generation-time constraint layer; Track B is now OPEN as the
> local executable harness for sanctioned-door logic proof (DA-OPENED 2026-07-02; consumer: the 0.0.8.5
> seal-residue door-logic rungs — §3).

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable; hold every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2 / §1.2.1** — doctrine-as-type, the admission ladder, residue-as-tripwire (this track automates the *guard-scan* rung of that ladder).
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) §0 (esp. **§0.9** — doctrine-as-type / residue-as-tripwire carry-forward; **§0.6.6** — no inert scaffolding).
3. **This file** — the 0.0.8.4.6 canonical design file.
4. [`design_0_0_8_4_5_simthing_kernel.md`](design_0_0_8_4_5_simthing_kernel.md) **§5.2** — the B1–B8 bypass-state catalogue this layer screens for.
5. [`handoff_template.md`](handoff_template.md) — the spine + the `seal-residue-risk` field (CI surfaces the same residue classes).

**Established decisions — do NOT re-derive:**

- **Allowlist, not blocklist — the rigor that makes a clean scan *trusted*.** A *blocklist* scan ("find `for_boundary_delivery`") misses novel holes, so it needs a DA to catch the unknowns — which burns the very tokens this layer exists to save. An **allowlist** scan inverts it: *enumerate the sanctioned surface; everything outside it FAILS.* This catches **novel and subtle violations automatically** (a `#[doc(hidden)] pub fn -> Self` minter fails an allowlist scan with no reasoning), so a clean RELIABLE result is **trusted without DA re-verification.** The seal scans are written as allowlists (§5); the DA's residual is not "re-verify the grep" but "approve an allowlist *edit*" (a deliberate, greppable, infrequent review) + the genuinely-semantic logic in Track B.
- **CI screens; one narrow thing it still cannot prove.** With allowlist scans, the "doesn't prove a seal" residue **shrinks to one thing:** whether a *sanctioned door's internal logic is correct* (e.g. does `apply_candidate_f_exact_magnitude` actually compute the exact magnitude and write the right cell). That is `compile_fail`/parity territory (Track B), genuinely not greppable — but it is a *different, smaller, infrequent* activity than re-checking scans, and it is **not** triggered on every PR (only rungs that change a sanctioned door's logic, flagged by `seal-residue-risk`). A green allowlist-CI means *the sanctioned surface is closed and unwidened* — which is most of "the seal holds," not merely "no known-bad pattern."
- **Three verdicts, never two.** Every scan emits `PASS` / `FAIL` (hard, blocks the PR) / `INSPECT` (soft — a possible false-positive or an inherently-judgment scan; surfaces to DA, does **not** block). This is residue-as-tripwire applied to the scanner: an uncertain result is a flagged tripwire, not a silent pass or a false failure.
- **Heuristics + allowlists are auditable DATA, not shell logic (owner requirement).** The scan definitions and the allowlists live as **data in one known directory** — `scripts/ci/scans.tsv` (one line per scan: `id | severity | target-glob | pattern | exclude | doctrine-ref`), `scripts/ci/allow/*.txt` (one allowlist entry per line), and `scripts/ci/README.md` (the map + how to add/edit). They are **auditable** (git diff shows every change), **accessible** (one documented path), and **trivially modifiable** (a new scan or allowlist entry is one new line — you never touch the engine). `scripts/ci/doctrine_scan.sh` is a *thin, stable runner* that reads the data and emits the report; it changes only when the report *format* changes, never when the project's invariants shift. This is doctrine-as-data: the heuristics are data over a fixed engine, run identically locally and in CI.
- **The scanner must self-validate or it rots.** A scan that silently stops catching its violation is worse than no scan (false confidence — the `deny.toml` lesson, §0.6.6). The layer carries a fixture battery proving each scan still fires on a known-bad input and stays quiet on the known false-positive traps.
- **Public repo = free + unlimited standard runners.** Linux only; no toolchain, no build, no GPU, no Bevy — pure text. Wall-clock seconds.
- **Execution environment — the authoritative scan runs on GitHub, never on the owner's Windows box.** The owner works on **Windows 11 / PowerShell, not Linux**. The authoritative Track A scan + self-test (including `rg -U` multiline matching) run on `ubuntu-latest`, where ripgrep is preinstalled — so the owner is **never required to run `rg`, `bash`, or any scan locally**; a clean CI verdict is the gate. Local execution is strictly an **opt-in** convenience (the pre-push hook), and opting out costs nothing. The validation battery's "runs locally" means the *implementer's* dev-time proof while building Track A, plus the CI self-test — **not** a recurring owner task. (If the owner ever opts into the local hook, it needs ripgrep on Windows via Git Bash — `winget install BurntSushi.ripgrep`; otherwise no local tooling is needed at all.) Any `rg`-dependent or bash-dependent logic therefore lives on the GitHub side by construction.
- **Patterns are line-split-robust — and we *prove* it, not assume it.** A single-line regex misses a declaration split across lines (`pub fn …` on one line, `-> ThresholdEvent` on the next) — and for an *allowlist* scan that miss is a **false NEGATIVE: a forge slips through**, the single worst failure this layer can have. The in-scope fix is **multiline-capable matching** (`rg -U`/`--multiline` + whitespace-tolerant patterns), *not* a `cargo fmt --check` gate — fmt would drag a Rust toolchain into Track A and break the no-toolchain/free/instant property that is the whole point. Robustness is proven by a **split-declaration trap fixture** (§ fixtures): a sealed-producer with its return type on the next line, which the allowlist scan **must still catch**. If grep ever can't be made robust to a real split, that scan is downgraded to `INSPECT`, never left as a silent RELIABLE pass.
- **Success is measured in scans *deleted*, not scans owned (the admission ladder applied to the scanner itself).** A grep scan is rung 3 of the ladder (source scan); it exists *only because* the invariant isn't yet a type boundary (rung 1). So every RELIABLE scan carries a **`promotion-blocker`** field in `scans.tsv` — one line naming the type-boundary that would make it redundant (e.g. *"retire when `ColumnIndex` is a kernel-wide newtype"*). When the 0.0.8.4-lane work promotes that invariant to a type, **the same PR deletes the scan** — and `CI-A-CLOSEOUT-0`'s contract makes that mandatory, not optional. A RELIABLE scan with an *empty* promotion-blocker is a flagged anomaly (why is this prose-guarded and not a type?). This is the one defense against Grok's central risk — a scaffold that accretes and rots: the layer is **designed to shrink**, and a growing scan count is a regression signal, not progress.
- **The allowlist is a *typed admission record*, not a babysat list — the rustified onboarding heuristic.** Each `allow/*.txt` entry is structured data the scanner *enforces*: `symbol \| door-class \| rationale \| promotion-blocker`. `door-class` is one of the sanctioned grammars (`read` / `dispatch` / `apply` / `cpu_oracle` / `inert-util`); **a symbol whose name doesn't match its declared door-class grammar FAILS**, and **a line missing any field FAILS.** So a future producer/consumer onboards exactly one way: conform to a sanctioned door shape *and* file a one-line rationale + retirement condition. Casual cruft is structurally rejected (you cannot file `forge_x` under `read`, and you cannot add a bare symbol with no justification); a genuine new feature is *accommodated* by a conforming, self-describing entry. The rationale lives **in the data** (auditable in the diff), which replaces "remember to justify it in the PR description" (process ceremony) with an admission gate (enforced format). This keeps the allowlist narrow by construction while staying open to legitimate growth — the regime adapts without widening loosely.
- **INSPECT is scarce by design — a chronically-firing HEURISTIC is a retirement target, not furniture.** Grok's INSPECT-fatigue risk is real: if most PRs draw INSPECT flags, the DA learns to ignore them and the tripwire dies. Defense: HEURISTIC scans are *few and budgeted*. A HEURISTIC scan that fires repeatedly without ever surfacing a real finding is either **promoted** (made RELIABLE/allowlist if it can be) or **deleted** (same ladder logic) — never left to erode the signal. An INSPECT flag must stay rare enough that it always means "actually look."
- **INSPECT routes through a free triage tier before the scarce DA — and the corpus is maintained on a cadence (§1A).** INSPECT is a **cost-asymmetry loophole**: tripping a HEURISTIC costs an agent nothing (one retry), clearing it costs the DA real judgment, and a capable agent hill-climbing away from a hard FAIL converges on a HEURISTIC's false-positive zone by retry-pressure alone. Left unspecified, a green-with-INSPECT PR is *either* a silent backdoor *or* a DA-token drain — both lose. So INSPECT routes first to a **free/unmetered triage agent** (the orchestrator tier) that resolves it (delete / green / escalate) under a bounded loop + spam-detection + DA spot-audit; only the residue reaches the DA. The triage log is, for free, the per-scan-id telemetry that turns the retirement ladder above from aspiration into **periodic corpus maintenance** — a standing cadence where the DA prunes chronically-firing HEURISTICs, retires promoted scans, and trims stale allowlist entries. Full protocol in §1A.
- **HEURISTIC scans evaluate on the PR *delta*, not the whole-tree baseline (or the triage tier drowns).** A whole-tree scan re-flags every pre-existing legitimate semantic word / `.kind` read on *every* PR — the `CI-A-SCAN-DEFS-0` positive control already shows ~81 such baseline INSPECTs (73 `SEMANTIC-WORDS` + 8 `SIM-KIND-READ`). Re-surfacing those per-PR would swamp the §1A triage tier and trip its spam-bounds on every PR. So in CI, **HEURISTIC scans flag only occurrences introduced/touched by the PR's diff**; **RELIABLE scans stay whole-tree** (you want zero of those anywhere, always); and the §1A spam-bounds count **branch-introduced (delta)** INSPECTs, never baseline. HEURISTIC targets additionally exclude `tests/` / `#[cfg(test)]` and non-code lines so even the baseline stays small. The whole-tree scan remains the *seeding/positive-control* check (zero hard FAILs on master); per-PR evaluation is delta-scoped. This is a `CI-A-WORKFLOW-0` design constraint and a `scans.tsv` tuning constraint, settled now so it isn't relitigated.

---

## 1. The DA-equivalence + tripwire contract (binding for both tracks)

The whole point is *transparency*: the process continues to feel as if the DA ran the scans. That requires a
precise contract, not just a passing check.

**The doctrine-scan report** (the DA-equivalent artifact both tracks emit):

```
DOCTRINE SCAN REPORT  (commit <sha>, <timestamp>)
  scanner self-test: PASS|FAIL          # fixtures: every hard scan fired on its known-bad; no trap fired
  --- results ---
  <scan-id>  PASS|FAIL|INSPECT  <count>  <doctrine-ref>   [paths…]
  …
  --- summary ---
  hard failures: N   inspect flags: M   reliability: <legend>
  DOCTRINE-SCAN-VERDICT: PASS|FAIL|INSPECT  failures=N inspect=M selftest=PASS|FAIL
```

The final `DOCTRINE-SCAN-VERDICT:` line is a **stable machine-parseable footer** the orchestrator greps for one verdict + counts (no JSON parser, no dashboard — the minimal version of "machine-consumable"). The full report stays the human/DA artifact.

- **PASS** — clean. The orchestrator treats this scan as "DA ran it, no finding." For a **RELIABLE** scan (especially the allowlist scans), PASS is **trusted without DA re-verification** — that is the whole point; if the DA had to re-check every PASS, the layer would save nothing. The DA's only standing engagement with a RELIABLE scan is **reviewing an allowlist *edit*** (when a new sanctioned door is legitimately added — a small, deliberate, infrequent review visible in the diff), never re-running the scan.
- **FAIL** — a *reliable* hard scan hit a definitive violation. Blocks the PR. Equivalent to a DA HOLD on that condition.
- **INSPECT** — a *soft* scan hit, OR a hard scan whose hit lands in a known false-positive zone. **Does not block the check**, but is **not done**: it routes to the **triage layer (§1A)** — a free agent tier that resolves it (delete / green / escalate) so the scarce DA sees only the genuine-gray-zone residue. An INSPECT is never silently merged and never auto-routed straight to the DA.
- **scanner self-test** — if any hard scan stopped firing on its fixture (the scan rotted), the **whole report is FAIL** regardless of PR contents. A scanner that can't catch its own known-bad cannot certify anything.

**Reliability legend (the DA must know what to trust):** each scan is tagged `RELIABLE` (a hit is a real violation — CI-blocking) or `HEURISTIC` (a hit may be a false-positive / needs judgment — INSPECT-only). The DA consumes FAILs as findings, INSPECTs as "closer look," and ignores nothing silently.

**Loop discipline (binding — three precisions that keep the verdict honest):**
- **CI check-status vs doctrine verdict are two things.** The CI *check* goes red only on a hard `FAIL` or a self-test `FAIL`; an `INSPECT` verdict leaves the check green-but-flagged. So `PASS` (verdict) means `selftest=PASS, failures=0, inspect=0`; an INSPECT-bearing run is not a PASS even though its check is green.
- **INSPECT never auto-merges — it routes through triage (§1A), on *every* PR.** A green-with-INSPECT check is **not** a PASS (verdict ≠ check-status, above). On *any* PR — not only `seal-residue-risk` ones — an INSPECT routes to the free triage tier, which clears it (delete/green) or escalates to the DA; this closes the gap where a non-seal-residue PR could merge green-with-INSPECT unreviewed. A `seal-residue-risk` PR additionally **holds merge** until its escalation (if any) is DA-classified. Merge-then-notify is how INSPECT rots into furniture — the exact failure §0 forbids.
- **A green positive control is necessary, not sufficient — the DA audits allowlist *legitimacy*.** A too-permissive allowlist makes `master` scan clean while *laundering a real leak into "sanctioned."* At every gate the DA verifies each `allow/*.txt` record is a genuinely sanctioned door (correct `door-class`, true rationale) — not merely that hard-failures = 0. And **until `CI-A-WORKFLOW-0` lands there is no CI**: the scan is self-reported by the implementer, so the DA independently re-runs/audits the branch **against the tree, never the relayed report** (the kernel track's most-repeated lesson).

---

## 1A. The INSPECT triage layer — closing the cost-asymmetry loophole

**The flaw, named precisely.** INSPECT inverts the project's core economics exactly where it is weakest. The allowlist machinery exists *because* grep-compute is free and DA-judgment is scarce — but INSPECT triggers for free (one agent retry) and clears at DA cost. A capable agent hill-climbing away from a hard `FAIL` doesn't need to *solve* the violation; it needs any edit that lands the symbol in a HEURISTIC scan's acknowledged false-positive zone — the exact zone the DA tuned in for legitimate code. No malice required: retry-pressure alone converges on the soft spot. Left unspecified, a green-with-INSPECT PR is either silently merged (a backdoor) or routes every flag to the scarce DA (the layer earns nothing). Both lose.

**The fix — a free triage tier between the scanner and the DA.** This is the project's capability-tiered resourcing model (Cursor/Grok for code, Haiku/Sonnet for docs, Opus/Owner for judgment) extended one stage. An INSPECT routes first to a **triage agent** — the **orchestrator tier**, the owner's *free / unmetered* Codex/Grok web-chat — which resolves each flag to a terminal state; only the residue reaches the metered DA:
- **DELETE** — a genuine false-positive; logged with its reason; the PR proceeds.
- **GREEN** — the agent fixed the underlying issue. Triage **verifies *why* it is now legitimate** (correct door-class, real rationale, the violation actually gone) — *not* that the scanner merely went quiet. "Scanner stopped complaining" is **not** a clearance.
- **ESCALATE** — a real gray zone triage can't adjudicate → the DA, with triage's reasoning attached, so the DA *verifies a claim* instead of *deriving a finding from nothing*.

**Four fences keep triage honest — without them it is just a cheaper venue for the same dodge:**
1. **The agent pays first (cost symmetry).** Before any INSPECT is eligible for triage, the PR author attaches a **one-line structured justification per flag** (allowlist-rationale shape). No justification = `unresolved` (not-done). This shifts the *first* pass of cost from DA-judgment (scarce) to agent-prose (cheap, auditable).
2. **Bounded loop.** Triage gets a **hard cap of 3 attempts** per flag; past the cap → automatic ESCALATE. "Loop until the scanner goes quiet" unbounded is the same adversarial dynamic relocated, with the orchestrator as unpaid accomplice.
3. **Detectable INSPECT-spam → FAIL-equivalent (the bounds you asked for).** Triage escalates-as-FAIL immediately (no further loop) on any greppable hill-climbing signature: **(i)** more than 3 INSPECT flags on a single branch across its commit history; **(ii)** the same symbol tripping ≥ 2 *different* HEURISTIC scan-ids across commits (symbol-walking — hunting for the soft scan); **(iii)** INSPECT count rising while an original RELIABLE `FAIL` on the branch stays unaddressed. A PR that lands once in a genuine gray zone looks nothing like one that walked a symbol through four triggers — and the **branch history**, not the final state, makes the difference visible.
4. **Decorrelated reviewer + DA spot-audit.** The triage agent is a **different model family** from the coding agent (or runs a narrow doctrine-only context), so correlated blind spots don't pass a borderline case "for the same reason the agent produced it." And because a triage clearance is a *relayed report* — the precise thing the kernel track learned never to trust blindly — the **DA spot-audits a fixed % of DELETE/GREEN clearances against the tree** until triage accuracy is empirically established. A triage clearance is a **named sanctioned-door residue** (`triage-cleared-uninspected`): bounded by the audit rate, greppable via the triage log, never silent.

**The bonus the design didn't have to add: the triage log *is* the promotion telemetry.** Every outcome (delete/green/escalate + reason, keyed by `scan-id`) appends to `scripts/ci/triage_log.tsv`. That log is, for free, the per-scan-id failure-rate data the retirement ladder always needed and never had: a HEURISTIC that keeps reaching ESCALATE is *by construction* a scan that needs promotion or deletion. **Periodic corpus maintenance** is then a real, data-driven cadence (per-track-closeout, or scheduled): the DA reads the log and tightens/promotes/deletes — no separate monitoring effort.

**Scope of the triage regime — three classes land on the orchestrator; only one is triage.** Conflating them wastes the scarce DA turn the regime exists to save.
- **(A) INSPECT flags → the triage loop above** (delete/green/escalate, bounded, spam-bounds). This is the only thing "triage" means.
- **(B) Scope/acceptance judgments** — is an added artifact in-scope, inert (§H 11), thin-enough, or a forbidden dependency? **The orchestrator resolves these against the doctrine itself** (the CI-hot-path test; the "no commercial tooling / MCP" definition; the §H inert-scaffolding rules; the thin/free principle), *states its ruling with reasoning, and proceeds.* It escalates only when the doctrine is genuinely silent or the call needs DA *taste* (a novel or precedent-setting expansion). A standard interpreter used as a **dev-time** audit/enumeration helper (not in the per-PR scan path) is not "commercial tooling," and a robust parser that fixes a fragility which *caused* a gate failure is not scope creep — that is a doctrine-answerable (B), ruled by the orchestrator.
- **(C) Gate reviews → always the DA** (the tree-audit is non-delegable; the orchestrator holds and relays, never self-passes).
A scope judgment that arises *during* a gate review is **(B)-within-(C)**: the orchestrator rules it with reasoning and flags it for DA override at the audit that is happening anyway — it does **not** hand it up as an open decision. Escalating a doctrine-answerable (B) *as a decision* is the same cost-asymmetry mistake the triage loop fixes for (A).

**Honest residue (named, per doctrine — this does not eliminate the failure mode).** The triage tier *relocates* the failure to a free tier and *bounds* it with the audit; it does not close it. A colluding or blind-spot-sharing triage agent can still pass a violation — the spot-audit makes that *probabilistically caught*, not impossible. This is a deliberate trade: **"the DA verifies every INSPECT against the tree" → "triage verifies every INSPECT + the DA spot-audits a sample,"** justified *only* because the triage tier is free and the DA tier is scarce. It is recorded as residue-as-tripwire, not advertised as a closed gate. The **carrot (Track C)** attacks the same problem from the other end — an agent self-scanning in its inner loop has a *cheaper legitimate path than hill-climbing*, which lowers INSPECT volume at the source and lightens this tier.

---

## 2. Track A — grep-only CI doctrinal tripwire (NOW)

| Rung | ID | Scope | Recipient | State | DoD |
|---|---|---|---|---|---|
| 0 | `CI-A-TRACK-OPEN-0` | This doc + evidence row. | Haiku/Sonnet | **DONE** | doc lands. |
| 1 | `CI-A-SCAN-DEFS-0` | **The auditable data home.** `scripts/ci/scans.tsv` (one line per scan: `id \| severity(RELIABLE/HEURISTIC) \| target-glob \| pattern \| exclude \| doctrine-ref \| promotion-blocker`) + `scripts/ci/allow/*.txt` (typed admission records, one per line: `symbol \| door-class \| rationale \| promotion-blocker`) + `scripts/ci/README.md` (layout + the onboarding heuristic: how to add a scan, and how a new producer earns an allowlist entry). Seed it with §5. | Cursor/Grok | **COMPLETE** | data files exist; README documents add/edit + onboarding; a new scan = one `scans.tsv` line *with its promotion-blocker*; a new sanctioned door = one conforming `allow/*.txt` record. No heuristic lives in shell. |
| 1a | `CI-A-RUNNER-0` | `scripts/ci/doctrine_scan.sh` — a **thin, stable engine** that reads `scans.tsv` + `allow/`, applies each scan (multiline-capable, `rg -U`), emits the §1 report. The engine carries *no* invariant-specific patterns. **Every FAIL line prints its sanctioned remedy** — an allowlist FAIL says *"if this is a legitimate new door, add `<symbol>` to `allow/<file>.txt` with a one-line rationale (PR-reviewed); do not edit the scanner"* — so an agent channels the friction into the allowlist edit instead of burning tokens trying to defeat the CI. Runnable locally (`bash scripts/ci/doctrine_scan.sh`). | Cursor/Grok | **COMPLETE** | runs locally; emits the report; exit non-zero **only** on a hard FAIL (INSPECT exits zero); engine has zero hard-coded scan patterns; each FAIL line names the file+line to edit to resolve it legitimately. |
| 1b | `CI-A-ALLOWLIST-SCANS-0` | **The rigor upgrade — closed-set allowlist scans (RELIABLE), so a clean result is trusted without DA re-verification.** Three allowlists in the `scripts/ci/allow/` data home (rung 1), each a reviewed sanctioned set; *anything outside the set FAILs* (catches novel/subtle holes a blocklist misses): **(a) sealed-type producers** — every `pub fn … -> (Self\|ThresholdEvent\|EmissionRecord\|ThresholdEmission\|PlacedParticipant\|ResolvedWriteAuthority\|…)` in the authority crates must be on `allow/sealed_producers.txt` (`read_*`/`dispatch_*`/`apply_*`/`cpu_oracle_*`); **(b) authoritative buffer handles** — every `pub` `&Buffer`/`Buffer` field/accessor in `simthing-kernel` must be `pub(crate)` *or* on `allow/inert_buffer_handles.txt`; **(c) kernel public surface** — the kernel's exported `pub` items are a closed set in `allow/kernel_surface.txt`; a new public item not on the list FAILs. **Plus the onboarding heuristic — the scanner validates the allowlist's own form:** a record whose `symbol` name doesn't match its declared `door-class` grammar FAILS, and a record missing any field FAILS (casual cruft can't be filed; every entry is self-justifying). **0R2 (§1A):** constructor exclusion (`new`/`default`) applies only outside sealed impl blocks; public sealed `new/default -> Self` hard FAILs (no allowlist door); pre-existing `gpu_readback.rs` readback constructors are expected findings until a crate seal rung. | Cursor/Grok | **COMPLETE** (debt CLEARED at `CI-A-CLOSEOUT-0`: `SEALED_TYPES` migrated from `scan_allowlists.py` to `allow/sealed_types.txt` — data, fail-loud; engine now carries no sealed-type invariant) | each allowlist scan FAILs when a fixture adds an unsanctioned producer/handle/surface item; a *malformed* allowlist record (wrong door-class grammar, or missing rationale) FAILS; adding a *legit* new door = one conforming `allow/*.txt` record (greppable in the diff); the doc-hidden-minter fixture FAILs with no per-name blocklist pattern; **0R:** temp `impl ThresholdEvent { pub fn forge_probe -> Self }` hard FAILs; `PlainHelper::new -> Self` does not; **0R2:** sealed `new/default -> Self` hard FAILs; non-sealed `PlainHelper::new` does not. |
| 2 | `CI-A-FIXTURES-0` | `scripts/ci/fixtures/` — tiny known-bad inputs (one per RELIABLE scan: a `pub fn -> &Buffer`, a `for_kernel_readback` minter, a `type ColumnIndex = usize`, a missing-forbid-unsafe crate stub, an unsanctioned sealed-producer, **a split-declaration producer** — `pub fn forge(` on one line, `-> ThresholdEvent {` on the next — which the allowlist scan must still catch, **a malformed allowlist record** — a `forge_x` filed under door-class `read`, and a record missing its rationale — which the onboarding heuristic must reject) **and** known false-positive traps (a jomini-style `write_*`, a `studio_antialiasing`-style module name, a `pub(crate)` sealed accessor, a semantic word in a comment/doc). **Plus HEURISTIC production negative controls** (semantic words, `.kind` read, `.data[N]`, stringly channel) to prove exclusions did not no-op HEURISTIC scans. Corpus is inert until `CI-A-SELF-TEST-0`. | Cursor/Grok | **COMPLETE** | fixtures exist under `scripts/ci/fixtures/`; each known-bad maps to exactly one scan; ALLOW-SEALED-PRODUCERS has explicit/split/Self/constructor/doc-hidden fixtures; HEURISTIC production controls exist; trap corpus committed; `fixtures/README.md` maps fixture→scan/verdict; normal `doctrine_scan.sh` on master remains PASS. |
| 3 | `CI-A-SELF-TEST-0` | `scripts/ci/doctrine_selftest.sh` — runs each RELIABLE scan against its known-bad (must FAIL) and against the trap corpus + clean master (must NOT FAIL). The **high-leverage ubuntu executable dimension** the owner approved: this self-test runs *in CI* before the PR scan, so a rotted scan fails loudly. | Cursor/Grok | **DA-CLEARED** (0R via repair #1041) | Opus independent local run 2026-06-30: `doctrine_selftest.sh` **green on master** (positive control PASS, rot test PASS, no FAIL) — the `cfg_test_semantic_words` determinism defect is fixed. #1039's committed proof-junk is fully removed (no tracked junk remains). Non-blocking debt: ~7-min runtime (process-spawn bound) — a runnable-fast self-test is the anti-fabrication guarantee; optimize when convenient. |
| 4 | `CI-A-WORKFLOW-0` | `.github/workflows/doctrine-scan.yml` — `ubuntu-latest`, no toolchain: checkout → **ensure `rg` present** (preinstalled on the runner; a single `apt-get install -y ripgrep` fallback line keeps it resilient if the image ever changes) → run self-test → run PR scan → publish the report (job summary + uploaded artifact the DA reads). Hard FAIL fails the check; INSPECT annotates. **This is the authoritative gate — it runs entirely on GitHub; the Windows/PowerShell owner runs nothing.** | Cursor/Grok | **COMPLETE** (exercised green by the #1043 landing PR's Actions runs — PR run 28494374601, master push 28494467422) | a known-bad PR fails the check; a clean PR passes; an INSPECT-only PR passes with the flag visible in the report; the workflow does not assume any tool present on the owner's machine. |
| 4r | `CI-A-WORKFLOW-0R` | PR-delta HEURISTIC enforcement — `doctrine_pr_scan.sh` / `doctrine_scan.sh --pr-delta`: RELIABLE whole-tree, HEURISTIC on changed files/lines only; master push keeps whole-tree positive control. | Cursor/Grok | **COMPLETE** (DA-verified both sides: local `--prove-delta` PASS + GitHub PR run 28495379717 "Doctrine scan (PR delta)" step logs `heuristic scope: changed files / changed lines` → `VERDICT: PASS`, while the master-push path stays whole-tree) | PR workflow delta-scopes HEURISTIC; pre-existing baseline HEURISTIC outside delta does not re-flag; INSPECT-only PR exits 0; hard FAIL exits nonzero. |
| 4a | `CI-A-INSPECT-TRIAGE-0` | Implement §1A's concrete surface (protocol is doctrine; this is its thin data + tooling — **no new engine logic**): **(a)** the report carries a **per-INSPECT justification slot** the PR author fills (one line, allowlist-rationale shape); an INSPECT with no justification reports `unresolved`. **(b)** `scripts/ci/triage_log.tsv` (append-only: `scan-id \| branch \| outcome(delete/green/escalate) \| reason \| commit`) — the promotion telemetry. **(c)** `scripts/ci/inspect_spam_check.sh <branch>` — the three hill-climbing bound-greps (branch-history flag count > 3; same-symbol ≥ 2 HEURISTIC scan-ids; INSPECT rising while a RELIABLE FAIL stays open) → emit `SPAM` (escalate-as-FAIL) or `OK`. | Cursor/Grok | **DA-CLEARED** (0R via repair #1041) | Opus independent local run: `inspect_spam_check.sh --prove` **PASS all five cases** — the three §1A bounds (>3 delta INSPECTs; same-symbol ≥2 HEURISTIC scan-ids; INSPECT rising while a RELIABLE FAIL open) plus alias-quarantine (a branch named `spam` with no history → `OK`). Workflow enforces (`>` redirect, no `\|\| true` — a `SPAM` exit fails the step). Temp-repo isolation verified (the run left no synthetic `.rs` in the real tree). |
| 4b | `CI-A-SELFTEST-INSPECT-REPAIR-0` | Correct the contaminated 0R sequence: record the premature #1039/#1040 merges truthfully, keep current master free of root proof-junk, repair `inspect_spam_check.sh --prove` so all synthetic work occurs inside temp repos, remove branch-name verdict shortcuts, and rerun the local proof battery. | Codex | **COMPLETE** (DA-cleared #1042; GitHub-side workflow green — run 28495380603) | Opus independent tree-verification 2026-06-30: the contaminated #1039/#1040 substance is now sound (self-test green, spam-bounds real + enforcing + proven), and the repair honestly recorded the premature merges. **Process breach noted** (two rungs merged before DA clearance) — accepted *because* the self-report was truthful and the substance verified, **not** as precedent; no rung merges before DA clearance. Evidence follow-up: record the actual final post-merge Actions run ID. |
| 5 | `CI-A-DOCTRINE-LANDING-0` | **Docs-only — prepare every future agent on every track.** Land the CI layer permanently at all three altitudes so an agent learns the contract *without reading this track doc* — **including the §1A triage contract** (INSPECT routes to the free triage tier under bounded-loop + spam-bounds + DA spot-audit; the triage log drives periodic corpus maintenance): **(a) core design** (`simthing_core_design.md`, at/near §1.2/§1.2.1) — the doctrinal admission ladder now has an *automated* rung-3 enforcement layer (the CI doctrine-scan), whose success metric is scan **retirement** as invariants promote to types (the mechanized form of "a guard scan that exists only because a type didn't is a promotion target"); **(b) constitution** (`design_0_0_8_3.md`, carry-forward §0.x) — the **DA-equivalence contract** (a clean RELIABLE allowlist scan = the DA ran it; FAIL = HOLD; INSPECT = look), the **rustified onboarding heuristic** (the allowlist is a *typed admission record*, not a babysat list), and the scan-retirement obligation, tied to §0.6.6 no-inert-scaffolding; **(c) handoff template** — a spine bullet (CI doctrine-scan = the automated DA scan layer; trust a clean RELIABLE verdict, don't re-verify; an allowlist edit is a deliberate, reviewed widening) + the **mandatory `seal-residue-risk` field** (does this rung touch a sanctioned door?) + the retirement obligation (a rung that promotes an invariant to a type deletes its scan in the same PR) + the Track B trigger rule. | Haiku/Sonnet (DA-reviewed) | **COMPLETE** (#1043; DA-verified in the tree at closeout — DA-equivalence + retirement present at all three altitudes) | Landed at all three altitudes: core design §1.2.1 (new CI doctrine-scan paragraph), constitution §0.9.5 (DA-equivalence + §1A triage + allowlist + retirement + merge-hold contracts), handoff template (spine bullet + `ci-doctrine-scan`/expanded `seal-residue-risk` fields + retirement obligation + merge-hold/verify-the-tree rule). Evidence: `docs/tests/ci-a-doctrine-landing_results.md`. Final Actions run IDs for the 0R repair chain clarified in `docs/tests/ci-a-selftest-inspect-repair-0_results.md`; `.tmp_*` scratch cleanup (#1042) verified present. |
| F | `CI-A-CLOSEOUT-0` | Record the **DA-equivalence contract** (PASS = DA-ran; FAIL = DA-HOLD; INSPECT → DA) and the **retirement contract** (a future rung promoting an invariant to a type deletes its now-redundant scan in the same PR — scans shrink as types grow) in this doc; **verify `CI-A-DOCTRINE-LANDING-0` landed all three altitudes**; publish the reliability legend. | Opus/Owner (DA) | **DA-CLOSED** (2026-07-01) | both contracts recorded (see closeout block above); three-altitude landing verified in the tree; reliability legend published; `SEALED_TYPES` debt cleared; full battery PASS on master; **Track A CLOSED**. |

**Validation battery (Track A's own proof — runs locally, and the self-test also in CI):**
1. **Positive control** — scan current master → zero hard FAILs (no false-positive on the real tree).
2. **Negative control** — scan each known-bad fixture → the matching scan FAILs (the scan actually catches its violation).
3. **False-positive corpus** — scan the trap fixtures → no hard FAIL (exclusions work; traps land as PASS or INSPECT).
4. **Rot test** — neutralize a scan pattern → the self-test FAILs (the scanner can't silently degrade).

**Opt-in local pre-push hook (zero new scope — same engine).** `CI-A-WORKFLOW-0` also ships `scripts/ci/install-hooks.sh`, which a developer *opts into* to symlink `doctrine_scan.sh` as a `pre-push` hook — catching a leak before the network round-trip instead of after. **Opt-in, never auto-installed** (silently rewriting a contributor's `.git/hooks` on checkout is exactly the kind of inert-magic surprise we reject); the hook is Git-for-Windows compatible (bash hook under the Git shell). CI remains the authoritative gate — the hook is a courtesy fast-path, not a substitute.

### Track A closeout — **CLOSED** (Executive DA: Opus/Owner, 2026-07-01, `CI-A-CLOSEOUT-0`)

Track A is closed. All rungs COMPLETE/DA-CLEARED; the doctrine landed at all three altitudes (#1043); the `SEALED_TYPES` engine hard-code was migrated to `allow/sealed_types.txt` (data, fail-loud); the full command battery passes on master (self-test PASS, scan PASS, PR-delta PASS, spam-prove PASS, 195/195 kernel surface). Evidence: [`docs/tests/ci-a-closeout_results.md`](tests/ci-a-closeout_results.md).

**Contracts recorded as closed doctrine (landed core §1.2/§1.2.1, constitution §0.x, handoff spine):**
- **DA-equivalence** — a clean RELIABLE (allowlist) scan = "the DA ran it"; FAIL = HOLD; INSPECT → §1A triage. Trusted without DA re-verification; the DA's standing engagement is reviewing an allowlist *edit*.
- **Retirement** — a scan is residue: when its invariant promotes to a type / admission hard-error, the same PR deletes or narrows the now-redundant scan. Success = scans *deleted*, not owned.

**Reliability legend (published):**
```
RELIABLE  — clean is DA-equivalent for that scan class; FAIL = HOLD; allowlist scans are closed-set, trusted when clean.
HEURISTIC — produces INSPECT, never a hard FAIL; routes to §1A triage; delta-scoped in PR CI, whole-tree only for the master positive control.
INSPECT   — requires author justification (missing = unresolved); bounded loop + spam-bounds prevent hill-climbing; DA spot-audit is the backstop.
SELFTEST  — validates the scanner + fixtures; tool-missing emits FAIL, never a false PASS (§0.6.6).
```

**Remaining non-blocking debt:** `doctrine_selftest.sh` runtime ~7 min (process-spawn bound) — optimize when convenient; it does not block close. **Track B (executable harness) remains deferred until a consumer needs executable seal-proof; Track C (the carrot) is DA-CLOSED.** *(Update 2026-07-02: that consumer arrived — Track B OPENED, §3.)*

**Auditable screening-surface reference:** [`ci_screening_surface.md`](ci_screening_surface.md) — the single authoritative map of the screening logic, every scan/allow/block-list file, the strict rigor to add a `scans.tsv` or allowlist entry, and the triage agent's narrowing role. Built for auditability, maintenance, and Track C introspection/onboarding. Any change to the screening surface updates it in the same PR.

## 3. Track B — local executable validation harness (OPEN — DA-OPENED 2026-07-02)

> Opens after Track A lands and a consumer needs it (e.g., 0.0.8.5 rungs wanting seal-proof on every change).
> **Revised execution split (owner-ratified 2026-07-02):** only **GPU/Bevy-exclusive** execution stays local
> (real-adapter parity/terminal proofs, Studio/`simthing-mapeditor` legs); **all other executable verification
> runs GitHub-side** (§3B) — free CPU runners, connector-visible to the webchat orchestrator. Reuses the
> same data home (§0) and the §1 report contract, so the DA consumes build/test/seal-proof results in the
> *same* tripwire-tagged form as the grep layer. **DA adjudication (2026-07-02):** D4's "`cargo test
> --workspace` is never run" binds *agent loops* (token/time economy); the free CI substrate is exempt — the
> GH workflow runs per-crate CPU batteries excluding Bevy/GPU, and this exemption never transfers back to an
> agent's local loop.
>
> **Trigger rule (the one Track-B detail fixed now, so it isn't re-litigated later):** Track B runs locally
> for any rung whose handoff declares `seal-residue-risk` — i.e. it modifies logic *inside* a sanctioned door
> (the residue grep can't reach, per §1). Every other rung is fully served by Track A. This keeps the
> expensive layer off the PRs that don't need it and is the precise complement to "green Track A ≠ door logic
> verified." Anything beyond this rule is deferred until a consumer actually opens Track B.
>
> **Opening adjudication (executive DA, 2026-07-02, owner-ratified).** The consumer arrived and is named: the
> **0.0.8.5 seal-residue door-logic rung class.** Live evidence: the original `TP-SCALE-ENVELOPE-0` terminal
> proof was a `catch_unwind` false-green that **passed grep-CI by design** (door logic is invisible to Track A,
> exactly as §4 declares) and was caught only downstream; its 0R/0R2 repairs were `seal-residue-risk` rungs
> whose seal-proof (`compile_fail` + CPU-oracle parity + real-adapter terminal assertion) had to be run by the
> scarce DA personally — the precise cost this track exists to eliminate. Phases 3–6 of 0.0.8.5 (shipsize
> decoder, combat arena, fronts, fleet movement) make recurrence near-certain. The trigger rule above is
> unchanged and now live. **Named candidate variant (recorded, owner-gated, not opened):** a separate,
> non-blocking GitHub-side workflow running only the `compile_fail` doc-test suite (toolchain, no GPU) — it
> would give the webchat orchestrator remote seal-proof visibility instead of a relayed local report. It must
> never touch the blocking Track A gate (the no-toolchain/instant property is inviolate) and opens only on
> owner authorization.

| Rung | ID | Scope | Recipient | State | DoD |
|---|---|---|---|---|---|
| 0 | `CI-B-TRACK-OPEN-0` | Open when a consumer needs per-change seal-proof. | Opus/Owner (DA) | **DONE — DA-OPENED** (2026-07-02) | consumer named: the 0.0.8.5 seal-residue door-logic rungs (owner-ratified; see opening adjudication above). |
| 1 | `CI-B-LOCAL-HARNESS-0` | `scripts/ci/doctrine_tests.sh` (local) — **narrowed to the GPU/Bevy-exclusive residue (owner-ratified 2026-07-02)**: real-adapter parity/terminal proofs and Studio/`simthing-mapeditor` legs only. Everything CPU-side moves to `CI-B-GH-CPU-0`. Batch invocations, never spawn-per-fixture (§3C simplicity debt). Emits the §1 report. | Cursor/Grok | **OPEN** (queued) | runs locally; emits the report; covers only what GitHub cannot execute. |
| 2 | `CI-B-TRIPWIRE-TAGS-0` | Executable-specific tripwires: **GPU-skipped → `INSPECT`** (seal/parity not fully verified here; owner's machine confirms — never a silent PASS); **flaky / perf-variance → `INSPECT`** with the run band (the +49% single-run noise needs a multi-run); **compile_fail proven / parity bit-exact → `PASS`**. | Cursor/Grok | **OPEN** (queued after 1) | each tag emitted correctly on a representative run. |
| F | `CI-B-CLOSEOUT-0` | "Test batteries executed by the DA" holds for the executable dimension too — run on the machine that can execute them; contract recorded. | Opus/Owner (DA) | **OPEN** (after 1–2) | recorded; track CLOSED. |

### 3B. Track B expansion — GitHub-side CPU execution for webchat orchestration (owner-mandated, 2026-07-02)

> **Owner mandate (2026-07-02):** extend Track B so a webchat orchestrator with the GitHub connector can *run
> and consume* CPU-side executable verification GitHub-side, not only via relayed local reports. Constraint
> honored by construction: the blocking Track A gate keeps its no-toolchain/instant property untouched —
> everything here is a **separate, non-blocking workflow**. Contract: a green GH-CPU run is **DA-equivalent for
> the CPU-side seal-proof class only**; GPU parity stays local and tripwire-tagged (GPU-skipped → `INSPECT`),
> per §1 and the Track B trigger rule. Verified 2026-07-02: `.gitattributes` LF normalization is in place, so
> ubuntu byte-identity tests are platform-safe.
>
> **Owner edict on full batteries (2026-07-02):** Track B exists to avoid hygiene-theater test sweeps. Bare
> full-crate `cargo test -p <crate>` is forbidden in automatic PR-triggered, comment-triggered, and default
> doctrine-exec paths. Broad full-crate batteries are quarantined behind owner-deep `workflow_dispatch` only
> and must never be the default proof path for a small-edit handoff.

| Rung | ID | Scope | Recipient | State | DoD |
|---|---|---|---|---|---|
| 3 | `CI-B-GH-CPU-0` | `.github/workflows/doctrine-exec.yml` — **separate, non-blocking** executable proof. Default PR execution is the bounded `ci-b-webchat-smoke` profile: script syntax, profile lint, and plan proof only. Exact targeted profiles may run named/doc/test-binary commands. Broad full-crate `cargo test -p <crate>` batteries are quarantined as `owner-deep-full-cpu-quarantined`, allowed only by `workflow_dispatch` with `owner_deep=true`. Emits the §1 report + a **`DOCTRINE-EXEC-VERDICT:`** footer to the job summary and `doctrine_exec_report.json`. | Cursor/Grok | **PROBATION** (`CI-B-WEBCHAT-PR1R`) | smoke default runs quickly; profile lint rejects casual full-crate tests; owner-deep is noisy and dispatch-only; timeout/fail/probe paths finalize with footer and artifact; Track A workflow is untouched. |
| 4 | `CI-B-GH-COMMENT-0` | ChatOps initiation: a `/seal-proof` PR comment (**collaborator-only; never fork-triggered**) dispatches `doctrine-exec.yml` on the PR head and posts/updates **one sticky PR comment** carrying the report footer — the webchat orchestrator both *initiates* and *reads* executable verification in the PR thread, no local relay. | Cursor/Grok | **PROBATION** (`CI-B-WEBCHAT-PR1`) | comment triggers the run; sticky comment updates, never duplicates; non-collaborator comments are ignored. |
| 5 | `CI-B-SURFACE-TRUTH-0` | **Candidate — owner-gated.** In `doctrine-exec.yml`: `cargo public-api` diff of `simthing-kernel` against a committed baseline — the **compiler-derived** public surface, auditing the grep allowlist's *enumeration completeness* (the single-line-`pub use` class of hole, §5). The fast grep stays the blocking gate; this is its periodic deep verifier — the "stricter reference implementation" differential the verifier-fuzzing literature prescribes (arXiv:2606.01066, verified). | Cursor/Grok | **PROBATION** (`CI-B-WEBCHAT-PR1`) | a surface item invisible to the grep enumeration but present in rustdoc output is reported. |
| 6 | `CI-B-GH-TRIAGE-0` | **Triage/orchestration empowerment (owner-ratified 2026-07-02 — the high-leverage GH-side tooling).** (a) The doctrine-scan workflow posts/updates **one sticky PR comment** carrying the `DOCTRINE-SCAN-VERDICT` footer + any INSPECT lines + the spam-check verdict — the triage agent reads the whole §1A picture in the PR thread. (b) A **collaborator-only** `/triage <scan-id> <delete\|green\|escalate> <reason>` PR comment-command validates the §1A row format and **commits the row to `scripts/ci/triage_log.tsv`** on the PR branch — the webchat triage tier discharges the entire §1A loop GitHub-side, with the row visible in the PR diff. (c) A malformed `/triage` command is rejected with the required format printed (FAIL-as-teacher). No new engine logic; the §1A protocol is unchanged — this is its remote *surface*. | Cursor/Grok | **PROBATION** (`CI-B-WEBCHAT-PR1`) | `/triage` appends a valid row and rejects malformed ones; the sticky comment updates in place, never duplicates; non-collaborator comments are ignored; the log row is diff-visible. |

#### 3B.1 Webchat-orchestration contract (adopted from orchestrator review, DA-adjudicated 2026-07-02 — governs rungs 3–6)

- **Command channels:** `/seal-proof` and `/triage` are accepted from **both** `issue_comment` and
  `pull_request_review` / review-comment events — the webchat GitHub connector reliably submits PR reviews
  even where plain issue comments are unavailable. Same collaborator-only rule; **never run untrusted fork
  code under a write token.**
- **SHA-bound verdicts (the verify-the-tree rule, mechanized):** every sticky comment, job summary, and
  artifact carries `pr`, `head_sha`, `base_sha`, `tested_ref`, `workflow_run_id`, `job_id`. A report is
  **stale unless its `head_sha` equals the current PR head** — the orchestrator never accepts an old green
  after a force-push.
- **Merge-ref testing:** PR proofs run against `refs/pull/<PR>/merge` where available (the
  `actions/checkout` default on `pull_request`); the report states `tested_ref` and `merge_ref_status:
  PASS | UNAVAILABLE` — `UNAVAILABLE` is `INSPECT` for merge-sensitive rungs (a branch-head PASS can still
  fail after master merges).
- **Durable machine artifact:** upload `doctrine_exec_report.json` (`artifact_version: doctrine-exec.v1`;
  verdict, SHAs/refs above, exact commands, tests, failures, inspect entries, triage rows) beside the human
  footer. The footer stays the source of truth; the JSON is its **generated mirror from the same run**
  (digest discipline) — never a divergent second truth.
- **Profiles (data over the fixed engine):** `scripts/ci/doctrine_exec_profiles.tsv` maps rung risk classes
  to exact targeted batteries (`profile_id | profile_class | risk_class | crate_checks | tests | doc_tests |
  gpu_required | expected_verdict_if_gpu_missing`); invoked as `/seal-proof profile=<id>`. `profile_class`
  is `smoke | targeted | probe | owner-deep`. Smoke is the PR default, targeted must be exact and bounded,
  probe is isolated guard-bite work, and owner-deep is dispatch-only artillery. Populated **reactively per
  rung class, never speculatively** (§4 rigor); prevents over- and under-running batteries.
- **Probe mode (guard-bite on demand):** `/seal-proof probe=<probe-id>` runs a known-bad fixture and
  **expects red — a green probe against a known-bad is itself a FAIL.** Probes run in temp-workspace
  isolation (the #1041 lesson). This mechanizes the DA deep-review action "prove the guard bites"
  (`ci_screening_surface.md` §5). Seed probes: `compile-fail-seal-break`, `panic-swallow`,
  `invisible-pub-use`, `macro-expanded-seal-export`.
- **Plan mode:** `/seal-proof plan [profile=<id>]` prints the resolved commands **without running them** —
  orchestration confirms the battery matches the rung's Graduation-routing block before spending runner time.
- **REJECTED — verdict PR labels** (`doctrine-exec/pass|inspect|fail`): a non-SHA-bound advisory mirror
  reintroduces the exact staleness channel the SHA-binding closes, and a second status surface rots into
  inert scaffolding (§0.6.6). The sticky comment + the checks UI are the display surface; do not re-derive.

Operational orchestrator guide: [`ci_screening_surface.md`](ci_screening_surface.md) §9 is the quick-reference surface for webchat-triggered `/seal-proof`, `/triage`, stale-report checks, report artifacts, profiles, probes, and plan mode. §3B.1 remains the canonical contract; the surface doc is the operator-facing map.

### 3C. DA re-evaluation log (2026-07-02, Fable 5 — full-structure review, owner-mandated)

Findings and proposals recorded so they are never re-derived. Nothing below weakens the blocking gate or adds a
metered dependency; everything is free-tier. Nothing was found that should be **deleted** — the structure passes
its own minimalism test; the opportunities are promotions and free-capability captures, not repairs.

**Platform promotions (owner-gated; one-time repo settings — each lifts an existing convention to platform
enforcement, the admission-ladder move applied to the repository itself):**
1. **Branch protection on `master` with `doctrine-scan` as a required status check.** Verified 2026-07-02:
   master is currently **unprotected** — FAIL-blocks-merge is convention only. This makes it platform fact.
2. **Enable repo auto-merge** (verified currently disabled). DA clearance = approve; the platform waits for
   green — removes the merge watch-loop for AFK/webchat orchestration.
3. **CODEOWNERS over `scripts/ci/**` + `docs/sanctioned_surface.md`** — platform-enforced review on
   screening-surface edits. Trade-off: requires the owner's human review event on rare allowlist edits.

**Track C / GWM extensions (owner-gated; constitution-level if adopted):**
4. **The Promotion Rider.** Extend the breakthrough valve's signed-payoff geometry (arXiv:1402.3610 — strictly
   positive, decoupled marginal utility) from rare architectural experiments to the **common currency**: any
   rung MAY carry an optional, **non-gating** rider that promotes one rung-adjacent invariant up the admission
   ladder (scan/prose → type/admission hard-error), retiring its scan **in the same PR**. Genuine = credited in
   the graduation log; absent = zero; fabricated = FAIL. Self-verifying by construction (the rider's proof *is*
   net-negative enforcement surface). Converts `scans.tsv` shrinkage from a passive obligation into a standing
   positive pull.
5. **The baseline-zero promotion signal.** Corpus-maintenance rule: a HEURISTIC whose whole-tree master baseline
   is **zero** across consecutive maintenance reviews is **promotion-ready** (upgrade to RELIABLE, or complete
   its type boundary) — the opposite tail of "chronically firing → retire." **Live datum:** as of 2026-07-02
   all four HEURISTICs are baseline-zero on master; all four are flagged for promotion evaluation at the next
   corpus-maintenance review.

**Data-home addition (fold into the CI-B-1 handoff; §4 rigor — fixture + trap + selftest wiring):**
6. **`TEST-PANIC-SWALLOW` (HEURISTIC → INSPECT):** `catch_unwind` over `crates/**/tests/**` — born from the live
   `TP-SCALE-ENVELOPE-0` false-green (the incident is its own known-bad fixture). Promotion-blocker: retire if a
   proof-path panic-swallow becomes structurally unrepresentable.
6b. **Adversarial evasion fixtures (same handoff):** extend the fixture corpus with the Rust evasion classes
   grep is structurally weakest against — a macro-expansion producer (`macro_rules!` expanding to a
   `pub fn -> SealedType`), an inline-`mod` split declaration, an attribute-interposed declaration
   (`#[inline]` between `pub fn` and its return type), and **rename-on-re-export grammar laundering**
   (`pub use …::forge_x as read_x;`) — each either caught by its scan or honestly downgraded per the §0
   line-split rule. Grounded in verifier fuzzing (arXiv:2606.01066, verified): any looseness in the verifier's
   parsing stack is a latent false-PASS channel, and the allowlist false negative is this layer's named worst
   failure.
7. **Scheduled corpus-maintenance workflow (owner-gated, parking-aware):** a monthly `schedule:` run of the
   whole-tree scan + per-scan-id triage-log stats, updating **one standing issue** — keeps the §1A maintenance
   cadence alive while the repo is parked. Only valuable if the owner reads it; owner decides.
8. **Windows-runner CPU parity matrix (owner-gated candidate, orchestrator-proposed 2026-07-02):** a
   `windows-latest` job running the `f32::to_bits` oracle suites so the cross-platform bit-exactness sentinel
   (§3B rung 3) proves both directions in CI rather than ubuntu-CI-vs-owner's-box. Costs runner minutes
   (Windows runners are slower); opens only on owner authorization.

**External research adjudication (2026-07-02; owner-relayed digest, all four arXiv IDs verified live on arXiv):**
- **ADOPTED IN PART — arXiv:2606.01066** (*Fuzzing RLVR Verifiers*): as items 6b (adversarial evasion fixtures)
  and the `CI-B-SURFACE-TRUTH-0` stricter-reference differential. The full dynamic mutation **engine is
  REJECTED**: it gold-plates rung-3 residue designed to shrink, against an RL-optimizer threat model we do not
  have — our agents are frozen, bounded-retry, and already bounded by the §1A spam-bounds.
- **ADOPTED IN PART — arXiv:2606.31706** (*AdaTrans*, error-adaptive repair; 95.51%/81.09% empirical): the
  repair-posture stratification lands as one onboarding bullet in `ci_screening_surface.md` §7 — scanner FAIL =
  token-cheap printed remedy; kernel seal breach = structural re-derivation, never lifetime/clone/`unsafe`
  patch-append; parity mismatch = oracle-first.
- **REJECTED — arXiv:2605.04000** (RL false-positive suppression + selective fuzz for Rust static analysis):
  category error here — our INSPECTs flag *ontology* leaks, not memory-safety warnings; memory safety is
  discharged at rung 1 (`forbid(unsafe_code)` + compiler), and fuzz-validating admission-guaranteed layouts is
  the §H proof-battery anti-pattern. Its "~81 baseline INSPECTs" premise is stale (baseline is zero); an
  auto-clear of INSPECT→PASS would delete the legitimacy check ("scanner stopped complaining is not a
  clearance"). Its sound grain — ambiguous static signal → cheap localized dynamic check before scarce
  judgment — **is already the Track B trigger rule.**
- **REJECTED / PARKED — arXiv:2602.01698** (*Latent Exploration Decoding*): requires intermediate-layer logit
  access — infeasible on the closed-API fleet (the same gate as the parked constrained-decoding study). Its
  behavioral analog (explore-then-harden dual pass) is deliberately **inverted** by the breakthrough valve
  (conformant baseline FIRST; exploration rides risk-free alongside) — the GWM-correct order; a mandated
  unconstrained first pass on production rungs is a drift channel, not a carrot.

**Evaluated and REJECTED (do not re-derive):** Miri/sanitizers (the authority crates forbid `unsafe`; near-zero
yield); consolidating the three thin Python engines (churn without dividend); a new track letter or central
registry (the per-track addendum already generalizes); making `doctrine-exec` blocking (a toolchain in the merge
path violates the inviolate no-toolchain property); agent reputation/scoring tiers (governance ceremony);
`cargo fmt`/clippy gates in Track A (same inviolate property). **Simplicity debt noted:** the ~7-min self-test
runtime is Windows process-spawn overhead only (ubuntu ≈1 min); the Track B local harness must batch, never
spawn-per-fixture.

## 3A. Track C — the carrot (generation-time constraint, DA-CLOSED)

> The stick (Tracks A/B) catches violations *after* generation. The carrot pulls the same constraint
> geometry *forward* so the conformant path is the path of least resistance — the immediate-feasible form of
> the constrained-decoding research (arXiv 2504.09246 / 2605.30054 / 2605.29986). **Full logit-masking stays
> parked: it needs logit access we don't have on a closed-API fleet (Cursor/Grok/Claude).** What *is*
> feasible reuses Track A's artifacts and needs no new infrastructure. Opens after Track A lands; lightens
> §1A by reducing INSPECT volume at the source.

> Closeout note: Track C is closed as of 2026-07-01. The table preserves rung history; current lifecycle state is COMPLETE for C1/C2/C3 and DA-CLOSED for CF.

| Rung | ID | Scope | Recipient | State | DoD |
|---|---|---|---|---|---|
| 0 | `CI-C-TRACK-OPEN-0` | Open after Track A closes. | Opus/Owner (DA) | **DONE — DA-OPENED** (2026-07-01; Track A verified CLOSED in the tree, `doctrine_scan` PASS) | Track A CLOSED; Track C opened, sequenced, and later DA-CLOSED at `CI-C-CLOSEOUT-0`. |
| 1 | `CI-C-INNER-LOOP-0` | **Tier 1 — tighten the agent's inner loop.** Convention (handoff + landing): a coding agent runs `cargo check -p <crate>` **and** `bash scripts/ci/doctrine_scan.sh` after each small edit, not at PR time — the *same* scanner consulted earlier, so a doomed path is pruned seconds after birth, in the agent's own loop, before DA/CI/triage see it. The FAIL-with-remedy (already in `CI-A-RUNNER-0`) is the steering signal. | Haiku/Sonnet (convention) + Cursor/Grok (adopt) | **COMPLETE** (DA light review 2026-07-01 — trusted green CI; verified only the residue: `debug_kind()` is display-only, zero tick-path call sites, no new semantics; the `faction_capability_unlock` slip caught pre-PR by the inner loop; `triage_log.tsv` seeded with 1 real GREEN row (commit fd11b746), clearance spot-audited legitimate) | Convention landed as a one-line pointer in `handoff_template.md` (context spine) and `agents.md`, both delegating detail to `ci_screening_surface.md` §6. Sample rung: a real `simthing-sim` inner-loop edit (`ThresholdSemantic::debug_kind`) tripped `SEMANTIC-WORDS` (INSPECT, real transcript), was triaged GREEN and fixed before PR, logged in `triage_log.tsv`. See `docs/tests/ci-c-inner-loop-0_results.md`. |
| 2 | `CI-C-DIGEST-0` | **Tier 2 — the sanctioned-surface digest (context-space "CFGzip").** `scripts/ci/gen_digest.sh` reads `allow/*.txt` → a compact `docs/sanctioned_surface.md`: the *only* kernel doors an agent may call (door-class + signature) + the forbidden patterns + why. Injected into any kernel-touching handoff. The CI onboarding heuristic keeps `allow/*.txt` current, so the digest is **accurate for free** — the same artifact that keeps CI honest steers the agent's context. The killer aid for **low-context agents**: a tiny, always-current statement of the valid surface, so a small window can't drift. **Token-economy purpose — it is the agent's *pre-computed grep answer* (read-instead-of-grep):** a kernel-touching agent otherwise burns exploratory greps rediscovering the sanctioned surface every rung; the digest hands it the answer, replacing the *"what may I call / is X sealed"* greps — as the inner-loop self-scan replaces *"did I violate"* greps and FAIL-as-teacher replaces *"where is the violation"* greps. **Boundary:** the digest answers *sanctioned-surface + conformance only*, never general code navigation — it is **not a code index and must not grow into one** (that metastasis is what §0's minimalism forbids; general navigation greps stay the agent's own). | Cursor/Grok | **COMPLETE** (0R DA-verified at CF, 2026-07-01: `gen_digest --check` wired into the workflow with `set -o pipefail` (exit not masked); DA perturbed the digest → `--check` exit 1 (drift bites); 5/5 embedded sha256 match live sources) | `scripts/ci/gen_digest.sh` regenerates `docs/sanctioned_surface.md` from `scripts/ci/allow/*.txt` + `scripts/ci/scans.tsv`; `--check` byte-compares and verifies generated door/type rows exactly equal parsed allowlist data; **and `--check` runs in `.github/workflows/doctrine-scan.yml` so drift hard-FAILs on every PR and master push.** Evidence: `docs/tests/ci-c-digest-0_results.md` and `docs/tests/ci-c-digest-0r_results.md`. |
| 3 | `CI-C-TRACK-ADDENDUM-0` | **Opt-in, auto-detaching per-track CI addendum — generalize the harness without sprawl.** A production track MAY carry a CI addendum **co-located with its own track doc** (a fenced `ci-addendum` block, or a sibling `<track>.ci.tsv` / `<track>.ci.allow/`): its track-specific scans + the sanctioned-surface a track-scoped digest draws from. Three hard properties keep the minimalist discipline intact: **(a) strictly opt-in** — most tracks carry none; absent an addendum only the global floor runs. **(b) auto-detaching** — the runner loads it only when invoked for that track's scope (the PR/handoff declares its canonical track doc); it travels *with* the track doc, so when the track archives its addendum archives too and stops applying — **no central registry accumulates, the global `scripts/ci/scans.tsv` / `allow/*.txt` stay sparse and singular.** **(c) additive-only** — an addendum can ADD a scan, TIGHTEN, or define a digest surface; it can **never remove or widen the global floor** (a config-level laundering attempt — an addendum that disables/loosens a global scan — hard FAILs). Same rigor as the floor: DA-reviewed like an allowlist edit, each entry carries `promotion-blocker` + door-class grammar; **populated reactively from the triage log, never speculatively.** The `CI-C-DIGEST-0` generator reads *global + the active track's addendum* for that track's onboarding only. | Cursor/Grok | **COMPLETE** (DA-verified at CF, 2026-07-01: `--prove-addendum` PASS with substantive assertions — opt-in, auto-detach, additive-only-reject, digest-scope; global `scans.tsv`/`allow/**` byte-unchanged; impl commit `bc2e23ecc7` clean; results-doc packaging irregularity ruled cosmetic) | `doctrine_scan.sh --track-doc <track-doc>` loads only the selected sibling `<track-doc>.ci.tsv` / `<track-doc>.ci.allow/`; default scan remains global-only; `--prove-addendum` proves opt-in, auto-detach, additive-only scan-id rejection, and active-track-only digest scope. No `scripts/ci/scans.tsv`, `scripts/ci/allow/**`, workflow, or crate edits. Evidence: `docs/tests/ci-c-track-addendum-0_results.md`. |
| F | `CI-C-CLOSEOUT-0` | Record that the scanner now serves all three pipeline positions — **before** (digest), **during** (inner-loop self-scan), **after** (CI gate) — one artifact slid leftward; FAIL-as-teacher confirmed; the opt-in per-track addendum is documented in `ci_screening_surface.md` as the *only* sanctioned generalization path (global floor stays sparse). **+ the first corpus-maintenance review:** Track C dogfoods the live CI+triage tier — every INSPECT its PRs (and the C1 substrate-touching inner-loop demo) raise is triaged via §1A and **logged to `triage_log.tsv`, never silently passed**; at closeout the DA reads the accumulated corpus, classifies what fired (false-positive / real / gray), and names any chronically-firing HEURISTIC as a retirement/promotion candidate (or records "corpus thin → scans well-tuned"). | Opus/Owner (DA) | **DA-CLOSED** (2026-07-01 — C1/C2/C3 all COMPLETE; three-position scanner live; corpus reviewed = 1 GREEN row, too thin for promotion/retirement, no action; **Track C CLOSED**; evidence `docs/tests/ci-c-closeout-0_results.md`) | recorded; `triage_log.tsv` corpus reviewed; retirement/promotion candidates named or explicitly "none"; corpus observation recorded; track CLOSED. |

## 4. Track D - test-corpus paring: admission ladder applied to tests

Track D opens after `CI-B-WEBCHAT-PR1R` as a corpus-maintenance track for tests themselves. Fable High / DA ruling
`DA-RULING: ADMISSION-BOUNDARY-COLLAPSE` resets the deletion authority: the old source-file-family ledger is legacy
context only. Current authority is boundary-keyed, with tests hanging off the proof boundary that owns the invariant.

The burden of proof is on retention for admission-adjacent, hygiene-theater, and usecase-superseded rows. Historical examples are not inherently valuable. A test whose invariant is owned by a type/seal boundary, admission hard-error, doctrine scan, compile-fail proof, oracle, golden artifact, or required invariant is retired, collapsed, or consolidated according to the owning boundary tier. Rejection-class enumeration beyond one representative is kabuki by existing law.

Classifier-input families consolidate to one table-driven test when variants exercise distinct classifier paths. They do not survive as N independent tests, and they are not deleted if no other boundary owns the classifier behavior.

Fable route for operators:

- Owner-deep full batteries are quarantined artillery, not routine proof.
- Until material reduction lands: weekly scheduled sentinel = sentinel-core only; full quarantined battery = workflow_dispatch-only.
- `sentinel-core` means oracle/parity, compile_fail, golden-byte, STEAD-required, determinism, doc-named invariants, and active live rung suites where applicable.
- After paring, the full battery may graduate to scheduled sentinel plus ladder-boundary run.
- Smoke PASS is mechanics-only; it is not seal-proof.
- Seal-residue rungs still require targeted profile/probe proof.
- Timeout semantics and profile data remain Track B concerns.
- Track D standard and ledger rungs create deletion authority; per-boundary waves perform actual deletions/collapses.
- **GHA targeted Doctrine Exec is CPU-only** unless the profile is explicitly `owner-deep`. Atlas/Bevy/GPU/desktop/mapeditor/tools runtime binaries are local-owner-deep only. The GHA profile may use `cargo check` floors for affected crates, but not runtime integration binaries that require desktop/GPU/cold heavyweight paths. Enforced by `scripts/ci/doctrine_exec_gha_proof_seal.sh` via `doctrine_exec_profile_lint.sh` (see `GHA-PROOF-SEAL-0`).

Draconian rule: never pare `compile_fail`, trybuild, seal-proof, oracle-parity, golden-byte, STEAD-required,
invariant-required, doc-named invariant coverage, `custom_layout_ethics_axis`, escaped-bug regressions, or active
track live rung suites while their track is open, unless a later DA-cleared rung proves an explicit stronger
superseding boundary.

Boundary tiers:

- `TIER1_TYPE_SEAL`: illegal state is uncompilable; runtime duplicates are DELETE candidates with zero runtime representatives.
- `TIER2_ADMISSION_HARD_ERROR`: one representative negative test per admission boundary; variants collapse to that representative.
- `TIER3_DOCTRINE_SCAN`: scanner self-test is the representative; source tests duplicating scanned invariants are DELETE candidates.
- `TIER4_CLASSIFIER_CONSOLIDATION`: real classifier coverage becomes one table-driven test containing the input rows.
- `TIER5_BEHAVIOR_REGRESSION`: retained behavior/escaped-bug rows unless a future stronger owner is named.
- `TIER6_PROMOTION_REQUIRED`: real invariant with no owner yet; this is a rustification queue, not a shield.
- `TIER7_NEVER_PARE`: terminal proof classes and active live rung suites.

| Rung | ID | Scope | State | DoD |
|---|---|---|---|---|
| D0 | `TEST-PARE-INVENTORY-0` | Mechanical inventory and classification of the current test corpus; zero deletions. | **DONE — DA-APPROVED** (2026-07-02, validated end-to-end during the D-DA/D-LAW deep review) | `scripts/ci/test_inventory.tsv` exists; `scripts/ci/test_inventory_check.sh` validates schema, exact mechanical coverage, never-pare rules, and superseding-boundary requirements; results recorded in `docs/tests/test_pare_inventory_0_results.md`. |
| D1 | `TEST-PARE-AUDIT-1` | Legacy source-file-family audit of D0 candidate rows. | **SUPERSEDED** | Preserved as historical context only; it is no longer deletion authority after `DA-RULING: ADMISSION-BOUNDARY-COLLAPSE`. |
| D2a | `TEST-PARE-CLAUSETHING-0` | First per-crate collapse under the old same-family standard. | **COMPLETE / HISTORICAL** | Closed by #1086; its 41 blocked rows are rekeyed by boundary in `TEST-PARE-STANDARD-DA-0`. |
| D-DA | `TEST-PARE-STANDARD-DA-0` | Adopt boundary-keyed paring standard and regenerate the ledger with zero deletions. | **DONE — DA-APPROVED** (2026-07-02, deep review; merged #1087) | `scripts/ci/test_pare_boundaries.tsv` and `scripts/ci/test_pare_boundary_rows.tsv` cover every live inventory row plus historical PARED rows; `scripts/ci/test_pare_boundary_check.sh` enforces owner/tier/disposition rules; module-marker rows are mapped or promotion-required; no cargo tests or crate edits. |
| D-LAW | `TEST-ADMISSION-REGIME-0` | Make test admission standing law, not Track D commentary. | **DONE — DA-APPROVED** (2026-07-02, deep review + DA 0R; landed on master via #1088→#1089) | Doctrine landings in core/constitution/handoff; `promotion_target` survivor trichotomy in `scripts/ci/test_inventory.tsv`; `TEST-BUDGET` delta heuristic; `test_inventory_drift_check.sh` stock gate wired into doctrine scan/selftest; kernel/sim KEEP strict tier; promotion-wave plan reconciles with ledger. |
| D2b | `TEST-PARE-SPEC-0` | First material `simthing-spec` deletion/collapse/consolidation wave under standing admission law. | **DONE — merged #1091** | Source review records 451 initial rows: 425 runtime rows deleted, 7 hygiene-theater inputs consolidated into one table-driven test, 1 integration representative retained from the actionable set, and 18 source-level rows blocked by the `src/**` edit ban and rekeyed for source-rung follow-up; live inventory drops to 5,870 rows; the `test-pare-spec` Doctrine Exec profile runs exact edited `simthing-spec` integration binaries only. |
| D2c | `TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0` | Cross-crate hygiene-theater classifier-input consolidation under Fable consolidation-exit. | **DONE — merged #1092** | 132 independent integration tests retired; one CPU-only `simthing-spec` table (`table::track-d::hygiene_theater_cases`) preserves all case labels; 0R rejected Bevy/Linux desktop bootstrap proof; targeted Doctrine Exec PASS run 28626085144; merge commit `ca19515999`; live inventory 5,739 rows. |
| D2d | `TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-0` | CPU-safe Tier 2 admission-boundary collapse over `simthing-clausething` and `simthing-mapgenerator`. | **DONE - merged #1094** | 33 clausething Tier 2 tests deleted, 8 representatives retained, mapgenerator rows blocked for checker-law follow-up, final targeted CPU-only proof PASS; merge commit `68f6b87de52964580fa6b11020e7d9ce64d13077`; live inventory 5,706 rows. |
| D2e | `TEST-EDIT-SCOPE-GATE-0` | Data-driven Track D test-edit scope gate replacing hardcoded checker exceptions. | **DONE - merged #1095** | `scripts/ci/test_edit_scope.tsv` authorizes clausething/spec historical CPU-safe test edit surfaces and DA-approved mapgenerator Tier 2 test surface; prove mode rejects src, kernel/sim, GPU/driver/mapeditor/tools/workshop, and other unauthorized crate edits; merge commit `b3da4f2b49`. |
| D2f | `TEST-PARE-MAPGENERATOR-ADMISSION-COLLAPSE-0` | Collapse CPU-safe mapgenerator Tier 2 admission fossils under the data-driven edit gate. | **DONE - merged #1096** | Six boundary families processed; 37 rows considered, 31 deleted, 6 representatives kept, 0 blocked; merge commit `5dc4c0d499`; live inventory 5,675 rows; final targeted Doctrine Exec PASS run `28630985641`, failures 0, inspect 0. |
| D2g | `TEST-PARE-CPU-SAFE-BOUNDARY-SWEEP-0` | Delete remaining CPU-safe fossil residue (table shells, hygiene cleanup) and disposition all live CPU-safe collapse rows. | **DONE - merged #1097** | 13 empty spec admission shells deleted; scenario_ingestion helper cleanup; 380 boundary rows dispositioned; inventory unchanged at 5,675; merge commit `cfe934f17b`; profile `test-pare-cpu-safe-boundary-sweep`. |
| D2h | `TEST-PARE-BROKEN-CLAUSETHING-ADMISSION-RESIDUE-0` | Delete broken clausething admission duplicates under owner-deep residue ruling. | **DONE - merged #1098** | 4 admission rows deleted; representatives proven; inventory 5,671; profile `test-pare-broken-clausething-admission-residue`. |
| D2i | `TEST-PARE-SRC-UNIT-FOSSIL-RESIDUE-0` | Delete src cfg-test admission/usecase fossil residue across protected crates under DA-approved temporary src scope. | **DONE — merged #1099** | 88 src unit rows deleted (mapeditor 42, kernel 15, core 9, sim 9, driver 6, gpu 6, feeder 1); inventory 5,583; profile `test-pare-src-unit-fossil-residue`; merge commit `3871f864e7`. |
| D2j | `TEST-PARE-GPU-BEVY-RESIDUE-0` | Delete Class 2 GPU/Bevy/desktop integration capability-proof relic tests across protected runtime crates under DA-approved temporary tests scope. | **DONE — merged #1100** | 251 integration rows deleted; 23 Class 1 kept; inventory 5,583 → 5,332; CPU-only GHA profile (cargo check floors + clausething CPU reps; no atlas hang); `atlas_0080_0` Class 1 survivor local-only; merge commit `a71700f3fb`; remedial closeout `a30bf54f3e`; Doctrine Exec master PASS run `28636990813` (failures=0 inspect=0). |
| D2k | `GHA-PROOF-SEAL-0` | Mechanical guard: non-owner-deep Doctrine Exec profiles cannot include Atlas/Bevy/GPU/desktop proof commands. | **DONE - merged #1101** | `doctrine_exec_gha_proof_seal.sh` + profile lint integration; `--prove-gha-proof-seal`; remediated `test-pare-src-unit-fossil-residue` mapeditor script residue; head `317aba88f649a027fcd2c9997b182a7c27005cce`; merge commit `e49c8a258e4bd58d9c78b6c82b698cd5650dbaca`; evidence `docs/tests/gha_proof_seal_0_results.md`. |
| D2l | `TEST-PARE-PROTECTED-CLASS-AUDIT-0` | Audit protected permanent-residue membership and judgment-class note rules before protected-residue deletion work. | **PROBATION / 0R** | `docs/tests/test_pare_protected_class_audit_0_results.md`; zero test deletion; 5,332-row inventory baseline; 829 protected KEEP rows audited by class-specific membership checks; 582 TRUE_MEMBER, 98 FALSE_MEMBER queued for PR B, 112 NEEDS_PROMOTION, 33 NECESSARY_CITED_DEPENDENCY, 4 LEDGER_DEFECT; 4,250 behavior-regression AUDIT rows confirmed not to be permanent survivor shields; future KEEP judgment rows require a specific `catches:` note. |
| D2m | `TEST-PARE-PROTECTED-RESIDUE-0` | Delete or terminally reclassify the failed protected-class residue surfaced by #1102. | **PROBATION** | `docs/tests/test_pare_protected_residue_0_results.md`; #1102 merged at `304541ae2e40a1afbb96d0ef9d435c5ceb06956c`; 98 FALSE_MEMBER protected oracle-parity claims reclassified to `usecase-superseded` PARE, 112 NEEDS_PROMOTION claims retained as `B-T6-PROTECTED-ORACLE-PROMOTION-REVIEW` AUDIT rows, 4 LEDGER_DEFECT module markers moved to module-marker expansion, and 33 dependency-floor rows preserved; inventory remains 5,332; zero source deletion and no cargo tests. |
| D2+ | `TEST-PARE-BOUNDARY-*` | Future coverage-complete deletion/collapse waves by boundary family. | **FUTURE** | Each wave processes every row in its named boundary families to terminal disposition; no row quotas; no broad full-crate test sweep is used as proof. |
| DF | `TEST-PARE-CADENCE-DF` | Decide owner-deep cadence after material reduction lands. | **FUTURE** | Until then, weekly scheduled sentinel = sentinel-core only and full quarantined battery = workflow_dispatch-only. |

> **DA graduation log (executive DA, 2026-07-02 — D0 + D-DA + D-LAW).** All three rungs **DONE / DA-APPROVED**
> after deep review against the tree: three-altitude doctrine landings verified (core §1.2, constitution
> §0.9.5 by addition incl. the kernel strict tier, handoff template §6); all four drift-gate FAIL cases
> proven biting (unledgered, stale-ledger, unowned-KEEP, kernel-strict-tier); ledger counts reconciled
> (6,301 rows; KEEP 829; AUDIT 5,472 awaiting waves); kernel/sim KEEP rows verified exclusively never-pare
> classes (138 oracle-parity / 39 seal-proof / 2 golden-byte). **DA 0R applied pre-merge:** the initial
> ledger filed all 122 promotion-target rows on never-pare-set members (the nine STEAD §8 suites +
> `custom_layout_ethics_axis`) — never-pare takes categorical precedence over promotion-targeting; root
> cause was the DA handoff's own incomplete `permanent-residue` enum (no doc-named/stead-required tokens).
> Enum extended in all three checkers, 122 rows reclassified, wave plan corrected to zero rows with the
> precedence rule recorded; the kernel/sim strict tier was deliberately **not** widened. **Process slip
> (the DA's own, recorded):** #1088 merged into its stacked base branch because #1087's head branch was not
> deleted at merge, so GitHub never retargeted the child PR — corrected via #1089 through the normal gate.
> Lesson: stacked merges use `--delete-branch`, or retarget the child before merging. **Recorded debt resolved by `TEST-PARE-SPEC-0`:** the legal `permanent-residue` token set now
> lives in `scripts/ci/test_residue_classes.tsv`, read by `test_inventory_drift_check.sh`,
> `test_inventory_check.sh`, and `test_pare_boundary_check.sh`.

## 5. Honest residue / non-goals

- **What grep still cannot prove, with allowlists, shrinks to one thing:** the *internal logic correctness* of a sanctioned door (does it compute/write the right thing) — Track B `compile_fail`/parity territory, genuinely not greppable, and triggered only on rungs that change a door's logic (flagged `seal-residue-risk`). The allowlist scans close the rest: "no unsanctioned producer/handle/surface exists" is now a *trusted* CI verdict, not a DA re-check. Do not, however, let a green Track A imply a sanctioned door's *logic* is verified — that is Track B.
- **HEURISTIC scans will have false-positives** by nature (semantic words, raw indices, kind reads can be legitimate). That is *why* they are `INSPECT`, not `FAIL` — surfacing-not-blocking is the correct posture; tightening a heuristic into a reliable type/admission boundary is the §1.2 promotion path, tracked separately.
- **No new commercial tooling / MCPs.** Pure shell + grep + GitHub Actions on a public repo. Free, instant, no learning-curve tax.
- Track A does **not** build, test, run GPU, or touch `simthing-mapeditor` (Studio/Bevy) — those are local-only.

## 6. Practical notes — the seed scans (reuse the DA's tuned greps)

The exact patterns the DA ran this session, with the false-positive exclusions already tuned. `CI-A-SCAN-SCRIPT-0`
encodes these; each carries its severity. Target `crates/**/src` (not `tests/` for HEURISTIC scans), exclude
`pub(crate)`, and exclude `compile_fail` doctest blocks + `//` / `///` / `//!` comment lines where the pattern
is illustrative.

**ALLOWLIST (strongest RELIABLE — closed-set; catches *novel* holes a blocklist misses):**
- **Sealed-type producer allowlist:** enumerate every `pub fn … -> (Self|ThresholdEvent|EmissionRecord|ThresholdEmission|PlacedParticipant|ResolvedWriteAuthority|CandidateFMagnitudeReport|…)` in the authority crates (`simthing-kernel`, and any crate re-exporting a sealed type). Each producer MUST be on `scripts/ci/allow/sealed_producers.txt` (the sanctioned doors: `read_*` / `dispatch_*` / `apply_*` / `cpu_oracle_*`). **Anything not on the list → FAIL.** This is the one that catches a re-added `from_boundary_delivery`, a `#[doc(hidden)] pub` minter, or *any future* unsanctioned path to a sealed type — no per-name blocklist needed.
- **Authoritative buffer-handle allowlist:** every `pub` `&Buffer`/`Buffer` accessor-or-field in `crates/simthing-kernel/src` must be `pub(crate)` **or** on `scripts/ci/allow/inert_buffer_handles.txt` (the provably-inert utilities, e.g. `IndexedScatterOp::dispatch` caller-owned args). Anything else → FAIL. (Allowlist form of the B3 scan below.)
- **Kernel public-surface allowlist (optional, strongest):** the kernel's exported `pub` items are a closed set in `scripts/ci/allow/kernel_surface.txt`; a new `pub` item not on the list → FAIL, forcing a deliberate, reviewable allowlist edit for every surface widening. **The door-class *grammar* (`read_*`/`apply_*`/`cpu_oracle_*`…) is enforced on the *producer* allowlist (`sealed_producers.txt`), where it is meaningful; the surface inventory must NOT file sealed/authority exports** (`ThresholdEvent`, `EmissionRecord`, `ThresholdEmission`, `ResolvedWriteAuthority`, `PlacedParticipant`, `WorldGpuState`, `GpuContext`, the `cpu_oracle_*` fns …) **under the wildcard `inert-util`** — that class matches *any* symbol (the runner's grammar check is a no-op for it), so blanket-`inert-util` launders the high-authority surface into "inert," defeating the legitimacy check (the PR #1021 binding criterion). `inert-util` is reserved for genuinely-inert constants/helpers; sealed/authority exports carry a distinct marker and are cross-referenced to their grammar-enforced producer in `sealed_producers.txt`. Enumeration must capture **both** grouped `pub use mod::{…}` and **single-line `pub use mod::Symbol;`** exports — missing the latter is a completeness hole.

**RELIABLE blocklist (a hit = violation → FAIL; keep as a fast belt-and-suspenders alongside the allowlists):**
- **B3 authoritative handle escaping the kernel:** `pub fn [a-z_]+\(&self\) *-> *&(wgpu::)?Buffer | ^\s*pub [a-z_]+ *: *Buffer | -> *BindingResource` over `crates/simthing-kernel/src`, **minus `pub(crate)`**. (Caught every kernel extraction leak.)
- **Named forge minters must never reappear:** `pub fn (from_boundary_delivery|for_kernel_readback|for_boundary_install)\b`. (The exact holes 0R/0R2 closed — redundant with the producer allowlist, but a cheap explicit tripwire.)
- **`forbid(unsafe_code)` present** on each semantic-free crate (`simthing-sim`, `simthing-kernel`): missing `#![forbid(unsafe_code)]` in `lib.rs` → FAIL.
- **AS-5 alias regression:** `type ColumnIndex *= *usize`.
- **`deny.toml` (or any compliance-shaped stub) reappears** unwired → FAIL (§0.6.6 — the inert-scaffolding lesson).

**HEURISTIC (a hit = `INSPECT`, possible false-positive → DA looks):**
- **Raw `data[N]`:** `\.data\[[0-9]+\]` over `crates/**/src`, **minus** `raw_lanes`/serialization sites. (FP: legit serialization byte-lanes.)
- **`match`/read on `.kind` in the sim:** `match .*\.kind | \.kind\b` over `crates/simthing-sim/src`, **minus** the kind-free runtime view. (FP: spec/authoring-layer kind, legit.)
- **Semantic game-words below the spec boundary:** `faction|combat|terran|pirate|diplomacy` over `simthing-sim`/`simthing-kernel` `src`, **minus** display-name/provenance/comments. (FP: a doc, a string, a test.)
- **Stringly channel identity:** `owner_ref *: *(Option<)?String | resource_key *: *(Option<)?String` over `crates/simthing-spec/src`. (FP: a justified serialization DTO with a recorded Deviation.)

**Known false-positive traps to exclude (the ones the DA hit and tuned out):** `crates/simthing-clausething/src/jomini/**` `write_*` (text writer, not buffer writes); `studio_antialiasing` module-name matches (not the AA *report*); `pub(crate)` (sealed = correct); a scan's own pattern-literals inside its `compile_fail`/results doc.

## 7. References

- Doctrine: [`simthing_core_design.md`](simthing_core_design.md) §1.2/§1.2.1; constitution [`design_0_0_8_3.md`](design_0_0_8_3.md) §0.6.6/§0.9.
- The B1–B8 catalogue: [`design_0_0_8_4_5_simthing_kernel.md`](design_0_0_8_4_5_simthing_kernel.md) §5.2.
- Handoff discipline: [`handoff_template.md`](handoff_template.md).
- Consumer: the 0.0.8.5 Terran-Pirate track, whose rungs consume the doctrine-scan layer as the standing automated DA scan.
- **Anti-satisficing / signed-payoff insight — Gopalakrishnan, Marden & Wierman, *"Potential Games are Necessary to Ensure Pure Nash Equilibria in Cost Sharing Games"* (2014), [arXiv:1402.3610](https://arxiv.org/abs/1402.3610).** The governing bound behind this layer's incentive design: a regime of pure verifiable constraints (a "push"-only gate) drives strategic agents to a Nash equilibrium *at the compliance boundary* — minimal-effort satisficing. To keep the equilibrium off that boundary, the **marginal utility of the elegant/experimental alternative must be strictly positive (> 0), not merely non-negative (≥ 0)** — a decoupled reward, not just a harmless lane. This is why the breakthrough valve (core §1.2.1) **signs** its payoff (genuine=+credit / none=0 / fabricated=FAIL) rather than settling for "risk-free," and why the whole layer pairs the stick (compliance floor) with a carrot (a positive pull the CI floor can enable but cannot itself adjudicate — elegance is DA judgment, never a greppable metric). *(The two 2026 ML-training papers offered alongside it — SIGReg / APO — describe gradient-time collapse in models we do not train; behaviorally apt as analogy, but not implementable on a frozen-model + grep-CI regime.)*
