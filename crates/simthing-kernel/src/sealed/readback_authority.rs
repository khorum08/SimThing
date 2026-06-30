//! GPU readback mint authority (KERNEL-CRATE-EXTRACT-0R).

/// Zero-sized compile-time token: sealed records may be minted from GPU POD
/// readback only when the caller holds kernel readback authority.
///
/// External crates cannot construct readback authority directly:
///
/// ```compile_fail
/// fn external_readback_authority_forge() {
///     let _ = simthing_kernel::ReadbackAuthority(());
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ReadbackAuthority(());

impl ReadbackAuthority {
    pub(crate) fn kernel_readback() -> Self {
        Self(())
    }

    /// Readback mint token for `simthing-gpu` session/buffer readback paths.
    #[doc(hidden)]
    pub fn for_kernel_readback() -> Self {
        Self::kernel_readback()
    }
}
