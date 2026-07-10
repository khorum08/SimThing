# SimThing Implementation Handoff — Base Template

> **Purpose.** The single skeleton every rung handoff fills in. It operationalizes the orchestration
> directives D1–D8 ([`design_0_0_8_5_clausescript_terran_pirate_galaxy.md`](design_0_0_8_5_clausescript_terran_pirate_galaxy.md) §1A.2)
> and the constitution's harness discipline ([`design_0_0_8_3.md`](design_0_0_8_3.md) §0.5).
>
> **It exists to kill a specific, observed reflex:** handoffs that grow a 30-file reading list, a test
> *battery* for conditions the type system / admission layer already guarantee, a wall of bespoke
> grep-guards that merely restate the scope diff, a triple-doc ceremony update, and a fully hand-authored
> implementation inline. That is **hygiene kabuki** — activity that produces governance artifacts instead
> of the specified feature (constitution §0.6.5). This template makes the kabuki structurally hard to write
> and the real feature easy. **A handoff that violates §H (Anti-kabuki rules) is rejected at review.**
>
> Fill the `<…>` placeholders. Delete any section that carries no signal for *this* rung — empty sections
> are themselves kabuki. Keep it tight: a good handoff is read in one pass and held in context for the
> whole task.

---

## Context spine - enforced pointers + retained unmechanized norms

> Mandatory in every handoff. Mechanized doctrine is cited by enforcing surface; only unmechanized norms stay
> explicit.

### Enforced doctrine pointers

- **Everything is a SimThing / no side subsystems:** cite the core-design, semantic-free scan, sanctioned-surface, type-boundary, or admission-hard-error surface that applies.
- **Resource-flow authority path:** use existing RF / overlay / EML / admission substrate; no parallel planner or resolved-state side path.
- **Movement-front / STEAD constraints:** cite STEAD/movement-front tests and design surfaces for map, movement, front propagation, or exact-magnitude routing.
- **Semantic-free sim/WGSL:** cite semantic-free guards and CPU-oracle parity for `simthing-sim`, WGSL, opcode stacks, or exact claims.
- **Substrate ladder before new primitives:** RF arena -> overlay column -> EML gadget tree -> JIT EML->WGSL shader -> only then Tier-2 opcode/kernel/role, with DA/Owner routing.
- **Admission behavior over governance artifacts:** validation lands as type/admission behavior where possible; no registry, preflight artifact, or prose stand-in for a hard error.
- **Doctrine as type, not prose:** type boundary > admission hard-error > guard scan > prose; retire guards made redundant by higher boundaries.
- **Kernel authority and seal residue:** owning crate holds authority; declare cross-crate capability leakage, unsafe, sealed access, GPU readback/dispatch, and authority-boundary edits as `seal-residue-risk`.
- **Doctrine scan as automated DA scan layer:** reliable green covers what it covers; FAIL holds; INSPECT routes through triage; allowlist edits widen sanctioned doors deliberately.
- **Inner loop:** use the canonical entrypoints below instead of bespoke validation rituals.

### Retained unmechanized norms

- **No rung touching PROBATION / authority / gate state merges before DA clearance.** Verify the tree, not the relayed report; pasted proof is a claim until branch-confirmed.
- **If the change cannot be expressed through the existing substrate and enforcement pointers above, stop and escalate to DA. Do not special-case around the boundary.**

### Canonical Entrypoints

