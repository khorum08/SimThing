---
rung: STUDIO-FLEET-ICONS-0
kind: rung
track: 0.0.8.6
base_sha: da8f7035c8b876c8e81d0450b68289230ef8f87e
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "MANUAL PROGRESSION; queued behind 12.3. Orchestrator owns review + delegated merge. [OVL]: Owner screenshot closes. OWNER RIDER (binding): icons legible, appealing, easily modifiable; the primitive is a BASE anticipating future 3D / scalable-vector / sprite backends — nothing hardcoded we rip out later."
surfaces: ["crates/simthing-mapeditor/src", "crates/simthing-mapeditor/tests", "docs/design_0_0_8_6_studio_live_ops.md", "docs/tests"]
forbidden: ["movement authority / gameplay semantics (read-only 12.4 projection)", "ScenarioSpec mutation from render/UI", "new render pipeline / asset system / WGSL semantics", "hardcoded per-fleet visuals scattered through render code"]
required_checks: ["cargo-check+studio-build", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "icon-base-would-require-new-pipeline"]
---
## BUILD
- Tiny ship icon (rocket/destroyer silhouette, <=75% of base max star blur size), read-only from the
  12.4 snapshot: anchored fleets of the SELECTED owner sit RIGHT of the star pointing at it; all
  others LEFT, mirror-symmetric. In transit: ~30% along the hyperlane toward destination (geometry
  from the build_hyperlane_bucket_mesh path), pointing at destination; snap to anchor on arrival.
  Default sessions may express no transit (12.4 InTransit renders when the sim says so).
- OWNER RIDER — the icon BASE: (a) a renderer-agnostic DESCRIPTOR layer (pure data per fleet:
  silhouette id, owner tint, anchor-or-transit placement, side, orientation, scale) computed from the
  snapshot with no render types; (b) ONE narrow renderer seam (small trait, e.g. FleetIconRenderer)
  whose current sole impl draws via EXISTING mechanisms (TypefaceIconSet glyph / billboard / small
  mesh). Future 3D/vector/sprite backends implement the seam; descriptors + placement math never change.
- Legible + appealing: readable at map zoom, owner-tinted; silhouette shape defined as DATA at one
  site (glyph id or one outline table) — changing the look is a one-site edit.
- [OVL] ops-telemetry rows: owner / anchor-or-transit / side / scale per fleet, verifiable against
  the Owner screenshot.
## FENCES
- Read-only; no movement authority; no Spec mutation; no new pipeline/asset/WGSL. Rider is
  load-bearing: hardcoded visuals woven into render code = remand; if the seam truly needs a new
  pipeline, STOP (DA-route).
## EXIT-PROOF
- Descriptor tests BITE renderer-free (right/left mirror by selected owner; transit 30% + orientation;
  arrival snap; <=75% bound). Renderer-seam test: a dummy second backend consumes identical
  descriptors (the forward-compat proof). cargo check + studio build + doctrine-scan/orient/doc-budget
  green; tests ledgered (birth_track 0.0.8.6-studio-live-ops). PROBATION LEADS the 12.5 cell +
  pointer row; orientation regenerated; [OVL] open until Owner screenshot.
