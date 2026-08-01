# GUYANG-COMPARATIVE-PROJECTIONS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8)
- Status: **STOP / DA-residue-present** after Remand 1 (`5150592918`) + Remand 1A (`5150615398`) — not exit-proof complete.
- HD-RECEIPT: `b8f9a2e4ef61`
- ORIENT-RECEIPT: `7579edb5dd6b`
- Remands: Board `5150592918`, scope-envelope `5150615398`
- Expected route: `DA-RESERVE(gate-wiring)` / DA design residue on margin↔sign-flip

## Scope envelope (Remand 1A)

| Path | Status |
|---|---|
| `scripts/ci/allow/kernel_surface.txt` | **REVERTED** — not a writable coding surface |
| `crates/simthing-kernel/src/**` production | **REVERTED** — no new public kernel doors |
| `crates/simthing-gpu/src/**` production | **REVERTED** — no new GPU re-exports |
| Comparative consumer | **`crates/simthing-driver/src/comparative_projection.rs`** only — uses already-landed field-sweep doors |
| Derived digest | Regenerated from restored sanctioned surface |

## What is load-bearing (driver consumer)

| Requirement | Result |
|---|---|
| Consumer over generic field-sweep (no new kernel authority) | PASS — driver-local compile of ordinary `FieldSweepRegistration` chains |
| Fixed derived column count independent of N | PASS |
| 1-class / opt-out / ≥2 Born | PASS (explicit helper; **not** install-path default-derived birth) |
| Dominance + exact top1−top2 + authored tie-break | PASS |
| Contest = stall magnitude under both-strong/small-margin | PASS when a stall **column is supplied**; generic Gu-Yang field-sweep does not yet emit that column end-to-end |
| Border = sign-flip only (no near-zero proxy) | **Residual proven**: with exact top1−top2, all margins ≥0 ⇒ sign-flip unreachable ⇒ no border band |
| Default-derived Anchored install birth | **NOT discharged** (no kabuki helper presented as install birth) |
| Unmodified TP integration witness | **NOT discharged** |
| CPU/GPU parity on compiled chain | PASS when GPU adapter present |

## Design residual (STOP for DA)

Exact contradiction remains:

- margin = top1 − top2 ⇒ always ≥ 0  
- border = sign-flip of margin ⇒ requires opposite signs  
- product of non-negative margins never &lt; 0  

No sanctioned signed comparative coordinate exists. Coding will not invent one.

Generic field-sweep Gu-Yang still lacks a co-located stall/`1−C/χ` output on the production chain; contest can only consume a stall column when that column is already present.

## Focused proof

```text
guyang_comparative_projections_0: driver-local consumer tests (see inventory)
gen_digest.sh --check: PASS (restored surface)
```

## Posture

Keep PR #1540 draft. No merge, pointer move, 5.9, or tiled-gather. Awaiting DA on margin/sign-flip and generic stall observable before claiming PROBATION exit-proof.