- Session admission: `bash scripts/ci/orient.sh --role=<coding|orchestrator|da>`; handoffs carry existing `ORIENT-RECEIPT` (stop if missing/stale).
- Track mutation (operator): `bash scripts/ci/gen_orientation.sh --open <track.md>`; `--check` is the freshness gate (Cold-Start Spine + corpus from live TSVs).
- **Doctrine lookup (THE entrypoint):** `bash scripts/ci/anchor_query.sh --domain <d> --paths <files...> --grep <term>` — verbatim anchors; **do not raw-grep doctrine docs**.
- **Anchored-doc edits:** run `bash scripts/ci/anchor_check.sh --resync` and commit updated `doctrine_anchors.tsv`.
- **Kernel doors:** read `docs/sanctioned_surface.md` first (generated from `allow/*.txt`). Graduated gates: `ExactMagnitudeProof` / Candidate F; decision ingress tokens → `StructuralCommitment`; `OpcodeRegistrationGate` / closed EvalEML; `ColumnIndex` via `col_for_role` (not `ColumnIndex::new`). EML authoring: `docs/eml_gadget_library.md`.
- **Clearance intake:** read auto-posted **Clearance Report** sticky; do not invoke `/clearance` for normal intake; `ANCHOR-ACKS` match sticky `REQUIRED-ANCHORS`.
- Routine local proof: `cargo check -p <crate>` when crates changed; **`bash scripts/ci/agent_scan.sh`**. Whole-tree `doctrine_scan.sh` is CI/maintainer. Conditional selftests when those surfaces change; else `scanner unchanged - selftest not required`.
- Track closure: `track_closeout.sh --build-manifest …` → `--check-eval` → `--apply` (`docs/track_closeout_protocol.md`). Never hand-delete inventory rows.
- GHA comment commands: `/orient`; `/clearance`; `/relay-lint`; `/triage`; `/anchor`; `/seal-proof`.

---

## 1. Identity

- **Rung ID / title:** `<RUNG-ID>` — `<one-line title>`
- **Type:** `<mechanical impl | design+impl | docs | DA review>`
- **Recipient agent:** `<agent>` — chosen by **Type**, per the routing table below.
- **Expected PR title:** `<RUNG-ID>: <imperative summary>`
- **Canonical design file (the one ladder this rung serves):** `<path>` — read it first; it controls scope/order/lifecycle.
- **`admission-amendment-request:` `denied` (default) | `allowed`** — whether *this* handoff grants the agent permission to **request** the owner-gated Admission-Substrate Amendment Valve. Leave `denied` unless the rung may genuinely need to add/repair/suspend a sealed kernel/admission restriction.
- **`seal-residue-risk:` `none` (default) | `<B#…>` | `authority boundary touched` | `scan-retirement candidate`** — does this rung touch the kernel **authority surface** (sealed types, authoritative buffers, GPU dispatch/encode/readback, derives on sealed types, `unsafe`, or kernel dependencies)? If yes, **name the bypass-state(s) it could produce** (the catalogue `B1–B8`, kernel track §5.2) so review runs the bypass scan and treats a hit as a **red flag** requiring DA sign-off. Use `scan-retirement candidate` when the rung promotes a scanned invariant to a type boundary or admission hard-error — see the **retirement obligation** below. The residue is unenforceable by types *by nature*; routing through it is always **deliberate**, so it must be **declared and scanned**, never silent.
- **`ci-doctrine-scan:`** — expected commands (`doctrine_scan.sh` / `doctrine_pr_scan.sh` / `doctrine_selftest.sh`); whether RELIABLE, HEURISTIC, or INSPECT paths are touched by this rung's diff; whether an `allow/*.txt` edit is expected (if so, name the file — an allowlist edit is a **deliberate, reviewed widening of a sanctioned door**, never a scanner-logic dodge to avoid a valid finding).
- **Retirement obligation** — if this rung promotes a scanned invariant to a type boundary or admission hard-error, it **deletes or narrows the now-redundant scan in the same PR** (`scripts/ci/scans.tsv` / `allow/*.txt`); a guard scan kept alive after its type boundary lands is residue, not a fixture.
- **Merge-hold rule** — a rung whose handoff or diff touches PROBATION status, kernel/CI authority, or gate-state semantics **does not merge before DA/Owner clearance**, full stop. **Do not trust relayed proof blindly — prefer verify-the-tree**, and **require** it for code-facing / long-lifecycle / horizontally impactful (load-bearing) escalations: the implementer's pasted transcript is a claim; the DA (or reviewer) confirms against the branch when the surface is load-bearing. Light residual and pure policy/stamp stages may stay lighter (see `agent_onboarding` DA). DA may run `bash scripts/ci/da_treeverify.sh --pr <n>` for an advisory depth profile (`DA-TREEVERIFY-PROFILE`) — not a clearance verdict.
- **Clearance reserve classification** — machine empty-class split (`CLEARANCE-ADMITTED-SCOPE-GAP-0`): `DA-RESERVE(admitted-scope-router-gap)` = admitted envelope + proof-present + missing class — **router debt**, class-harden, not a fresh DA design question. `DA-RESERVE(unclassified-scope)` = no class and no valid admitted-envelope claim. Novelty / gate-wiring preserved. Missing admitted-scope fields → `FAIL(...)`. Evidence: `docs/tests/clearance_admitted_scope_gap_0_results.md`.

