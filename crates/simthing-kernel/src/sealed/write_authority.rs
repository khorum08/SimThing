//! Resolved-state write authority token (KERNEL-WRITE-SEAL-0).

/// Zero-sized compile-time token: live resolved GPU column writes require
/// accumulator authority or an explicit boundary/admission install path.
///
/// External crates cannot construct write authority directly:
///
/// ```compile_fail
/// fn external_resolved_write_authority_forge() {
///     let _ = simthing_kernel::ResolvedWriteAuthority(());
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ResolvedWriteAuthority(());

impl ResolvedWriteAuthority {
    pub(crate) fn boundary_install() -> Self {
        Self(())
    }
}
