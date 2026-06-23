# STUDIO-FALLOFF-RULER-DEBUG-0 Results

## Status

PROBATION / diagnostic overlay — visual high-horizon falloff ruler and guide lines for owner calibration.

## PR / branch / merge

- Branch: `codex/studio-falloff-ruler-debug-0`
- PR: #926
- Merge: `e8b0009ee480c4a34092f29fc203a43e47741fe9`

## Purpose

High-horizon falloff is directionally correct but hard to judge by eye. This overlay draws the bottom-center → high-horizon ruler, tick marks, and emphasized star/effective-nameplate cut lines so Settings sliders can be verified against telemetry.

## Overlay behavior

- Toggle: **Show falloff ruler** (default off) in Render debug and Performance Telemetry → Performance isolation.
- Draws base (bottom-center) and vanishing (25% from top) points, main ruler line, ticks at 0/10/25/50/65/75/100%, cyan **Star falloff** line, magenta **Effective nameplate falloff** line, and a legend with current slider values.
- Uses egui painter only (diagnostic UI); GPU TypeFace nameplates unchanged.

## Telemetry proof

Nameplate debug now reports falloff metric, ruler base/vanishing px, star/relative/effective falloff %, effective falloff screen y %, sample visual progress %, and sample final alpha.

## Visual smoke

Agent cannot capture Studio locally. Owner should enable **Show falloff ruler** and verify:
- 100% star / 100% relative → effective line at horizon (100%).
- 100% star / 65% relative → effective line at 65% ruler (~51.25% from top).
- 50% star / 50% relative → effective line at 25% ruler.
- 10% star / 100% relative → effective line near foreground.
- Moving sliders moves guide lines only; label rendering unchanged.

## Validation

```
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor nameplate --lib
git diff --check
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted Studio debug-overlay calibration.

## DA recommendation

PROBATION until owner confirms the 65% effective line aligns with label fade at ~51.25% screen height and overlay matches Nameplate debug telemetry.