**The Admission-Substrate Amendment Valve (owner-gated; do not work around seals).** The kernel/admission
seals (AS-1–8B + the kernel track) are owner-gated. If you hit a seal that genuinely blocks the rung, you
have exactly two moves: **(a)** if this handoff sets `admission-amendment-request: allowed`, you may *request*
the valve — surface a written request for **Owner / Exec-DA** approval, stating why it cannot be a
registration / EML gadget / overlay within the existing seal and whether it is add/repair/suspend; **(b)**
otherwise, **escalate the blocker to the DA.** You never self-grant, never suspend a seal yourself, and never
build a sidecar around it. The valve opens only on owner interrogation + approval, recorded as a Deviation
with a greppable marker (full protocol: the kernel track §3A).

**Architectural experiments — gated *and* invited (the breakthrough valve; core §1.2.1, constitution §0.9.6).**
The valve is a *pressure-relief for genuine insight, never an alternative to solving the problem.* If, while
delivering this rung, you perceive a materially better architecture that a seal blocks:
- **Deliver the conformant baseline first and let the rung close green.** The experiment is **decoupled and
  risk-free**: it **never gates your PASS** and **cannot cost you closure** — so surfacing it is welcomed, not
  penalized. (This is deliberate: without it, a closure-seeking agent buries the insight.)
- **Surface it as a *proven* proposal, not prose:** the working baseline + bit-exact CPU-oracle parity + a
  **measured** local hot-path dividend + why the substrate ladder (D2: RF → overlay → EML → JIT) cannot achieve
  it. **No baseline, no measured dividend, or an unexhausted D2 ladder → it is not entertained** — this keeps
  the channel *costlier than conformance*, never a shortcut, so it can't become the aspirational exit.
- **You surface; the Owner decides.** Never self-grant. A baseline-backed, dividend-carrying insight is a valued
  contribution; an argument in place of a working solution is a `FAIL`.
- **The payoff is *signed* so honesty strictly dominates — risk-free is not enough.** A genuine gated experiment
  is **positively credited** (recorded to the triage/amendment log with your authorship; the Owner routes trust
  and interesting work by it); finding **none costs you exactly nothing** (no obligation to invent one — silence
  is a valid, unpenalized outcome); a fabricated / ungated argument-in-place-of-a-solution is a `FAIL`. Merely
  *harmless-but-unrewarded* (payoff ≥ 0) still loses to silence for a closure-seeker; the surfaced insight must
  pay **strictly more than silence** (payoff > 0) or the equilibrium satisfices to the compliance boundary
  (Gopalakrishnan–Marden–Wierman 2014 — a stable non-degenerate equilibrium requires marginal utility of the
  elegant alternative to be positive, not just non-negative).
> **Design constraint for any future exploration lane (do NOT build before a consumer exists).** If/when the
> amendment log shows real demand and a lane is built, it must be **structurally non-gating** — a self-test
> proving a conformant baseline's PASS is *invariant* to any attached experiment artifact — so the risk-free
> guarantee is provable-by-construction, not merely promised (marginal-utility-≥0, mechanized). The *positive*
> pull (rewarding elegance) is **not** greppable and must **not** become a CI metric/score — elegance is DA
> judgment by nature; a CI script can enforce the floor (≥0), never adjudicate the reward (>0).

