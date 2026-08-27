//! The SimThing Vendor Door.
//!
//! This crate is a stateless leaf facade over graduated engine mechanisms. Its
//! public entry surface is exactly five verb namespaces: [`derive`],
//! [`populate`], [`overlay`], [`bind`], and [`run`]. It owns no simulation
//! state, registry, scheduler, cache, history, evaluator, or runtime authority.
//!
//! A sixth allocation or granting operation is deliberately absent:
//!
//! ```compile_fail,E0432
//! use simthing_embedder::grant;
//! ```

#![forbid(unsafe_code)]

pub mod bind;
pub mod derive;
pub mod overlay;
pub mod populate;
pub mod run;
