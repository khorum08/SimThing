# MapGeneratorCLI — connected disc galaxy with bright core, 1500 stars, clustered, 3000px

**Classification: PROBATION visual evidence (DA sign-off pending).** Owner-requested render: a **disc**-shaped
galaxy, 1500 stars, 300×300 lattice, with the Stellaris-style logical clustering (local faction-start areas),
a **bright inaccessible galactic core**, and a **guaranteed single interconnected galaxy — no island
clusters** — blue starlanes, no wormholes, 3000px.

## Artifact
`docs/tests/mapgenerator_cli_disc_1500_connected_3000.png` (3000×3000). 1500 stars, **2722 base starlanes**,
a bright warm galactic core over the inaccessible core void.

## Regeneration command
```
cargo run -p simthing-mapgenerator --bin mapgen -- \
  --shape elliptical --stars 1500 --lattice-edge 300 --seed 770421 \
  --num-wormhole-pairs 0 --num-gateways 0 \
  --max-hyperlane-distance 7 \
  --num-hyperlanes-min 1 --num-hyperlanes-max 5000 --num-hyperlanes 5000 \
  --no-partitions \
  --cluster-count 5 --cluster-radius 400 \
  --hyperlanes base --hyperlane-color blue --draw-core \
  --png-size 3000 \
  --render-png docs/tests/mapgenerator_cli_disc_1500_connected_3000.png
```
**Connectivity is ON by default** (`HyperlaneGeometryParams::ensure_connected` defaults true — a galaxy
with orphaned systems is unusable), so `--connect-galaxy` is no longer needed. A designer can opt out with
`--allow-disconnected` (or add/remove links at runtime in-game).
The default `core_radius=120` leaves the **galactic-core void** (~40 cells, where stars can't be placed — it's
inaccessible in game); `--draw-core` paints the bright core glow over it.

## Owner-feedback correction (render-clip bug — the real cause of "orphans")
The galaxy data was **always** fully connected (COMPONENTS=1, every star laned), but the preview still
*showed* orphaned stars and disconnected clusters. Root cause was a one-line render bug: the preview lane
filter read `max_hyperlane_chebyshev.unwrap_or(DEFAULT_PREVIEW_MAX_HYPERLANE_CHEBYSHEV)`, so the intended
"draw all lanes" sentinel (`None`) silently became a **4-cell clip**. Base lanes are generated up to distance
7 (max 9), so every lane longer than 4 cells — including all the connecting bridges — was hidden, making a
connected galaxy *look* like a field of islands. Fixed: `None` now means **no clip** (draw every lane);
`Some(d)` still clips. This was **not** an N-connections/connectivity issue — `connect_components` already
guarantees a single component; the preview was just hiding the lanes that prove it.

## Owner-feedback corrections (earlier revision)
1. **The core void was correct** — it is the galactic core (bright, inaccessible). Restored (do **not** pass
   `--core-radius 0`) and added `--draw-core` to render the bright core graphic.
2. **Island clusters fixed at the root.** The previous render used `--max-hyperlane-distance 3`, but on this
   disc the stars sit ~5 cells apart, so distance-3 couldn't reach nearest neighbours: the base graph
   fragmented into **702 components** that connectivity then joined with 701 hair-thin threads (graph-
   connected, but it *read* as a sea of islands). Raising the link distance to **7** lets each star reach its
   real neighbours, so the base graph is naturally connected (24 components) and connectivity adds only ~23
   short bridges → a genuine connected web.

## Both hard requirements verified on this exact render
- **One interconnected galaxy, no islands:** measured **COMPONENTS = 1**, **largest component = 1500**, and
  **all 1500 stars have at least one lane** — literally no isolated system.
- **Galactic-core void preserved + lit:** the ~40-cell central void is kept (inaccessible) and rendered as a
  bright warm core.

