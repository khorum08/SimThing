//! Phase T-2 — resource economy compile rejection tests.

#[path = "support/resource_economy_compile.rs"]
mod support;

use simthing_core::SubFieldRole;
use simthing_spec::{
    compile_resource_economy, EmissionFormulaSpec, PropertyKey, RecipeInputSpec,
    ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec, ResourceTransferSpec, SpecError,
};
use support::{amount_property, exact_eml_registry, fast_eml_registry, register_amount_property};

fn pk(ns: &str, name: &str) -> PropertyKey {
    PropertyKey::new(ns, name)
}
