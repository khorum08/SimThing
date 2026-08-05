//! EML-LN-PRIMITIVE-0 cost gate: driver-originated compiled resource effects
//! for the LN primitive candidate versus its own pinned gadget encoding,
//! judged through the landed `ExactPrimitiveAdmissionDoor::verify_cost` key.

use ash::{vk, Entry};
use naga::back::spv;
use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, EmlResourceClass};
use simthing_gpu::{
    apply_field_sweep_registration, field_param, FieldAdjacency, FieldLawProof, FieldSweepOutput,
    FieldSweepRegistration, FieldSweepRegistrationRequest,
};
use simthing_kernel::{
    ExactPrimitiveAdmissionDoor, ExactPrimitiveCostEvidence, ExactPrimitiveResourceEffect,
};
use std::ffi::{CStr, CString};

const TARGET_VENDOR_ID: u32 = 0x10de;
const TARGET_DEVICE_NAME: &str = "NVIDIA GeForce RTX 4080 Laptop GPU";

fn node(opcode: u32, a: u32, b: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a,
        b,
        c: 0,
        d: 0,
    }
}

fn literal(value: f32) -> EmlNodeGpu {
    node(eml_opcode::LITERAL_F32, value.to_bits(), 0)
}

fn target() -> EmlNodeGpu {
    node(eml_opcode::TARGET_VALUE, 0, 0)
}

fn trivial_program() -> Vec<EmlNodeGpu> {
    vec![
        node(eml_opcode::PARAM, field_param::ACCUMULATOR, 0),
        node(eml_opcode::RETURN_TOP, 0, 0),
    ]
}

/// Pinned gadget baseline: degree-5 Horner `ln(1+f)` Taylor approximation as
/// an ordinary authored tree — `((((C5·f + C4)·f + C3)·f + C2)·f + C1)·f`
/// with alternating-sign Taylor coefficients for ln(1+f). ~21 nodes,
/// `LegacyFixed32` — LN's own ordinary-EML encoding before the primitive.
fn gadget_baseline_post_program() -> Vec<EmlNodeGpu> {
    let coefficients = [
        -1.0 / 5.0_f32,
        1.0 / 4.0,
        -1.0 / 3.0,
        0.5,
        1.0,
    ];
    let mut nodes = vec![literal(coefficients[0])];
    for coefficient in &coefficients[1..] {
        nodes.push(target());
        nodes.push(node(eml_opcode::MUL, 0, 0));
        nodes.push(literal(*coefficient));
        nodes.push(node(eml_opcode::ADD, 0, 0));
    }
    nodes.push(node(eml_opcode::RETURN_TOP, 0, 0));
    nodes
}

fn ln_candidate_post_program() -> Vec<EmlNodeGpu> {
    vec![
        target(),
        node(
            eml_opcode::CLAMP_BOUNDED,
            simthing_core::EML_LN_DOMAIN_MIN_BITS,
            simthing_core::EML_LN_DOMAIN_MAX_BITS,
        ),
        node(eml_opcode::LN, 0, 0),
        node(eml_opcode::RETURN_TOP, 0, 0),
    ]
}

fn registration(post_program: Vec<EmlNodeGpu>) -> FieldSweepRegistration {
    let col = ColumnIndex::try_from_admitted_authored(0, 1).expect("bounded column");
    let adjacency = FieldAdjacency::independent_slots(64, col).expect("independent slots");
    let order = adjacency.apply_canonical_order_proof();
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 1,
        output: FieldSweepOutput::Matrix(col),
        map_program: trivial_program(),
        fold_program: trivial_program(),
        identity_bits: 0.0f32.to_bits(),
        post_program,
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("cost-evidence program admission")
}

