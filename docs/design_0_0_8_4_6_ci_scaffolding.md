# 0.0.8.4.6 — CI Scaffolding: the Doctrinal Tripwire Layer

> **Status: PROPOSED (production track, 2026-06-29, owner-directed).** An infrastructure sub-track in the
> 0.0.8.4.x lane (after the closed `simthing-kernel` track 0.0.8.4.5), sequenced **before** the 0.0.8.5
> Terran-Pirate track. *(Owner said "0.0.8.5"; that number is held by Terran-Pirate, so this is numbered
> 0.0.8.4.6 to avoid collision — bump TP to 0.0.8.6 and renumber this to 0.0.8.5 on owner request.)*
>
> **Purpose.** Automate the repeated DA doctrinal scans into a free, fast, public-repo GitHub Actions layer
> so that **agents and the orchestrator may treat a clean CI result as "the DA ran these scans"** — and a
> flagged result routes to DA for the judgment a grep cannot make. The executable tests (builds, GPU parity,
> Studio) stay **local**, by architecture. Two tracks: **A (now)** the grep-only tripwire layer + its own
> validation; **B (deferred)** a local executable harness that emits the same report contract.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable; hold every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2 / §1.2.1** — doctrine-as-type, the admission ladder, residue-as-tripwire (this track automates the *guard-scan* rung of that ladder).
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) §0 (esp. **§0.9** — doctrine-as-type / residue-as-tripwire carry-forward; **§0.6.6** — no inert scaffolding).
3. **This file** — the 0.0.8.4.6 canonical design file.
4. [`design_0_0_8_4_5_simthing_kernel.md`](design_0_0_8_4_5_simthing_kernel.md) **§5.2** — the B1–B8 bypass-state catalogue this layer screens for.
5. [`handoff_template.md`](handoff_template.md) — the spine + the `seal-residue-risk` field (CI surfaces the same residue classes).

**Established decisions — do NOT re-derive:**

