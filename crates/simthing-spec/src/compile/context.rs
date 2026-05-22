use simthing_core::DimensionRegistry;

/// Threads a mutable `DimensionRegistry` through batch spec compilation.
///
/// `CompileContext` is the recommended entry point when compiling multiple
/// specs from the same `DomainPackSpec` / `GameModeSpec` — it owns the
/// registry borrow and exposes the same `compile_*` operations as the
/// top-level free functions.
pub struct CompileContext<'a> {
    pub registry: &'a mut DimensionRegistry,
}

impl<'a> CompileContext<'a> {
    pub fn new(registry: &'a mut DimensionRegistry) -> Self {
        Self { registry }
    }

    /// Borrow the registry immutably (e.g. for overlay compilation, which does
    /// not mutate the registry).
    pub fn registry(&self) -> &DimensionRegistry {
        self.registry
    }

    /// Borrow the registry mutably (e.g. for property compilation, which
    /// registers a new `SimProperty`).
    pub fn registry_mut(&mut self) -> &mut DimensionRegistry {
        self.registry
    }
}
