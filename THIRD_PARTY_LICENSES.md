# Third-party licenses

This file records vendored third-party code included in the SimThing workspace.

## jomini (text parsing path only)

| Field | Value |
|---|---|
| **Package** | jomini |
| **Upstream origin** | https://github.com/rakaly/jomini |
| **Vendored version** | v0.34.1 |
| **Vendored commit** | `fff00d8c7f8f06c084d776d1a2c98b34324e64ed` |
| **License** | MIT |
| **Copyright holder** | Nick Babcock (from upstream `Cargo.toml` `authors` at vendored commit; upstream `LICENSE.txt` omits the copyright line) |
| **Vendored path** | `crates/simthing-clausething/src/jomini/` |
| **License file** | `crates/simthing-clausething/src/jomini/LICENSE` |
| **Scope note (CT-0a)** | Only the Clausewitz/ClauseScript **text parsing path** is vendored and used: `TextTape` lexer/parser, DOM readers, scalar/encoding helpers, and text writer. Binary save format, envelope handling, melting, serde derive integration, and incremental `TokenReader` are **excluded**.

## Noto Sans Regular (TYPEFACE-LR0 test fixture only)

| Field | Value |
|---|---|
| **Font** | Noto Sans Regular |
| **Upstream origin** | https://github.com/googlefonts/noto-fonts |
| **Fixture path** | `crates/simthing-workshop/assets/typeface/test_font.ttf` |
| **License** | SIL Open Font License 1.1 (OFL) |
| **Copyright holder** | Google LLC and contributors (see upstream OFL header) |
| **Scope note (TYPEFACE-LR0)** | Hermetic workshop test fixture for font load + glyph metrics only. **Not** the bundled default game font decision. Full upstream file committed for deterministic cmap/metrics tests.
