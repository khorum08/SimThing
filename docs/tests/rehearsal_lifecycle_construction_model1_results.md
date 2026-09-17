# Construction Leaf A — complete terminal Model-1 result

**PASS-MODEL-1 / PROBATION / proof-present / review-pending.**
Model 2 remains CLOSED. This coding result does not graduate rung 2.1 or authorize
merging #2062.

Authority: dispatch [5722222332](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5722222332),
release [5722220973](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5722220973),
and DA [5722135739](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5722135739).
The same branch was rebased once onto exact released master
`2e334ddd6718c332d8c29736c1c9166bbb4c97c5` (#2072). Historical head
`aaf27a630a25dd8fc16404cd192b614043f1b3f9` is retained at
`codex/0088-construction-pre-2072`; all earlier evidence and witnesses remain intact.
One coding lane, no subagents. This resume adds evidence only.

## Executed contract

The unchanged complete suite independently passes on rebased code head
`4f2f1fccfcf44de43a995c5112c7cd65b4cd7a9c`: **14 Leaf A tests plus four
supplemental production-path tests, zero failures/ignored/filtered tests**.
The final documentation-bearing head, hosted artifacts, and fresh final-body
clearance are bound in the PR body and Board return; that head reruns the same
complete sequence. [Raw execution](rehearsal_lifecycle_construction_model1_raw_results.md)
preserves every emitted runtime diagnostic from this first complete rerun.

| Obligation | Executed evidence |
|---|---|
| Canonical ingress first | The production admission/seal path accepts the exact E8 pin before allocation, WIP or recipe tests execute. |
| Opposing allocation | Two ordinary resource arenas favor opposite projects; ownership follows the actual separate allocation results. |
| Owned WIP / exactly-once | Incomplete inputs remain explicitly owned and recoverable until the ordinary recipe consumes a complete pair. No incomplete input becomes irreversible build credit. |
| Material disposition | Residual A/B plus consumed-pair receipts balances each resource's authored total supply (4, then 7, then 10). Receipt records consumption, not a residency reservation or a substitute product. Funded refusal/cancellation explicitly scraps consumed inputs; no refund is promised or manufactured. |
| Placement/refusal/restart | Ordinary recipe consumption precedes structural placement. Fragmented capacity causes typed contiguous-extent refusal. Fixed authored source pulses subsequently fund a fresh candidate; the refused candidate never replays. |
| Cancellation and extent release | Ordinary identity-based Remove retires the funded subtree and its committed placement. The next fresh product uses the exact released extent with a new grant identity. |
| Repeated full products | The 14-generation matrix reaches two fresh successful completions at G7/G12 in one continuing session, after the initial refusal. Product/component/overlay identities differ. Properties, overlays, membership, child structure, parent relations, bindings and GPU observations 18/19 are asserted. |
| Independent determinism | 32 cases: P/Q crossed with independently reversed children, arenas, recipe order, and physical prefix spacers. Actual host rows differ ({1,2} versus {5,6}); economic/live-count/facility traces match exactly. Every case cancels its first product and reuses its extent for a fresh second subtree. |
| Integrated identity negatives | Stale/foreign binding tables refuse BindingTableStale before GPU work; bound participant removal refuses BoundIdentityRemapped with the identity; a duplicate resident product refuses GrowthEntitlement with its grantee while preserving its original placement. |
| Frozen threshold definition | All four changed-threshold negatives refuse through #2072's admitted law; no second product is created. Identical clear/re-register controls remain healthy. Details below. |
| Source-generation exactly-once | Healthy integrated traces retain sealed source generations [1,5], while quiet boundaries leave facility ordinal unchanged. Supplemental sparse/contiguous controls reject duplicate consumption and regressing source generations without altering the accepted window. |
| Fail-stop and immutable retries | Twelve stale/foreign preflight attempts preserve values, clock, schedule and facility ordinal. Twenty-four post-touch retries preserve values, clock, schedule, bindings and facility ordinal and remain ExecutionIdentity-faulted. |

The terminal table contains **46 sessions**: 32 independent-order cases, eight
identity-refusal cases, two unchanged-alert controls, and four changed-threshold
negatives. All pass. The earlier recipe/lifecycle, funded cancellation, canonical
multi-remove, placement/restart and ample-capacity control witnesses also execute;
the terminal table is not a replacement for them.

## Closure of the last substrate defect

Tick-zero commitments remain frozen against receipt thresholds 0.5 and 1.5.
The unchanged tests use the public clear/re-register alert API to alter the second
threshold; ordinary Phase-5 GPU mint supplies the real crossings. The plan is not
recompiled or replaced.

- **Early 0.75, P and Q:** at G2, source generation 1 carries post-value 1 and
  threshold bits `0x3f400000`, versus admitted 1.5 bits `0x3fc00000`. Dispatch
  refuses; both products are absent and facility ordinal remains 0.
- **Post-start 1.25, P and Q:** at G6, source generation 5 carries post-value 2 and
  threshold bits `0x3fa00000`, versus admitted `0x3fc00000`. Dispatch refuses;
  the first product remains, the second is absent, and facility ordinal remains 1.
- Each of those four touched generations then refuses three retries without
  mutating the captured state. The two identical-rebuild controls retaining 1.5
  create only the first product and continue normally.

The kernel error is `ActionBandExecutionError::FrozenThresholdDefinitionStale`.
The ordinary-session wrapper records it as
`ActionBandIngress(Dispatch(Gpu("...admitted threshold bits...")))` because the
GPU dispatch boundary carries its display string. The integrated tests assert
typed ingress refusal and no second product; the raw diagnostics independently
identify the actual admitted-definition error and bits. The supplemental
`frozen_threshold_definition_is_authority_not_registration_index` test directly
asserts the inner enum and proves identical rebuild, reorder and additive unbound
churn remain lawful while changed definitions/remaps refuse.

No new semantic obligation, synthetic crossing, CPU threshold comparator, manual
generation restamp, alternate facility, readback-conditioned policy or
construction-specific allocator is introduced. Existing negative-only stale/
foreign allocator plants remain adversarial setup, not a healthy execution path.

## Reproduction and scope

On the reference NVIDIA GeForce RTX 4080 Laptop / Vulkan adapter, execute in order:

```text
cargo check -p simthing-driver --features simthing-gpu/eml-resource-profiling
bash scripts/ci/agent_scan.sh --base 2e334ddd6718c332d8c29736c1c9166bbb4c97c5
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_ingress -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_discriminator -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_wip -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_recipe -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test actionband_overlay_actuation_0 -- --nocapture --test-threads=1
```

Recipe source SHA-256 remains
`5ab7655fd70d335906bb7000442278fa4b5d1b68df433289532157361539a805`.
The thirteenth E8 pin remains required=observed `ec5a2a30afaee795`, bundle
`9c2091194366b23b`; this release requires no roll. Cargo check passes with existing
warnings. Local scan/test budget passes with zero failures and zero INSPECT flags;
final head/run-specific details live in the PR body and Board return.

This resume changes only this report and its raw companion. The full PR retains
four driver test files, twenty evidence documents, and fourteen ordinary AUDIT /
delete-at-closeout inventory rows. Production src, E8 components/pin, Cargo,
scenarios, Studio, ClauseThing, gate/class/anchor/workplan/orientation, exceptions,
triage and budget exemptions are untouched. Historical RED documents describe
their original revisions and are not overwritten by this successor PASS.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: e5b941f898d53f86a8be2e19afba51d52a41dda06f69dff4d3b52817ffed89c6
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: actionband-crossing-surface@623db585f145
ANCHOR-ACK: actionband-determinism-lifecycle@6306c484732c

Receipt carried; governance sources and construction contract unchanged.