**Recipient routing (Type → agent tier).** Name the agent explicitly; do not default to the most capable
model for a mechanical task or the cheapest for a judgment task.

| Type | Recipient | Why |
|---|---|---|
| **Coding** — mechanical impl, design+impl, refactor, test | **Cursor / Grok** | high-throughput code execution against a fixed contract; the type system + admission layer + this template are the guardrails, so a coding agent needs latitude inside the fence, not DA judgment. |
| **Docs** — results ledger, design-doc edit, evidence-index update, closeout prose | **Haiku / Sonnet** | bounded, well-specified writing; no architectural authority needed. (Docs closeout still rides *with* its impl PR — §H — it is not its own rung.) |
| **DA review** — acceptance, sign-off, Scope-Ledger adjudication, Deviation approval, ontology conformance | **Opus / Owner** | the residue types and admission cannot reach (core §1.2 §5): no-CPU-planner, no-flattening, semantic conformance. Only the DA writes a sign-off, never pre-filed. |

A handoff that routes a DA-judgment rung to a coding agent, or burns a DA/Opus turn on mechanical edits, is
mis-routed — fix the Type or split the rung.

## 2. Harness — required reading (tight; not the repo)

Fixed base (always): the **context spine** above + the **canonical design file** (§1) + **this template**.
Plus **only the files this rung actually edits or directly depends on** (≤ ~6). If you find yourself needing
a file not listed, that is a signal the rung is mis-scoped — note it, don't silently widen.

```
<rung-local file 1>
<rung-local file 2>
…(keep it to what you will touch)
```

> Do **not** paste a 20-file boundary reading list. The spine carries the principles; the design file
> carries the scope; the rung-local files carry the work. More than that is drift (D7).

## 3. Mission (≤5 lines)

`<What to build, as a contract — not as code. State the capability and where it attaches on the tree.>`

## 4. Contract & conformance (the fence — specify, don't hand-author the implementation)

**Interface / behavior contract** (names may be adapted; concepts are stable):

```
<public API surface OR behavioral contract — interfaces, inputs, outputs, error classes.
 Do NOT inline the full implementation; the implementer owns how, within this fence.>
```

**Invariants this change must hold** (cite the spine items / D-directives that apply, not all of them):

- `<e.g. reuses the single existing atlas/arena — no second texture/bind-group/arena path>`
- `<e.g. malformed input is a spanned hard error at admission, not a runtime branch (D8)>`
- `<e.g. opt-in / default-off; presence alone stays inactive>`
- `<e.g. exact path routes through Candidate F; no native sqrt in a decision gate>`

**Dependency discipline** (if any): `<allowed crates; "do not bump wgpu/bevy/<core>"; record direct licenses>`.
If resolution tries to upgrade a broad core/render graph, **stop and report.**

## 5. Exploration latitude (production-forward by default)

Within the §4 fence the implementer **chooses the implementation freely.** A cleaner, more
SimThing-conformant factoring than any sketch here is **welcome and taken without asking** — note it in
the evidence doc; it does not need a new approval rung. When the substrate already supports the goal,
**build it** — do not insert a docs/closeout rung in front of the feature; closeout rides *with* the
implementation PR. Exploration is bounded only by: it must lower through existing substrate (D2),
hard-error at admission for malformed input (D8), and be provable by a load-bearing test (§6).

### 5.1 Design-space sketch — OPTIONAL, DA-authored, PROBATIONARY (seal-residue-risk rungs only)

> **Status: probationary convention (added 2026-07-04, DA). Governed by the Necessity Test** (core §1.2):
> it survives only if it demonstrably improves first-pass implementation quality on seal-residue rungs that
> nothing else provides. **Decision gate:** re-evaluated after the 0.0.8.5 seal-residue rungs generate an
> evidence base; if no measurable benefit surfaces, this section is **deleted**, not kept as good-process
> theater. It is **never mandatory** and adds **zero CI machinery** (no scan, no attribute, no gate).