- **CI screens; it does not prove.** A grep verifies a *pattern is absent*; it cannot prove a *seal holds* (the subtle holes this session — the `#[doc(hidden)] pub` minter, "can external forge via any path" — needed `compile_fail` + DA reasoning, not grep). CI is the cheap **80%**; the seal-proof layer (Track B, `compile_fail`) and DA judgment cover the rest. A green grep-CI means *no known-bad pattern is present* — necessary, not sufficient.
- **Three verdicts, never two.** Every scan emits `PASS` / `FAIL` (hard, blocks the PR) / `INSPECT` (soft — a possible false-positive or an inherently-judgment scan; surfaces to DA, does **not** block). This is residue-as-tripwire applied to the scanner: an uncertain result is a flagged tripwire, not a silent pass or a false failure.
- **Single source of truth, runnable both places.** The scans live as one versioned script in the repo (`scripts/ci/doctrine_scan.sh`), run identically locally and in CI — the same doctrine-as-type principle (the scan is data/code, not duplicated prose).
- **The scanner must self-validate or it rots.** A scan that silently stops catching its violation is worse than no scan (false confidence — the `deny.toml` lesson, §0.6.6). The layer carries a fixture battery proving each scan still fires on a known-bad input and stays quiet on the known false-positive traps.
- **Public repo = free + unlimited standard runners.** Linux only; no toolchain, no build, no GPU, no Bevy — pure text. Wall-clock seconds.

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
```

- **PASS** — clean. The orchestrator treats this scan as "DA ran it, no finding."
- **FAIL** — a *reliable* hard scan hit a definitive violation. Blocks the PR. Equivalent to a DA HOLD on that condition.
- **INSPECT** — a *soft* scan hit, OR a hard scan whose hit lands in a known false-positive zone. **Does not block**, but is surfaced to the DA as "a grep can't adjudicate this — look." The DA (human or Opus) reviews INSPECT flags exactly as it reviews residue tripwires.
- **scanner self-test** — if any hard scan stopped firing on its fixture (the scan rotted), the **whole report is FAIL** regardless of PR contents. A scanner that can't catch its own known-bad cannot certify anything.

**Reliability legend (the DA must know what to trust):** each scan is tagged `RELIABLE` (a hit is a real violation — CI-blocking) or `HEURISTIC` (a hit may be a false-positive / needs judgment — INSPECT-only). The DA consumes FAILs as findings, INSPECTs as "closer look," and ignores nothing silently.

---

## 2. Track A — grep-only CI doctrinal tripwire (NOW)

| Rung | ID | Scope | Recipient | DoD |
|---|---|---|---|---|
| 0 | `CI-A-TRACK-OPEN-0` | This doc + evidence row. | Haiku/Sonnet | doc lands. |
| 1 | `CI-A-SCAN-SCRIPT-0` | `scripts/ci/doctrine_scan.sh` — the seed scan set (§6), each a named scan with pattern + target glob + **false-positive exclusions** + severity (`RELIABLE`/`HEURISTIC`) + doctrine-ref. Emits the §1 report. Runnable locally (`bash scripts/ci/doctrine_scan.sh`). | Cursor/Grok | script runs locally; emits the report; exit non-zero **only** on a hard FAIL (INSPECT exits zero). |
| 2 | `CI-A-FIXTURES-0` | `scripts/ci/fixtures/` — tiny known-bad inputs (one per RELIABLE scan: a `pub fn -> &Buffer`, a `for_kernel_readback` minter, a `type ColumnIndex = usize`, a missing-forbid-unsafe crate stub) **and** known false-positive traps (a jomini-style `write_*`, a `studio_antialiasing`-style module name, a `pub(crate)` sealed accessor, a semantic word in a comment/doc). | Cursor/Grok | fixtures exist; each known-bad maps to exactly one scan. |
| 3 | `CI-A-SELF-TEST-0` | `scripts/ci/doctrine_selftest.sh` — runs each RELIABLE scan against its known-bad (must FAIL) and against the trap corpus + clean master (must NOT FAIL). The **high-leverage ubuntu executable dimension** the owner approved: this self-test runs *in CI* before the PR scan, so a rotted scan fails loudly. | Cursor/Grok | self-test green on current master; flipping any scan pattern to a no-op makes the self-test FAIL (proven). |
| 4 | `CI-A-WORKFLOW-0` | `.github/workflows/doctrine-scan.yml` — `ubuntu-latest`, no toolchain: checkout → run self-test → run PR scan → publish the report (job summary + uploaded artifact the DA reads). Hard FAIL fails the check; INSPECT annotates. | Cursor/Grok | a known-bad PR fails the check; a clean PR passes; an INSPECT-only PR passes with the flag visible in the report. |
| F | `CI-A-CLOSEOUT-0` | Record the **DA-equivalence contract** (agents treat PASS as DA-ran; FAIL as DA-HOLD; INSPECT → DA) in this doc + a one-line pointer in the handoff template so handoffs cite "CI doctrine-scan = the automated DA scan layer." | Opus/Owner (DA) | contract recorded; reliability legend published; track CLOSED. |

**Validation battery (Track A's own proof — runs locally, and the self-test also in CI):**
1. **Positive control** — scan current master → zero hard FAILs (no false-positive on the real tree).
2. **Negative control** — scan each known-bad fixture → the matching scan FAILs (the scan actually catches its violation).
3. **False-positive corpus** — scan the trap fixtures → no hard FAIL (exclusions work; traps land as PASS or INSPECT).
4. **Rot test** — neutralize a scan pattern → the self-test FAILs (the scanner can't silently degrade).

## 3. Track B — local executable validation harness (DEFERRED)

> Opens after Track A lands and a consumer needs it (e.g., 0.0.8.5 rungs wanting seal-proof on every change).
> Executable tests stay **local** (Rust toolchain + the owner's GPU/Windows), per architecture.

Sketch (not opened here): `scripts/ci/doctrine_tests.sh` runs the **seal-proof** layer locally — `cargo fmt
--check`, `cargo test -p <crate> --doc` (the `compile_fail` suite that *proves* the seals), targeted
`cargo check -p`/`test -p`, and the parity tests — emitting the **same §1 report contract** with tripwire
tags specific to executables:

- **GPU-skipped** → `INSPECT` (the seal/parity wasn't fully verified here; the owner's machine must confirm) — never a silent PASS.
- **flaky / perf-variance** → `INSPECT` with the run band (e.g. the +49% single-run noise we saw needs a multi-run).
- **compile_fail proven / parity bit-exact** → `PASS`.

Track B's value: the DA consumes build/test/seal-proof results in the *same* tripwire-tagged form as the grep
layer, so "the test batteries were executed by the DA" holds for the executable dimension too — just run on
the machine that can actually execute them.

## 4. Honest residue / non-goals

- **Grep cannot prove a seal holds** (§0 decision). The subtle forge vectors stay covered by Track B's `compile_fail` + DA reasoning. Do not let a green Track A imply a proven seal.
- **HEURISTIC scans will have false-positives** by nature (semantic words, raw indices, kind reads can be legitimate). That is *why* they are `INSPECT`, not `FAIL` — surfacing-not-blocking is the correct posture; tightening a heuristic into a reliable type/admission boundary is the §1.2 promotion path, tracked separately.
- **No new commercial tooling / MCPs.** Pure shell + grep + GitHub Actions on a public repo. Free, instant, no learning-curve tax.
- Track A does **not** build, test, run GPU, or touch `simthing-mapeditor` (Studio/Bevy) — those are local-only.

## 5. Practical notes — the seed scans (reuse the DA's tuned greps)

The exact patterns the DA ran this session, with the false-positive exclusions already tuned. `CI-A-SCAN-SCRIPT-0`
encodes these; each carries its severity. Target `crates/**/src` (not `tests/` for HEURISTIC scans), exclude
`pub(crate)`, and exclude `compile_fail` doctest blocks + `//` / `///` / `//!` comment lines where the pattern
is illustrative.

