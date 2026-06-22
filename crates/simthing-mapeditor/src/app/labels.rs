//! Studio typeface label seam — re-export of the LR8 presentation adapter.

pub use simthing_tools::{
    icon_name_to_codepoint, resolve_studio_display_text, spawn_studio_typeface_label,
    studio_typeface_label_diagnostics, try_parse_damage_value, StudioDamageTextEmitter,
    StudioLabelKind, StudioTypefaceLabel, StudioTypefaceLabelConfig,
    StudioTypefaceLabelDiagnostics, StudioTypefaceLabelPlugin, TextLabel, TextLabelRenderMode,
    TextPerfDiagnostics, TypefaceIconSet,
};
