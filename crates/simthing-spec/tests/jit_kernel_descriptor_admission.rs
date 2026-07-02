//! Phase M-JIT-DESC-1 — Spec-layer JIT kernel descriptor admission preview.

use simthing_spec::{
    landed_jit_kernel_descriptors, validate_exact_kernel_inputs,
    validate_kernel_descriptor_admission, KernelDescriptorSpec, KernelLane, KernelOutputSpec,
    NativeMathClass, OutputAuthority, SpecError,
};

fn grad0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
}

fn grad1() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_1_observer_score")
        .expect("grad1 descriptor")
}

fn sqrt0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0 descriptor")
}

fn assert_admission_err(spec: &KernelDescriptorSpec, reason_substr: &str) {
    let err = validate_kernel_descriptor_admission(spec).expect_err("expected admission failure");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

fn assert_exact_input_err(producer: &KernelDescriptorSpec, inputs: &[&str], reason_substr: &str) {
    let err =
        validate_exact_kernel_inputs(producer, inputs).expect_err("expected exact-input failure");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