For a rung flagged `seal-residue-risk`, the DA (or whoever holds gate-authority over the *opening spec*)
**may** add a short **Design-space sketch**: *"if the seal/type constraints did not exist, here is what the
solution space would look like."* The rationale is the cross-agent analog of latent-exploration decoding
(arXiv:2602.01698, verified — a decode-time single-model effect of ~0.61pp pass@1; the isomorphism, not the
magnitude, is what is borrowed): the **higher-entropy author** — the one *not* under gate pressure — hands
the implementing agent a richer prior *before* it collapses to the minimal gate-satisfying path. Binding
constraints so it cannot become the pathologies the CI track already fenced:

- **DA-authored, at the opening-spec stage — NEVER the implementing agent.** An implementer-authored sketch
  is the gameable same-agent dual-pass (plausible prose that is just the compliant solution described, then
  "hardened" into identical code — zero entropy preserved). Different-author-under-different-pressure is the
  *entire* load-bearing property; without it the sketch is theater.
- **No automated content check.** The sketch is *not* scanned for "genuine exploration" (that is §1A
  hill-climbing bait). Its only reader is the DA at acceptance review, using it as referenceable evidence
  that the design space was articulated before the implementation collapsed it.
- **No new artifact class, attribute, or commit-ordering rule.** It is one optional section of the opening
  spec that travels in the handoff context. It is not `SKETCH.md`, not `#[admitted(...)]`, not a gate.

Omit it freely — silence is a valid, unpenalized outcome. This is a lightweight prior-enrichment device on a
thin empirical thread, deliberately optional and self-deleting if it does not earn its keep.

## 6. Proof — minimal and load-bearing (the anti-battery rule)

Every test must name the **regression it catches.** Apply this triage:

- **Behavior that can actually regress** → test it (real lowering, GPU==CPU-oracle bit-exact, settle-then-bubble, determinism, no-panic-on-bad-input, cache-hit-no-recompute).
- **A rejection class** (malformed input the admission layer hard-errors on a **live/canonical path**) → **zero tests** — the production hard-error *is* the coverage (Necessity Test; the old "one representative per boundary" is a **retired fossil premise**, floor is **zero** not one). A negative test survives only if it catches genuine parser/format behavior a type + the live hard-error cannot (e.g. a specific span/message contract nothing else exercises).
- **Guaranteed by the type system / typestate / the firewall** → **zero tests.** You do not test that an enum cannot hold an illegal variant.
- **Reuse existing oracles/guards** (the semantic-free guard, the arena-allocation oracle, the canonical save/load battery). **Never re-derive** a guard the repo already runs.
- A new KEEP-class test names its promotion target or its permanent-residue class. **Lifecycle (standing law, §4.1 of CI-scaffolding design):** every test is **assumed DELETED at this track's closure** unless it (a) carries a canonical notion — then promote it into a `simthing-kernel` type/seal or EML opcode-stack construct and **delete the test**, (b) is a `TIER7` terminal proof class with a `catches:` note, or (c) is a non-runnable `dependency-floor` helper. "Might be useful later" is not a keep reason.

List the load-bearing tests for this rung, each with its one-line justification:

```
<test_name>            — catches: <the specific regression>
…
```

> GPU legs skip cleanly without an adapter. `cargo test --workspace` is **never** run in routine proof — the sole exception is the **one-time PR-ladder closure certificate** (§4.1): a track/ladder closeout may run `cargo test --workspace --all-targets` once to certify the survivor set as a whole (or to satisfy a deferred DA review). It is a closure certificate, not a validation path.

## 7. Evidence — one doc, one line, one row (no ceremony triple-update)

- **One results doc:** `docs/tests/<rung>_results.md` — include **only** the sections that carry signal
  for this rung: `Status` · `PR/branch/merge` · `What changed` · `Load-bearing proofs` · `Scope Ledger`
  (specified vs implemented/proxied/deferred, constitution §0.6) · `Known gaps / next`. Drop the rest.