fn c_char_array(value: &[std::ffi::c_char]) -> String {
    unsafe { CStr::from_ptr(value.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

fn compile_spirv(source: &str) -> Vec<u32> {
    let module = naga::front::wgsl::parse_str(source).expect("parse field-sweep WGSL");
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .expect("validate field-sweep WGSL");
    let mut options = spv::Options::default();
    options.flags = spv::WriterFlags::LABEL_VARYINGS | spv::WriterFlags::FORCE_POINT_SIZE;
    options.bounds_check_policies = naga::proc::BoundsCheckPolicies {
        index: naga::proc::BoundsCheckPolicy::Restrict,
        buffer: naga::proc::BoundsCheckPolicy::Unchecked,
        image_load: naga::proc::BoundsCheckPolicy::Unchecked,
        image_store: naga::proc::BoundsCheckPolicy::Unchecked,
        binding_array: naga::proc::BoundsCheckPolicy::Unchecked,
    };
    let pipeline = spv::PipelineOptions {
        shader_stage: naga::ShaderStage::Compute,
        entry_point: "main".into(),
    };
    spv::write_vec(&module, &info, &options, Some(&pipeline))
        .expect("compile field-sweep WGSL to SPIR-V")
}

struct MeasuredEffect {
    registers: u64,
    binary_bytes: u64,
    local_memory_bytes: u64,
}

unsafe fn measure_pipeline(
    device: &ash::Device,
    extension: &ash::khr::pipeline_executable_properties::Device,
    pipeline_layout: vk::PipelineLayout,
    source: &str,
    label: &str,
) -> MeasuredEffect {
    let spirv = compile_spirv(source);
    let shader_info = vk::ShaderModuleCreateInfo::default().code(&spirv);
    let shader = unsafe { device.create_shader_module(&shader_info, None) }
        .expect("create cost-evidence shader module");
    let entry_name = CString::new("main").expect("entry point");
    let stage = vk::PipelineShaderStageCreateInfo::default()
        .stage(vk::ShaderStageFlags::COMPUTE)
        .module(shader)
        .name(&entry_name);
    let create_info = vk::ComputePipelineCreateInfo::default()
        .flags(vk::PipelineCreateFlags::CAPTURE_STATISTICS_KHR)
        .stage(stage)
        .layout(pipeline_layout);
    let pipeline =
        unsafe { device.create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None) }
            .map_err(|(_, error)| error)
            .expect("create statistics-enabled cost-evidence pipeline")[0];
    unsafe { device.destroy_shader_module(shader, None) };

    let pipeline_info = vk::PipelineInfoKHR::default().pipeline(pipeline);
    let executables = unsafe { extension.get_pipeline_executable_properties(&pipeline_info) }
        .expect("query cost-evidence executable properties");
    let mut effect = MeasuredEffect {
        registers: 0,
        binary_bytes: 0,
        local_memory_bytes: 0,
    };
    for (executable_index, _) in executables.iter().enumerate() {
        let executable_info = vk::PipelineExecutableInfoKHR::default()
            .pipeline(pipeline)
            .executable_index(executable_index as u32);
        let statistics = unsafe { extension.get_pipeline_executable_statistics(&executable_info) }
            .expect("query cost-evidence executable statistics");
        for statistic in statistics {
            let name = c_char_array(&statistic.name).to_ascii_lowercase();
            let value = unsafe {
                match statistic.format {
                    vk::PipelineExecutableStatisticFormatKHR::INT64 => statistic.value.i64 as u64,
                    vk::PipelineExecutableStatisticFormatKHR::UINT64 => statistic.value.u64,
                    _ => continue,
                }
            };
            eprintln!("EML_LN_COST_STAT {label} name={name:?} value={value}");
            if name.contains("register") {
                effect.registers = effect.registers.max(value);
            } else if name.contains("binary") {
                effect.binary_bytes = effect.binary_bytes.max(value);
            } else if name.contains("local memory") {
                effect.local_memory_bytes = effect.local_memory_bytes.max(value);
            }
        }
    }
    unsafe { device.destroy_pipeline(pipeline, None) };
    effect
}

#[test]
#[ignore = "remand 5186492955: production EvalEML LN authority removed; STOP frozen; candidates use standalone WGSL"]
fn eml_ln_primitive_0_cost_gate_beats_the_pinned_gadget_baseline() {
    let baseline = registration(gadget_baseline_post_program());
    assert_eq!(
        baseline.resource_class(),
        EmlResourceClass::LegacyFixed32,
        "the Horner ln(1+f) gadget baseline exceeds CompactStack4 and rides Legacy32"
    );
    let candidate = registration(ln_candidate_post_program());
    assert_eq!(
        candidate.resource_class(),
        EmlResourceClass::CompactStack4,
        "the guarded LN primitive call is a compact program"
    );
    let interpreter_source = include_str!("../../simthing-kernel/src/shaders/field_sweep.wgsl");
    let candidate_source = candidate
        .generated_jit_wgsl_for_profiling(EmlResourceClass::CompactStack4)
        .expect("candidate generated JIT source");

    let entry = unsafe { Entry::load() }.expect("load the system Vulkan loader");
    let app_name = CString::new("simthing-eml-ln-cost-evidence").unwrap();
    let app_info = vk::ApplicationInfo::default()
        .application_name(&app_name)
        .api_version(vk::API_VERSION_1_3);
    let instance_info = vk::InstanceCreateInfo::default().application_info(&app_info);
    let instance = unsafe { entry.create_instance(&instance_info, None) }
        .expect("create profiling-only Vulkan instance");
    let target_device = unsafe { instance.enumerate_physical_devices() }
        .expect("enumerate Vulkan physical devices")
        .into_iter()
        .find(|physical_device| {
            let properties = unsafe { instance.get_physical_device_properties(*physical_device) };
            properties.vendor_id == TARGET_VENDOR_ID
                && c_char_array(&properties.device_name) == TARGET_DEVICE_NAME
        })
        .expect("cost gate requires the certified adapter");
    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(target_device) };
    let queue_family_index = queue_families
        .iter()
        .enumerate()
        .find(|(_, family)| family.queue_flags.contains(vk::QueueFlags::COMPUTE))
        .map(|(index, _)| index as u32)
        .expect("certified adapter exposes a compute queue");
    let queue_priorities = [1.0f32];
    let queue_info = vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities);
    let extension_names = [ash::khr::pipeline_executable_properties::NAME.as_ptr()];
    let mut executable_feature =
        vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR::default()
            .pipeline_executable_info(true);
    let device_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(std::slice::from_ref(&queue_info))
        .enabled_extension_names(&extension_names)
        .push_next(&mut executable_feature);
    let device = unsafe { instance.create_device(target_device, &device_info, None) }
        .expect("create profiling-only Vulkan device");
    let extension = ash::khr::pipeline_executable_properties::Device::new(&instance, &device);

    let bindings = (0..8u32)
        .map(|binding| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(binding)
                .descriptor_type(if binding == 6 {
                    vk::DescriptorType::UNIFORM_BUFFER
                } else {
                    vk::DescriptorType::STORAGE_BUFFER
                })
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        })
        .collect::<Vec<_>>();
    let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
    let set_layout = unsafe { device.create_descriptor_set_layout(&set_layout_info, None) }
        .expect("create descriptor-set layout");
    let set_layouts = [set_layout];
    let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts);
    let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) }
        .expect("create pipeline layout");

    let interpreter = unsafe {
        measure_pipeline(
            &device,
            &extension,
            pipeline_layout,
            interpreter_source,
            "canonical-interpreter-legacy32-gadget-baseline",
        )
    };
    let jit = unsafe {
        measure_pipeline(
            &device,
            &extension,
            pipeline_layout,
            &candidate_source,
            "jit-ln-primitive-compact4",
        )
    };
    eprintln!(
        "EML_LN_COST baseline registers={} binary={} local={} | candidate registers={} binary={} local={}",
        interpreter.registers,
        interpreter.binary_bytes,
        interpreter.local_memory_bytes,
        jit.registers,
        jit.binary_bytes,
        jit.local_memory_bytes
    );

    let evidence = ExactPrimitiveCostEvidence {
        resource_class: EmlResourceClass::CompactStack4,
        canonical_interpreter: ExactPrimitiveResourceEffect {
            register_count: interpreter.registers,
            binary_size_bytes: interpreter.binary_bytes,
            local_memory_bytes: interpreter.local_memory_bytes,
        },
        primitive_candidate: ExactPrimitiveResourceEffect {
            register_count: jit.registers,
            binary_size_bytes: jit.binary_bytes,
            local_memory_bytes: jit.local_memory_bytes,
        },
    };
    ExactPrimitiveAdmissionDoor::verify_cost(evidence)
        .expect("LN beats its own gadget encoding under the unweakened cost key");

    unsafe {
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_descriptor_set_layout(set_layout, None);
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}
