//! Studio typeface label seam — re-export of the LR8 presentation adapter.

#[allow(unused_imports)]
pub use crate::studio_typeface_shell::{
    mount_studio_typeface_plugins, typeface_fixture_font_bytes, StudioTypefaceShellMounted,
    StudioTypefaceShellPlugin,
};
#[allow(unused_imports)]
pub use simthing_tools::{
    icon_name_to_codepoint, resolve_studio_display_text, spawn_studio_typeface_label,
    studio_typeface_label_diagnostics, try_parse_damage_value, StudioDamageTextEmitter,
    StudioLabelKind, StudioTypefaceLabel, StudioTypefaceLabelConfig,
    StudioTypefaceLabelDiagnostics, StudioTypefaceLabelPlugin, TextLabel, TextLabelRenderMode,
    TextPerfDiagnostics, TypefaceIconSet,
};
