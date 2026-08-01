use simthing_core::EmlResourceClass;

const STACK_LIMIT_TOKEN: &str = "const EML_STACK_MAX: u32 = 32u;";

/// JIT-specialize the sole resource token in a canonical EML interpreter.
/// Algebra, opcodes, control flow, and bindings remain byte-identical.
pub(crate) fn specialize_eml_stack_limit(
    canonical_source: &str,
    resource_class: EmlResourceClass,
) -> String {
    assert_eq!(
        canonical_source.matches(STACK_LIMIT_TOKEN).count(),
        1,
        "canonical EML shader must contain exactly one stack-limit token"
    );
    canonical_source.replace(
        STACK_LIMIT_TOKEN,
        &format!(
            "const EML_STACK_MAX: u32 = {}u;",
            resource_class.stack_slots()
        ),
    )
}