## How connectivity is guaranteed (the new pass)
`connectivity::connect_components` runs as part of generation when `--connect-galaxy` is set, over the
**authored structural gridcell coordinates**:
1. Union-find over the base hyperlanes → connected components.
2. **Phase 1 (short bridges):** Kruskal over candidate pairs within expanding Chebyshev windows (8→16→32→64),
   adding the shortest cross-component links first.
3. **Phase 2 (guaranteed completion):** any components still separate are merged by their **nearest member
   pair**, so the result is *always* one component.
Bridges are canonical, deduplicated, self-link-free, deterministic, and folded into the **base** network (as
in Stellaris, connectivity is part of the base hyperlane graph). No routes / predecessors / pathfinding;
integer Chebyshev only, no sqrt.

## Clustering — local faction-start areas
`--cluster-count 5 --cluster-radius 400` runs the producer's logical clustering (`assign_clusters`): 5
nearest-anchor regions over authored coords, used for faction/initializer buckets. The visible dense
link-neighbourhoods in the web are the local star clusters; the 5 logical regions partition them for
faction starts. (Cluster *bridge* couplings are a separate classification and are not part of the base
preview; the connected base web already ties the galaxy together.)

## The starlanes are the genuine algorithm
Same `generate_hyperlane_topology` on authored coords (STEAD-0/-1): candidates within Chebyshev-7 (matched to
the disc's ~5-cell inter-star spacing so the web connects), shortest first, per-node fanout cap 4, dedup/
self-link rejected. On a **scattered disc**, nearest neighbours fan out in all directions, so the network is
the Stellaris-style web of local clusters (unlike the thin-armed spiral, which traced its arms). No
wormholes/gateways (`--num-wormhole-pairs 0 --num-gateways 0`).

## The connectivity proof is a production output (not just a test)
`GalaxyGenerationResult.connectivity: Option<ConnectivityReport>` carries the proof on the **production**
result of `generate_galaxy_with_structure`: `components_before/after`, `bridges_added`, `max_bridge_chebyshev`.
Any caller can assert "one interconnected galaxy" without re-deriving it, and the `mapgen` CLI prints it:
`connectivity: 1 component(s) after N bridge(s) … — one interconnected galaxy, no island clusters`. Proven
end-to-end by `connectivity::production_generation_result_surfaces_the_connectivity_proof`.

## New capabilities (committed with this artifact)
- `connectivity` module + `connect_components` (lib) — guarantees one connected galaxy; the
  `ConnectivityReport` is surfaced on `GalaxyGenerationResult` and printed by the CLI.
- `HyperlaneGeometryParams::ensure_connected` (serde-default false) + CLI `--connect-galaxy`.
- CLI `--no-partitions` — skip galaxy partition/bridge generation (the defaults don't scale to 1500 stars;
  clustering still runs).
- CLI `--draw-core` + `GalaxyPreviewOptions::draw_core_glow` — paint a bright galactic-core glow over the
  inaccessible core void (`CoreMask::radius_cells`/`center`, integer; render-only).
- When connectivity is on, the preview draws **all** base lanes (so the intentional connectivity bridges are
  visible, not filtered by the render distance cap).

## Tests
`crates/simthing-mapgenerator/tests/connectivity.rs` (5): merges two islands into one; no-op when already
connected; no self-links/duplicates; deterministic; and **the real 1500-star disc is one connected component
after the pass**.

## Validation
`cargo fmt --all -- --check` clean · `cargo test -p simthing-mapgenerator` all green (21 files) ·
`cargo test -p simthing-clausething --test stead_spatial_contract_guards` 11 green. Render/producer-only; no
closed lowerer / runtime / GPU / spec changes; no STEAD doctrine touched; 1000px previews unaffected.

## DA status
**DA-APPROVED 2026-06-15 (owner sign-off).** The owner — design authority for the MapGeneratorCLI/Mapping
track — reviewed and approved; PROBATION cleared → CURRENT_EVIDENCE in `current_evidence_index.md`.
