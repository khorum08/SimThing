# Resident-clearing qualification — NINTH E8 roll (inherited byte-invalidation)

DA increment per Orchestration relay `5680736076` (Astra blocked return `5680396235`,
draft witness #2062 held OPEN/UNMERGED). One bounded D-scope seal repair between
rungs/leaves; no construction law, Model-1/Model-2 decision, numerical law, component-list
change, or new capability. A pin repair is never evidence for Model 2.

## Cause (verified independently at adjudication and again here)

PR #2061 (approved expiry-debt disposal, DEEP-TREE `5675445722`) deleted expired TEST
functions inside three E8-hashed sources — `crates/simthing-driver/src/arena_allocation_plan.rs`
(1 fn), `crates/simthing-kernel/src/accumulator_op/session.rs` (3 fns),
`crates/simthing-driver/src/session.rs` (3 fns) — 319 deletions, ZERO additions, no
production statement touched. The seal hashes full source bytes by design, so the canonical
byte change lawfully invalidated the pin although production semantics did not change.
Astra's read-only tuple recomputation reproduces the current observed fingerprint from
current canonical blobs and reproduces the old pin exactly when only the old Git semantic
bundle is substituted; the RED below re-derives the same observed value on the reference
machine. The observed value is evidence, not self-authorizing mutation.

- Prior qualified semantic bundle (@`dae7ab1b`): `dfcb52ff6a412e2e`
- Current canonical bundle: `c6b6a70c64f7ae81`

## Old-pin refusal (RED, required first)

Current canonical master `ffa5b944`, reference tuple (NVIDIA GeForce RTX 4080 Laptop GPU /
Vulkan / NVIDIA 595.79; rustc 1.95.0; `simthing-gpu/eml-resource-profiling`):

```text
RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 6fe1d809c05ee0f4
thread 'resident_clearing_parity_terminal_referee' panicked at
crates\simthing-workshop\tests\resident_clearing_parity_0.rs:1732:5:
assertion `left == right` failed
  left: 8061962344363647220   (observed 0x6fe1_d809_c05e_e0f4)
 right: 8009790104337190185   (pinned  0x6f28_7da9_8675_0d29)
test result: FAILED. 0 passed; 1 failed
```

The seal refusing the amended bundle before economics, as designed — identical decimal
pair to the ordinary-`SimSession::open` refusal in Astra's blocked witness
(`ResidentClearing(LiveHead(UnqualifiedAdapter { required: 8009790104337190185, observed:
8061962344363647220 }))`).

## Roll

Exactly the two existing literals, identical observed value; no third site, no
comparator/ABI/component-list/`build.rs` change; #2061's deletions preserved (no proof
restored to recover the prior hash):

- `crates/simthing-gpu/src/resident_clearing_runtime.rs`
  `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT` `0x6f28_7da9_8675_0d29` → `0x6fe1_d809_c05e_e0f4`
- `crates/simthing-workshop/tests/resident_clearing_parity_0.rs`
  `QUALIFIED_RECORD_FINGERPRINT` `0x6f28_7da9_8675_0d29` → `0x6fe1_d809_c05e_e0f4`

Pin chain: `0xb295_851d_f402_d50b` → `0x42a7_338e_0b42_b5be` → `0x6f28_7da9_8675_0d29` →
`0x6fe1_d809_c05e_e0f4`.

## Post-roll qualification (reference machine; equal to the eighth-roll floor)

- `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
  → `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 6fe1d809c05ee0f4`; 1 passed
  (fingerprint equality + full mutant matrix inside the referee).
- `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_score_and_bands_0 -- --nocapture --test-threads=1` → 3 passed.
- `cargo test -p simthing-gpu --features eml-resource-profiling resident_clearing_runtime -- --nocapture --test-threads=1` → 4 passed.

## Clean-checkout proof

- commit: `4d4c50c13a8be5d925b3f5d2ac579bc3013e10ac`
- command: fresh `git clone` at that exact commit (canonical LF checkout — repo
  `.gitattributes` `* text=auto eol=lf` and `core.autocrlf=false` both verified), sibling
  directory `simthing-e8-roll9-verify`, never %TEMP%;
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 6fe1d809c05ee0f4`; referee
  `1 passed; 0 failed` — pinned fingerprint reproduced and the qualified witness admitted
  from the fresh canonical-LF clone.