**RELIABLE (a hit = violation → FAIL):**
- **B3 authoritative handle escaping the kernel:** `pub fn [a-z_]+\(&self\) *-> *&(wgpu::)?Buffer | ^\s*pub [a-z_]+ *: *Buffer | -> *BindingResource` over `crates/simthing-kernel/src`, **minus `pub(crate)`**. (Caught every kernel extraction leak.)
- **Named forge minters must never reappear:** `pub fn (from_boundary_delivery|for_kernel_readback|for_boundary_install)\b`. (The exact holes 0R/0R2 closed.)
- **`forbid(unsafe_code)` present** on each semantic-free crate (`simthing-sim`, `simthing-kernel`): missing `#![forbid(unsafe_code)]` in `lib.rs` → FAIL.
- **AS-5 alias regression:** `type ColumnIndex *= *usize`.
- **`deny.toml` (or any compliance-shaped stub) reappears** unwired → FAIL (§0.6.6 — the inert-scaffolding lesson).

**HEURISTIC (a hit = `INSPECT`, possible false-positive → DA looks):**
- **Raw `data[N]`:** `\.data\[[0-9]+\]` over `crates/**/src`, **minus** `raw_lanes`/serialization sites. (FP: legit serialization byte-lanes.)
- **`match`/read on `.kind` in the sim:** `match .*\.kind | \.kind\b` over `crates/simthing-sim/src`, **minus** the kind-free runtime view. (FP: spec/authoring-layer kind, legit.)
- **Semantic game-words below the spec boundary:** `faction|combat|terran|pirate|diplomacy` over `simthing-sim`/`simthing-kernel` `src`, **minus** display-name/provenance/comments. (FP: a doc, a string, a test.)
- **Stringly channel identity:** `owner_ref *: *(Option<)?String | resource_key *: *(Option<)?String` over `crates/simthing-spec/src`. (FP: a justified serialization DTO with a recorded Deviation.)

**Known false-positive traps to exclude (the ones the DA hit and tuned out):** `crates/simthing-clausething/src/jomini/**` `write_*` (text writer, not buffer writes); `studio_antialiasing` module-name matches (not the AA *report*); `pub(crate)` (sealed = correct); a scan's own pattern-literals inside its `compile_fail`/results doc.

## 6. References

- Doctrine: [`simthing_core_design.md`](simthing_core_design.md) §1.2/§1.2.1; constitution [`design_0_0_8_3.md`](design_0_0_8_3.md) §0.6.6/§0.9.
- The B1–B8 catalogue: [`design_0_0_8_4_5_simthing_kernel.md`](design_0_0_8_4_5_simthing_kernel.md) §5.2.
- Handoff discipline: [`handoff_template.md`](handoff_template.md).
- Consumer: the 0.0.8.5 Terran-Pirate track, whose rungs consume the doctrine-scan layer as the standing automated DA scan.
