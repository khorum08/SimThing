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

## Context spine — restated verbatim in every handoff (the anti-drift anchor)

> This block is **non-negotiable and never abbreviated away.** It is the cheapest insurance against the
> core principles (STEAD, RF arenas, semantic-free sim) drifting out of a low-context agent's window.

- **Everything is a SimThing.** New behavior = SimThings + properties + overlays + `AccumulatorOp`
  registrations on the one recursive tree — never a subsystem beside it, never a runtime `match kind`.
- **All conflict / opportunity / ambition / diplomacy is resource flow:** `accumulate → reduce up →
  settle → mask/disburse down → threshold crossings fire decisions`. No combat/economy/AI/pathfinding engine.
  **There is one authoritative resolution path — this one.** Do not write resolved state or emit a decision
  from a CPU side-computation; route it as an `AccumulatorOp` / overlay registration or a threshold→`EmitEvent`
  crossing. If the compiler fights you trying to mutate state or emit from outside the kernel, **that is the
  boundary, not a bug to work around** — escalate to the DA instead. (The write/emission/participation seals
  that make this *uncompilable* rather than merely directed are the AS-9+ cluster; until they land this is a
  binding directive, not yet a type guarantee.)
- **Decisions are GPU-resident threshold crossings (FIELD_POLICY), never a CPU planner.**
- **The map is the Movement-Front automaton (STEAD):** bounded-horizon falloff (P1), one shared stencil
  (P2), attractor/threshold projection (P3). The front *is* the route; movement is gradient-following
  reparenting. Exact-magnitude gates route through Candidate F.
- **`simthing-sim` and WGSL are semantic-free.** Behavior is EML opcode-stack data over one interpreter;
  semantics compile away at admission. Exact claims carry bit-exact CPU-oracle parity.
- **Substrate ladder before any new primitive (D2):** (1) RF arena → (2) overlay on a column → (3) EML
  gadget tree → (4) JIT EML→WGSL shader → (only then, Tier-2, with parity) a new opcode/kernel/role.
- **Enforcement is admission *behavior*, not a governance *artifact* (D8).** "Validate X" means a spanful
  hard error in the decoder/admission layer — never a new registry/preflight/validation doc that restates it.
- **Doctrine as type, not prose (core §1.2 — the admission ladder).** Encode an invariant at the highest
  rung that can express it: **type boundary** (illegal state uncompilable) > **admission hard-error** >
  **guard scan** > **prose**. Do not narrate in prose or a guard test what a type should make
  unrepresentable; a guard scan that exists only because a type didn't is a promotion target. Spend the
  budget Rust frees on type/admission boundaries + semantic conformance — never on process ceremony.
- **The kernel owns authority; seals don't cross crate boundaries (core §1.2.1, constitution §0.9).**
  Authoritative state/effects live in `simthing-kernel`; consumers get a read-only view + sanctioned doors.
  **Mint authoritative types in the crate that privately owns their source of truth; never re-seal across a
  crate boundary with a token** (cross-crate `pub`/`#[doc(hidden)]` is capability-for-everyone). The
  irreducible residue (CPU-oracle twin, WGSL, inert utilities) is a **named tripwire catalogue, not a gap** —
  routing through it is deliberate circumvention; flag it (`seal-residue-risk`). An artifact that looks like a
  gate but enforces nothing is deleted, not annotated.
- **CI doctrine-scan is the automated DA scan layer (core §1.2.1, constitution §0.9.5).** A clean **RELIABLE**
  scan is **DA-equivalent** for that scan — trust it, don't re-verify; `FAIL` is a **HOLD**; `INSPECT` routes
  to §1A triage (bounded, cost-symmetric, spam-bounded), never a silent pass. An allowlist edit is a
  **deliberate, reviewed widening of a sanctioned door**, never a scanner-edit dodge. A scan is itself
  residue: promoting its invariant to a type/admission boundary **retires the now-redundant scan in the same
  PR** (§1.2's promotion-target rule, mechanized).
- **No rung touching PROBATION / authority / gate state merges before DA clearance.** A truthful corrective
  self-report of a breach may be accepted on its merits; it is never precedent for skipping clearance again.
  **Verify the tree, not the relayed report** — a clean CI check or a proof transcript pasted into a PR body
  is a claim, not a substitute for the DA independently confirming the branch.

If the change cannot be expressed within these lines, **stop and escalate to the DA** — do not special-case.

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
- **Merge-hold rule** — a rung whose handoff or diff touches PROBATION status, kernel/CI authority, or gate-state semantics **does not merge before DA/Owner clearance**, full stop. **Do not trust relayed proof — verify the tree**: the implementer's pasted transcript is a claim; the DA (or any reviewer standing in) confirms it against the actual branch before treating the rung as cleared.

**The Admission-Substrate Amendment Valve (owner-gated; do not work around seals).** The kernel/admission
seals (AS-1–8B + the kernel track) are owner-gated. If you hit a seal that genuinely blocks the rung, you
have exactly two moves: **(a)** if this handoff sets `admission-amendment-request: allowed`, you may *request*
the valve — surface a written request for **Owner / Exec-DA** approval, stating why it cannot be a
registration / EML gadget / overlay within the existing seal and whether it is add/repair/suspend; **(b)**
otherwise, **escalate the blocker to the DA.** You never self-grant, never suspend a seal yourself, and never
build a sidecar around it. The valve opens only on owner interrogation + approval, recorded as a Deviation
with a greppable marker (full protocol: the kernel track §3A).

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

## 6. Proof — minimal and load-bearing (the anti-battery rule)

Every test must name the **regression it catches.** Apply this triage:

- **Behavior that can actually regress** → test it (real lowering, GPU==CPU-oracle bit-exact, settle-then-bubble, determinism, no-panic-on-bad-input, cache-hit-no-recompute).
- **A rejection class** (malformed input the admission layer hard-errors) → **one** representative negative test that the span-error fires. **Not** an enumeration of every malformed variant.
- **Guaranteed by the type system / typestate / the firewall** → **zero tests.** You do not test that an enum cannot hold an illegal variant.
- **Reuse existing oracles/guards** (the semantic-free guard, the arena-allocation oracle, the canonical save/load battery). **Never re-derive** a guard the repo already runs.

List the load-bearing tests for this rung, each with its one-line justification:

```
<test_name>            — catches: <the specific regression>
…
```

> GPU legs skip cleanly without an adapter. `cargo test --workspace` is **never** run.

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

## 11. Response format

```
Status:
PR / Merge:
What changed:
Load-bearing proofs (+ what each catches):
Scope Ledger:
Conformance (spine/D-directives held):
Known gaps / next:
```

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
8. **Drops or abbreviates the context spine.** The spine is the anti-drift anchor and is always verbatim (C).
9. **Invents a new resolution mechanism** while an RF arena / overlay / EML gadget / JIT-EML shader
   suffices (D1, D2).
10. **Pads the evidence doc with empty or mission-restating sections.** Sections carry signal or are cut.
11. **Produces inert scaffolding** — a file/module/config that *looks like* a gate/capability/completed
    structure but enforces or does nothing (unwired config, empty placeholder, uncalled stub, dead
    allowlist, a scaffold for a feature that never landed). These aggregate into a false appearance of
    completeness and become **handwave vectors** (a later agent cites the file's existence as compliance
    that isn't there). The real thing is created when *wired*; an artifact that looks like a gate but isn't
    one is **removed, not annotated** (constitution §0.6 binding 6). Delete inert scaffolding you encounter.

> The litmus, every time: **does this line prove or build the feature, or does it produce a governance
> artifact about the feature?** If the latter, cut it.
