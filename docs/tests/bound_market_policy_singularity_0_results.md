# BOUND-MARKET-POLICY-SINGULARITY-0 — ingress qualification STOP

Status: STOP / PROBATION / preliminary proof only / DA-review-pending / OPEN / UNMERGED.

Dispatch: [5587411392](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5587411392).
Exact base: `3d4f87deb43398b9a01cc52c38f663141f889c39`.
HD-RECEIPT: 8686242af710
ORIENT-RECEIPT: da3e92f7e86f
role: coding
orientation_rule_stamp: 5127541ebd64b1ab
orientation_digest_sha: 0c167ca5128bf1b49604f4195548d42269fa36f0f3fefcc4fadb976967957c6e

The required F1 runtime falsifier cannot yet reach economics on the exact base.
`SimSession::open_with_clearing_posture` refuses the existing fifth pin before
the scenario is installed. The unchanged frozen 15.8 session witness independently
refuses at `SimSession::open` with the same required/observed pair.
These are ingress failures, **not** the mandated F1/F2 score/precedence REDs.
No production edit or E8 roll has been made; F2/F3 and the remedy remain pending.

## Actual runtime evidence

Both commands exit 101 at session open, before any economic settlement:

```text
cargo test -p simthing-workshop --test bound_market_policy_singularity_0 constant_score_preserves_resident_precedence_in_both_postures -- --exact --nocapture --test-threads=1
cargo test -p simthing-workshop --test resident_session_integration_conformance_0 ordinary_session_identity_half_and_registry_permutation_cross_real_generations -- --exact --nocapture --test-threads=1

ResidentClearing(LiveHead(UnqualifiedAdapter {
    required: 1528885755714777159,
    observed: 3790907948833039218
}))
required hex: 0x1537_b17e_9388_b047
observed hex: 0x349c_02b1_c265_e372
```

The active build enables `EML_RESOURCE_PROFILING`, matching the previously
qualified Workshop feature tuple. Compiler provenance is unchanged:
`rustc 1.95.0 (59807616e 2026-04-14)`, `x86_64-pc-windows-msvc`.
`cargo check -p simthing-workshop --test bound_market_policy_singularity_0`
passes. The staged test exercises actual sessions, both postures and
step/run/record; its live-basis and G/U assertions have not yet been reached.

## Read-only byte diagnosis

The E8 build script hashes the raw bytes of 32 named components. Comparing the
archived fifth-roll build provenance with the current generated provenance
identifies exactly two component changes:

| Component | Qualified CRLF FNV | Current LF FNV |
| --- | --- | --- |
| `crates/simthing-driver/src/resident_clearing_runtime.rs` | `37a5e71bcab37c03` | `df7b6d8b5fea19b2` |
| `crates/simthing-driver/src/growth_entitlement.rs` | `3a79e18e2ae1b2f3` | `8e25b4b92fabb6a4` |

Current bytes of both files equal their committed Git blobs. `.gitattributes`
specifies `* text=auto eol=lf`. Production source/build/Cargo diffs from the
previous coding return `d86bc175` to this base are empty.

An **in-memory-only** LF-to-CRLF transformation of those two files reproduces
their archived component hashes exactly. Combining those reconstructed bytes
with the other 30 unchanged components reproduces the complete fifth-roll
semantic bundle `94b8c7b853a6af15`; the current LF bundle is
`e8df874da7e17163`. No component file was rewritten for this diagnosis, and no
fingerprint, generated build data or qualification check was patched.

The recorded fifth pin therefore depends on two historical working-tree newline
representations that differ from this canonical checkout. The claim here is
about raw qualification bytes, not a discovered change in clearing arithmetic.

## Required orchestration/DA disposition

The HD requires two actual-session semantic RED commits **before** production
edits, and permits the sixth fixed-two-literal roll only **after** final semantic
edits. The two Driver files above are outside its production surfaces. Restoring
their historical CRLF representation would change sealed build inputs despite
an empty normalized Git diff; coding has not self-authorized that operation.

One concrete possible disposition is to authorize restoration of those exact,
hash-verified historical bytes solely for baseline F1/F2 reproduction, followed
by restoration of the committed LF bytes before the ruled semantic edit and
the one final sixth roll. Alternatively, orchestration/DA must amend the ingress
and E8 sequencing. This report requests that decision; neither route has been
executed. No build normalization, guard change, extra pin or interim roll is
proposed as an implicit implementation change.

F1/F2 semantic RED SHAs, F1/F2 GREEN, F3, the production gate deletion and the
sixth E8 packet remain **not obtained**. Full-workspace zero-RED is not claimed.
The committed-head Agent Scan, structural results and actual hosted artifacts
are reported by exact SHA in the accompanying Board/PR return.

The only other work is the required F1 inventory row, one actual anchor-query
reach row, and the bounded historical-header correction in the 15.12 evidence
document. Its semantic proof sections remain unchanged. All 48 rendered anchor
ACKs are carried in the return: 47 previously read hashes remain identical; the
changed ladder anchor was queried and its full delta read.
