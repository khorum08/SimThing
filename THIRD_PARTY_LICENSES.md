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
| **Vendored path** | `crates/simthing-clausething/src/jomini/` |
| **License file** | `crates/simthing-clausething/src/jomini/LICENSE` |
| **Scope note (CT-0a)** | Only the Clausewitz/ClauseScript **text parsing path** is vendored and used: `TextTape` lexer/parser, DOM readers, scalar/encoding helpers, and text writer. Binary save format, envelope handling, melting, serde derive integration, and incremental `TokenReader` are **excluded**.
