# EML-PRIMITIVE-DOMAIN-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.10)
- Status: **COMPLETE / DA-GRADUATED — merged #1618 @ 35dcf43a** (DA independently planted both the unguarded-admission and guard-as-certificate defects; each RED)
- Dispatch base: `0770af0a559d603aac9e716f937435666790b92d`
- Branch: `codex/eml-primitive-domain-0`
- ORIENT-RECEIPT: `6af1884543b0`
- Orientation digest: `13976629af6b908cc0d82c6a839ec910d5a2b9b10a8f12ce2b7deccdacf8f06e`
- Dispatch: Board comment `5173620090`; binding inline handoff `5173584826`
- Expected route: `DA-RESERVE(gate-wiring)`

## Landed door

`PrimitiveDomain` is a kernel-minted, private-representation binary32 interval with explicit ordering
across the negative and positive sign halves. Its endpoints and special-value policy travel through
the exact-determinism key onto `ExactPrimitiveAdmission`. The former two-value domain policy is now
only the special-value component of that interval.

Call sites have two distinct sealed paths. A bounded `SubFieldSpec` range is checked as a subset of
the primitive domain and mints a range certificate. An authored closed-vocabulary
`CLAMP_BOUNDED`/`MAX`/`SELECT` guard mints a different semantics token after its authored output
range is checked. The latter cannot be substituted for the former. The admission token adds zero
runtime nodes; an absent obligation hard-errors with its source span.

No interval analyzer, primitive implementation, cost/taxonomy change, resource-class change,
interpreter path, or successor-rung stub was added.

## Biting proofs

| Proof | Result |
|---|---|
| Domain seal and order | private-field `compile_fail`; `[-1,+1]` admits both signed zeros; `[+1,-1]` rejects |
| Unguarded call | exact `41..57` admission error; bypass mutant is RED |
| Shape separation | authored clamp succeeds only as guarded semantics; guard-as-range-certificate rejects at `41..57`; substitution mutant is RED |
| Shape-one cost | admitted range-certified path reports zero added runtime nodes |
| Zero primitive census | fresh and exercised generic doors retain `admitted_count() == 0` |
| Zero vocabulary census | `CLOSED_OPCODES` block SHA-256 is unchanged at `a670c17346e4f629779cb68d4e3477bff0d5f9af3d2a5b188fe57cea74ec4d32`; `eml_nodes.rs` and every `*.wgsl` path have an empty diff from dispatch base |
| Public-surface fence | no kernel allowlist or sanctioned-surface edit; new call-site proof types stay behind the existing admitted door |

## Verification

```text
cargo check -p simthing-kernel --lib: PASS
cargo check -p simthing-kernel --tests: PASS
cargo test -p simthing-kernel --lib exact_primitive_: 3 passed / 0 failed
cargo test -p simthing-kernel --doc: 43 passed / 0 failed
bash scripts/ci/test_inventory_drift_check.sh: PASS (1071 rows / 1071 discovered)
bash scripts/ci/test_inventory_drift_check.sh --prove: PASS
bash scripts/ci/doctrine_scan.sh: 0 reliable failures; expected whole-tree heuristic INSPECT only
doctrine-scan always-on gates: PASS; advisory exit-proof/dead-export findings recorded for PR review
doctrine-scan selftests: PASS (including relay 36, closeout 101, doctrine falsifier battery)
git diff --check: PASS
```

The dispatch base also lacked the `TransformOp` test-module import used by `overlay_prep`; one import
was restored so the kernel unit-test binary could compile. This is the only non-door source edit.

## Posture

No clearance, merge, pointer movement, primitive admission, 5.11 implementation, or successor-rung
work is claimed.
