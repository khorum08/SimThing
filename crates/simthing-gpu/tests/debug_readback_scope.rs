//! SIM-GPU-READBACK-SCOPE-0 — scoped debug readback guard proofs.

use simthing_gpu::{
    debug_readback_allowed, scoped_debug_readback_allowed, set_debug_readback_allowed,
};

#[test]
fn debug_readback_allowed_reports_current_state() {
    set_debug_readback_allowed(false);
    assert!(!debug_readback_allowed());
    set_debug_readback_allowed(true);
    assert!(debug_readback_allowed());
    set_debug_readback_allowed(false);
}

#[test]
fn scoped_debug_readback_guard_restores_previous_false() {
    set_debug_readback_allowed(false);
    {
        let _guard = scoped_debug_readback_allowed(true);
        assert!(debug_readback_allowed());
    }
    assert!(!debug_readback_allowed());
}

#[test]
fn scoped_debug_readback_guard_restores_previous_true() {
    set_debug_readback_allowed(true);
    {
        let _guard = scoped_debug_readback_allowed(false);
        assert!(!debug_readback_allowed());
    }
    assert!(debug_readback_allowed());
    set_debug_readback_allowed(false);
}

#[test]
fn scoped_debug_readback_guard_restores_after_error_if_testable() {
    set_debug_readback_allowed(false);
    let result: Result<(), &'static str> = (|| {
        let _guard = scoped_debug_readback_allowed(true);
        assert!(debug_readback_allowed());
        Err("simulated readback failure")
    })();
    assert!(result.is_err());
    assert!(!debug_readback_allowed());
}
