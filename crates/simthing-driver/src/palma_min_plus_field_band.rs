//! Legacy PATH-6 names — prefer [`crate::min_plus_traversal_field`] for production callers.

pub use crate::min_plus_traversal_field::{
    TraversalFieldBandError as PalmaMinPlusFieldBandError,
    TraversalFieldBandSession as PalmaMinPlusFieldBandSession,
    TraversalFieldBandTickReport as PalmaMinPlusFieldBandTickReport,
    TraversalFieldDispatchReport as PalmaMinPlusFieldDispatchReport, TraversalFieldExecutionMode,
    TraversalFieldGridBinding as PalmaMinPlusGridBinding,
    TraversalFieldShadowColumnCompatInput as PalmaMinPlusShadowColumnCompatInput,
    TRAVERSAL_FIELD_BAND_DEFAULT_ENABLED as PALMA_MIN_PLUS_FIELD_BAND_DEFAULT_ENABLED,
    TRAVERSAL_FIELD_ID as PALMA_MIN_PLUS_FIELD_ID,
    TRAVERSAL_FIELD_REGION_ID as PALMA_MIN_PLUS_REGION_ID,
    TRAVERSAL_FIELD_UTILITY_ID as PALMA_MIN_PLUS_FIELD_BAND_PROFILE_ID,
};
