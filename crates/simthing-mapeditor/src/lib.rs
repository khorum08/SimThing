//! SimThing Studio — Bevy presentation/authoring shell over MapGenerator producer output.
//!
//! ClauseThing/MapGenerator generates structural galaxy data; the editor breathes SimThing as a
//! render/UI metatable. Bevy transforms and visual z-height are render-only — never structural truth.

pub mod dialog;
pub mod generation;
pub mod session;
pub mod settings;
pub mod view_model;

#[cfg(windows)]
pub mod app;

#[cfg(windows)]
pub fn run() {
    app::run_studio();
}

#[cfg(not(windows))]
pub fn run() {
    eprintln!("SimThing Studio PR1 requires Windows.");
    std::process::exit(1);
}

pub use dialog::{StudioAction, WarningDialogModel};
pub use generation::{GenerationPreset, GenerationProfile, GenerationRunOutput};
pub use session::StudioSession;
pub use settings::{EditorSettings, WindowModeSetting};
pub use view_model::{
    StudioGalaxyRenderMeta, StudioGalaxyViewModel, StudioHyperlaneView, StudioStarView,
};