- **One evidence-index line** in `docs/tests/current_evidence_index.md`.
- **One status-row edit** in the canonical design file (lifecycle: PROBATION → … ).
- **No separate "production-log" prose essay** restating the mission. If a worklog exists, a one-line pointer is the maximum.

## 8. Validation (targeted; reuse, don't re-author guards)

```bash
cargo fmt -p <crate> -- --check
cargo check -p <crate>
cargo test -p <crate> --test <this_rung_test>
cargo test -p <crate> --test <one_or_two_named_regressions>   # not the whole tree
cargo test -p <crate> --test semantic_free_guard               # reuse the existing guard
git diff --name-only master...HEAD | grep -E "<forbidden-path-glob>" && echo "SCOPE VIOLATION" || true
```

> **One** scope check (the forbidden-path diff) and the **existing** semantic-free test are sufficient.
> Do **not** author additional bespoke grep-guards that merely restate the scope boundary — the diff *is*
> the boundary (D8, §H).

## 9. Acceptance (behavior, not artifact count)

PASS only if:

```
- <the capability works, expressed as observable behavior>
- <malformed input hard-errors at admission; no panic; no state mutation>
- <reuses the single existing atlas/arena/path; no parallel subsystem>
- <named regressions still pass; semantic-free guard passes>
- <Scope Ledger complete; lifecycle row updated>
- <no out-of-scope crate/path touched (the §8 diff is clean)>
```

## 10. Non-goals (brief)

```
<the adjacent rungs / systems explicitly NOT in scope — one list, no rationale essays>
```

## 10b. Orientation receipt (gate-wiring rungs — enforced by relay-lint)

A fresh agent session is oriented once at cold start by user / Owner / DA instruction.

A handoff does not command a new full orientation run. It requires the agent to carry its session
`ORIENT-RECEIPT` when the harness validates one, especially for gate-wiring / receipt-tracked work.

If the receipt is missing or stale, stop and report that to the operator / DA. Do not re-run full orientation
as a per-handoff habit.

Required receipt fields when carried:

```
ORIENT-RECEIPT:
role:
orientation_digest_sha:
```

## 10c. Anchor acknowledgement (trigger-domain rungs — enforced by relay-lint)

```
ANCHOR-ACK:
```

> One line per required anchor: `ANCHOR-ACK: <anchor_id>@<12-char hash>` (from `anchor_check.sh --resolve` or after `anchor_query.sh`). Match sticky `REQUIRED-ANCHORS`.

## 11. Response format

```
Status:
PR / Merge:
What changed:
Load-bearing proofs (+ what each catches):
Scope Ledger:
Conformance (spine/D-directives held):
Known gaps / next:
Graduation routing (for DA — why this is PROBATION, not self-marked COMPLETE):
  CI verdict:          <PASS-RELIABLE | INSPECT(n) | FAIL>
  Triage entries:      <none | scan-id:outcome …>   (this rung's rows in triage_log.tsv)
  Risk class:          <none | semantic | data-deliverable | gate-wiring | seal-residue | allowlist-edit>  (name ALL that apply)
  Falsification check: <the exact check(s) that would confirm/deny "done" — what the DA runs to spend tokens precisely>
  Recommended posture: <light: confirm deliverables | deep: investigate> — <one line why>
```

> **The orchestrator never self-marks a rung COMPLETE.** It relays PROBATION with the **Graduation-routing**
> block above so the DA routes graduation *cost* by declared risk, not by re-deriving it from scratch. The
> triage log says *what fired*; this block says *what the scanner can't see* (the structural risk class) — the DA
> needs both. Routing rule (`ci_screening_surface.md` §5): `Risk class: none` + green RELIABLE + no escalation →
> **light** (confirm named deliverables; tree dig optional unless load-bearing); **data-deliverable /
> gate-wiring / seal-residue / allowlist-edit / triage-ESCALATE** → **deep** (byte-faithfulness, prove-the-guard-bites,
> tree/legitimacy audit). The **Falsification check** names exactly where review should spend — never
> "re-verify everything." **Verify-the-tree is weighted, not universal:** required for code-facing /
> long-lifecycle / horizontally impactful load-bearing work; relaxed for pure policy, stamps, and light residual
> (`docs/agent_onboarding.md` DA section).
> **After a DA pass**, the DA stamps the active workplan Exit proof (`DA-GRADUATED / merged #<PR> @ <sha>` or
> equivalent DONE), results COMPLETE, regenerates orientation, and **merges the stamp PR** as part of the
> graduation conclusion (`docs/agent_onboarding.md` DA section) — not an orchestrator follow-up.

