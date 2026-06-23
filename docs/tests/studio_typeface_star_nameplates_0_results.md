# STUDIO-TYPEFACE-STAR-NAMEPLATES-0 Results

## Status

DONE / PROBATION - implemented and exercised on a real Windows Vulkan adapter; owner visual sign-off remains pending.

## PR / branch / merge

- Branch: `codex/studio-typeface-star-nameplates`
- PR: PENDING
- Merge: PENDING

## Mission

Exercise the closed `simthing-tools` typeface toolkit in the live `simthing-studio` client by rendering
formatted SimThing IDs below generated stars. The earlier `Camera2d` experiment triggered Bevy D2/D3
tonemapping bind-group failures, so this rung uses the existing Studio `Camera3d` without global LUT mutation.

## Implemented changes

- Added a feature-gated Camera3d `Transparent3d` world-billboard renderer to `simthing-tools`.
- Kept one aggregate draw entity and one persistent GPU instance buffer for the entire nameplate field.
- Rendered `SIM-{raw_id:06}` from authoritative ScenarioSpec structural placements at each star render anchor.
- Added persisted Settings controls for relative width, base transparency, falloff distance, and falloff transparency.
- Derived nameplate scale and distance attenuation from the existing star render settings.
- Added a `world_text_only` plugin mount so Studio does not register the toolkit's screen-space Core2d path.
- Preserved existing `Assets<Shader>`, `Assets<Mesh>`, and `Assets<Image>` resources when mounting the plugin.
  Bevy 0.16 `init_asset` replaces an existing store; the old mount erased loaded LUT/image assets.
- Selected the toolkit raster atlas mode for this first live dress rehearsal. Shaping, atlas residency, style
  tables, and the instanced renderer remain the same toolkit path; MSDF live-window tuning is deferred.

## Boundary checks

- Presentation-only; ScenarioSpec remains authority.
- No simulation runtime, RF, STEAD, save/load, or GPU-dispatch ownership changes.
- No extra live camera and no tonemapping LUT/Image/GpuImage/FallbackImage mutation.
- Shaping and atlas work are change-time only; steady-state draw remains GPU-instanced.
- The world-text WGSL remains semantic-free.

## Validation

- `cargo fmt -p simthing-tools -p simthing-mapeditor -- --check`
- `cargo check -p simthing-tools --features world-text-3d`
- `cargo test -p simthing-tools --features world-text-3d world_text --lib`
- `cargo test -p simthing-tools --features world-text-3d plugin_mount_preserves_existing_image_assets --lib`
- `cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard`
- `cargo test -p simthing-mapeditor nameplate --lib`
- `cargo test -p simthing-tools --features world-text-3d`
- `cargo test -p simthing-mapeditor`
- `cargo build --release -p simthing-mapeditor --bin simthing-studio`
- `git diff --check`

Outcomes:

- PASS - package-scoped format check, `git diff --check`, tools feature check, focused world-text,
  asset-store, semantic-free, and nameplate tests.
- PASS - full `simthing-tools` suite with `world-text-3d`, including real-adapter shader pixel proof.
- PASS - all 413 `simthing-mapeditor` library tests.
- PASS - release build; `target/release/simthing-studio.exe` remained alive for a 20-second native-window
  smoke with no panic, shader, validation, or LUT error.
- BASELINE LIMITATION - an existing LR8 meta-test recursively launches the same package test and deadlocks
  on Cargo's artifact lock (`lr8_headless_tests_still_pass`). Its underlying LR8 tests run directly.
- BASELINE LIMITATION - ten existing production-document assertions expect historical tokens absent from
  current `docs/design_0_0_8_3_studio_production.md`; this change only appends to that file.
- BASELINE LIMITATION - two existing `studio_planet_child_location_display` fixture assertions expect
  `terra_prime` with three children; current master returns `admitted_p` with zero. No touched file owns that path.

## Real-adapter observation

- Host adapter: NVIDIA GeForce RTX 4080 Laptop GPU through Vulkan.
- Generated galaxy: 2,400 systems.
- World-text aggregation: 24,000 glyph instances in one draw field.
- The live window survived repeated generation and camera movement without LUT, shader, pipeline, or ECS panic.
- Raster nameplates remained anchored below stars and attenuated with camera distance.

## Known gaps

- Whole-galaxy overview compresses IDs to small marks; closer framing is needed to read individual IDs.
- Production entity names remain outside this formatted-ID dress rehearsal.
- MSDF live-window appearance tuning is deferred; this rung deliberately uses the stable raster atlas path.
- Owner visual sign-off is pending, so this evidence remains PROBATION.

## DA status

Owner approved implementation on 2026-06-22. This report is not DA-promoted.