---

## §H. Anti-kabuki rules (binding — the heart of this template)

A handoff or its resulting PR is **rejected at review** if it does any of the following. Each is a
real, observed failure mode, not a hypothetical.

> **This section cuts ceremony, not proof — it is not a permission slip.** Every rule below targets
> *over*-production (governance theater). The opposite failure — **under-proof** — is rejected just as hard,
> and **citing an anti-kabuki rule to justify skipping proof is itself the non-conformant route this section
> exists to catch.** The load-bearing floor is **never** kabuki:
> - **Run the check and paste its real output.** Never assert, summarize, or fabricate a result you did not
>   produce. A "self-test / scan green" claim means the command **actually ran green on this branch** — not
>   that the logic "looks right." (Both happened on the CI track: a 0-byte checker with fabricated proof cases,
>   and an allowlist "clean" that a one-line grep disproved.)
> - **Verify the tree, not the relayed report**, and **do not merge before DA clearance** on any authority /
>   gate / PROBATION rung (spine, above).
> - **The maintained, data-driven, self-testing CI doctrine-scan is the one *sanctioned* guard layer** — the
>   mechanized rung-3 of the admission ladder. Rule 2 forbids *new ad-hoc greps in your PR*; it does **not**
>   forbid adding a reviewed `scripts/ci/scans.tsv` / `allow/*.txt` entry under the §4 rigor in
>   [`ci_screening_surface.md`](ci_screening_surface.md). Do not cite rule 2 or D8 to argue the CI screen
>   shouldn't exist or that you may skip running it.
>
> Kabuki is doing governance work *instead of* the feature. Skipping proof is doing **neither** — and is worse.

1. **Tests a condition guaranteed by the type system or hard-errored at admission**, beyond one
   representative negative test per rejection class. Batteries enumerating malformed variants are kabuki (A).
2. **Authors bespoke guard scripts that restate the scope diff.** One forbidden-path `git diff` check + the
   existing semantic-free test is the ceiling. Five greps for the same boundary is theater (A, D8).
3. **Triple-updates docs** (ladder + evidence index + a separate production-log essay). One results doc,
   one index line, one status row (§7).
4. **Hand-authors the full implementation inline.** Specify the contract + invariants + acceptance; the
   implementer owns the code and may improve the factoring within the fence (B).
5. **Lists more reading than the rung touches.** Spine + design file + rung-local files only (C, D7).
6. **Adds a docs/closeout rung in front of a buildable feature.** Closeout rides with the impl PR (B).
7. **Resolves a "validate/govern X" requirement into a new registry/table/preflight artifact** instead of
   an admission-layer hard error (D8, the noun-for-verb fence).
8. **Drops the context spine or removes its enforcing pointers / retained norms.** The compressed spine is the
   anti-drift anchor; every handoff carries it, but it does not re-expand mechanically enforced doctrine (C).
9. **Invents a new resolution mechanism** while an RF arena / overlay / EML gadget / JIT-EML shader
   suffices (D1, D2).
10. **Pads the evidence doc with empty or mission-restating sections.** Sections carry signal or are cut.
11. **Produces inert scaffolding** — a file that *looks like* a gate but enforces nothing (unwired config,
    empty placeholder, dead allowlist). Remove, don't annotate (constitution §0.6 binding 6).

> Litmus: **does this line prove or build the feature, or produce a governance artifact about it?** If the latter, cut it.
