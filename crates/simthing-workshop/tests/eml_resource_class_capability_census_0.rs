//! EML-RESOURCE-CLASS-ADMISSION-0 remand: bounded supported-adapter capability census.
//!
//! This is intentionally test-only. It asks the Vulkan driver directly which standardized
//! resource-footprint/performance doors exist before production resource-class semantics move.

use ash::{vk, Entry};
use naga::back::spv;
use simthing_core::EmlResourceClass;
use simthing_gpu::{
    compile_min_plus_field_sweep, compile_structured_field_sweeps, FieldSweepRegistration,
    MinPlusStencilConfig, StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, MIN_PLUS_INF, SATURATING_FLUX_CHI_CFL_MAX,
};
use std::ffi::{CStr, CString};
use std::process::Command;

const TARGET_VENDOR_ID: u32 = 0x10de;
const TARGET_DEVICE_NAME: &str = "NVIDIA GeForce RTX 4080 Laptop GPU";
const RELEVANT_COUNTER_LIMIT: usize = 64;

fn c_char_array(value: &[std::ffi::c_char]) -> String {
    unsafe { CStr::from_ptr(value.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

fn tool_available(name: &str) -> bool {
    Command::new(name).arg("--version").output().is_ok()
}

fn relevant_counter(name: &str, category: &str, description: &str) -> bool {
    let text = format!("{name} {category} {description}").to_ascii_lowercase();
    [
        "occup", "active", "wave", "warp", "register", "spill", "stall", "memory", "shader",
        "compute", "sm ", "sm_", "cache",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn field_sweep_spirv(source: &str, entry_point: &str) -> Vec<u32> {
    let module = naga::front::wgsl::parse_str(&source).expect("parse canonical field-sweep WGSL");
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .expect("validate canonical field-sweep WGSL");
    let mut options = spv::Options::default();
    // Match wgpu-hal's Vulkan writer posture for this adapter.
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
        entry_point: entry_point.into(),
    };
    spv::write_vec(&module, &info, &options, Some(&pipeline))
        .expect("compile canonical field-sweep WGSL to SPIR-V")
}

fn pipeline_statistic_value(statistic: &vk::PipelineExecutableStatisticKHR<'_>) -> String {
    unsafe {
        match statistic.format {
            vk::PipelineExecutableStatisticFormatKHR::BOOL32 => {
                format!("{}", statistic.value.b32 != vk::FALSE)
            }
            vk::PipelineExecutableStatisticFormatKHR::INT64 => statistic.value.i64.to_string(),
            vk::PipelineExecutableStatisticFormatKHR::UINT64 => statistic.value.u64.to_string(),
            vk::PipelineExecutableStatisticFormatKHR::FLOAT64 => {
                format!("{:.6}", statistic.value.f64)
            }
            other => format!("UNKNOWN_FORMAT({})", other.as_raw()),
        }
    }
}

unsafe fn create_profiled_pipeline(
    device: &ash::Device,
    pipeline_layout: vk::PipelineLayout,
    source: &str,
    entry_point: &str,
) -> vk::Pipeline {
    let spirv = field_sweep_spirv(source, entry_point);
    let shader_info = vk::ShaderModuleCreateInfo::default().code(&spirv);
    let shader = unsafe { device.create_shader_module(&shader_info, None) }
        .expect("create canonical field-sweep shader module");
    let entry_name = CString::new(entry_point).expect("shader entry point has no NUL");
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
            .expect("create statistics-enabled canonical field-sweep pipeline")[0];
    unsafe { device.destroy_shader_module(shader, None) };
    pipeline
}

unsafe fn print_pipeline_statistics(
    extension: &ash::khr::pipeline_executable_properties::Device,
    pipeline: vk::Pipeline,
    label: &str,
) -> Vec<(String, String)> {
    let pipeline_info = vk::PipelineInfoKHR::default().pipeline(pipeline);
    let executables = unsafe { extension.get_pipeline_executable_properties(&pipeline_info) }
        .expect("query canonical field-sweep executable properties");
    let mut rows = Vec::new();
    for (executable_index, executable) in executables.iter().enumerate() {
        println!(
            "EML_RC_JIT_PIPELINE_EXECUTABLE {label} executable_index={executable_index} name={:?} description={:?} stages={:?} subgroup_size={}",
            c_char_array(&executable.name),
            c_char_array(&executable.description),
            executable.stages,
            executable.subgroup_size,
        );
        let executable_info = vk::PipelineExecutableInfoKHR::default()
            .pipeline(pipeline)
            .executable_index(executable_index as u32);
        let statistics = unsafe { extension.get_pipeline_executable_statistics(&executable_info) }
            .expect("query canonical field-sweep executable statistics");
        for statistic in statistics {
            let name = c_char_array(&statistic.name);
            let value = pipeline_statistic_value(&statistic);
            println!(
                "EML_RC_JIT_PIPELINE_STAT {label} executable_index={executable_index} name={name:?} description={:?} format={:?} value={value}",
                c_char_array(&statistic.description),
                statistic.format,
            );
            rows.push((name, value));
        }
    }
    rows
}

#[test]
fn eml_resource_class_canonical_pipeline_resource_statistics() {
    let palma = compile_min_plus_field_sweep(&MinPlusStencilConfig {
        width: 16,
        height: 16,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x: 2,
        dest_y: 2,
        inf_sentinel: MIN_PLUS_INF,
    })
    .expect("PALMA generated-JIT admission");
    let (north, south, east, west) = StructuredFieldStencilConfig::zero_directional_weights();
    let gu_yang = compile_structured_field_sweeps(&StructuredFieldStencilConfig {
        width: 16,
        height: 16,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: north,
        weight_south: south,
        weight_east: east,
        weight_west: west,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 1.0,
            chi: SATURATING_FLUX_CHI_CFL_MAX,
            choke_output_col: None,
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Clamp,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    })
    .expect("Gu-Yang generated-JIT admission");
    let palma_source = palma
        .generated_jit_wgsl_for_profiling(EmlResourceClass::CompactStack4)
        .expect("PALMA generated source");
    let gu_yang_source = FieldSweepRegistration::generated_fused_jit_wgsl_for_profiling(
        &gu_yang[0],
        &gu_yang[1],
        EmlResourceClass::CompactStack4,
    )
    .expect("Gu-Yang fused generated source");
    let palma_program = palma.program_identity();
    let palma_cache = palma.jit_cache_identity();
    let (gu_yang_program, gu_yang_cache) =
        FieldSweepRegistration::fused_jit_identity_for_profiling(
            &gu_yang[0],
            &gu_yang[1],
            EmlResourceClass::CompactStack4,
        )
        .expect("Gu-Yang fused identity");
    let palma_label = format!(
        "case=PALMA class=stack4 program={:016x} cache={:016x}",
        palma_program.digest(),
        palma_cache.digest()
    );
    let gu_yang_label = format!(
        "case=Gu-Yang class=stack4 fused_program={:016x} fused_cache={:016x}",
        gu_yang_program.digest(),
        gu_yang_cache.digest()
    );
    let entry = unsafe { Entry::load() }.expect("load the system Vulkan loader");
    let app_name = CString::new("simthing-eml-resource-class-pipeline-statistics").unwrap();
    let app_info = vk::ApplicationInfo::default()
        .application_name(&app_name)
        .api_version(vk::API_VERSION_1_3);
    let instance_info = vk::InstanceCreateInfo::default().application_info(&app_info);
    let instance = unsafe { entry.create_instance(&instance_info, None) }
        .expect("create profiling-only Vulkan instance");
    let target = unsafe { instance.enumerate_physical_devices() }
        .expect("enumerate Vulkan physical devices")
        .into_iter()
        .find(|physical_device| {
            let properties = unsafe { instance.get_physical_device_properties(*physical_device) };
            properties.vendor_id == TARGET_VENDOR_ID
                && c_char_array(&properties.device_name) == TARGET_DEVICE_NAME
        });
    let Some(physical_device) = target else {
        println!(
            "EML_RC_PIPELINE_STAT status=SKIP target_adapter={TARGET_DEVICE_NAME:?} reason=target_not_present"
        );
        unsafe { instance.destroy_instance(None) };
        return;
    };

    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let queue_family_index = queue_families
        .iter()
        .enumerate()
        .find(|(_, family)| family.queue_flags.contains(vk::QueueFlags::COMPUTE))
        .map(|(index, _)| index as u32)
        .expect("target exposes a compute queue");
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
    let device = unsafe { instance.create_device(physical_device, &device_info, None) }
        .expect("create profiling-only Vulkan device with pipeline executable statistics");

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
        .expect("create canonical field-sweep descriptor-set layout");
    let set_layouts = [set_layout];
    let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts);
    let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) }
        .expect("create canonical field-sweep pipeline layout");

    let bespoke_bindings = (0..4u32)
        .map(|binding| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(binding)
                .descriptor_type(if binding == 0 {
                    vk::DescriptorType::UNIFORM_BUFFER
                } else {
                    vk::DescriptorType::STORAGE_BUFFER
                })
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        })
        .collect::<Vec<_>>();
    let bespoke_set_layout_info =
        vk::DescriptorSetLayoutCreateInfo::default().bindings(&bespoke_bindings);
    let bespoke_set_layout =
        unsafe { device.create_descriptor_set_layout(&bespoke_set_layout_info, None) }
            .expect("create unmodified bespoke descriptor-set layout");
    let bespoke_set_layouts = [bespoke_set_layout];
    let bespoke_pipeline_layout_info =
        vk::PipelineLayoutCreateInfo::default().set_layouts(&bespoke_set_layouts);
    let bespoke_pipeline_layout =
        unsafe { device.create_pipeline_layout(&bespoke_pipeline_layout_info, None) }
            .expect("create unmodified bespoke pipeline layout");

    let palma_pipeline =
        unsafe { create_profiled_pipeline(&device, pipeline_layout, &palma_source, "main") };
    let gu_yang_pipeline =
        unsafe { create_profiled_pipeline(&device, pipeline_layout, &gu_yang_source, "main") };
    let gu_yang_bespoke_pipeline = unsafe {
        create_profiled_pipeline(
            &device,
            bespoke_pipeline_layout,
            include_str!("../../simthing-gpu/src/shaders/structured_field_stencil.wgsl"),
            "stencil_step",
        )
    };
    let extension = ash::khr::pipeline_executable_properties::Device::new(&instance, &device);
    let palma_stats =
        unsafe { print_pipeline_statistics(&extension, palma_pipeline, &palma_label) };
    let gu_yang_stats =
        unsafe { print_pipeline_statistics(&extension, gu_yang_pipeline, &gu_yang_label) };
    let gu_yang_bespoke_stats = unsafe {
        print_pipeline_statistics(
            &extension,
            gu_yang_bespoke_pipeline,
            "case=Gu-Yang-bespoke reference=unmodified",
        )
    };

    assert!(
        !palma_stats.is_empty() && !gu_yang_stats.is_empty() && !gu_yang_bespoke_stats.is_empty(),
        "supported door must return compiled resource statistics for generated and reference pipelines"
    );
    println!(
        "EML_RC_JIT_PIPELINE_STAT comparison=PALMA_vs_Gu-Yang_vs_bespoke palma_statistics={} gu_yang_statistics={} bespoke_statistics={} palma_gu_yang_identical={} gu_yang_bespoke_identical={}",
        palma_stats.len(),
        gu_yang_stats.len(),
        gu_yang_bespoke_stats.len(),
        palma_stats == gu_yang_stats,
        gu_yang_stats == gu_yang_bespoke_stats,
    );

    unsafe {
        device.destroy_pipeline(palma_pipeline, None);
        device.destroy_pipeline(gu_yang_pipeline, None);
        device.destroy_pipeline(gu_yang_bespoke_pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_pipeline_layout(bespoke_pipeline_layout, None);
        device.destroy_descriptor_set_layout(set_layout, None);
        device.destroy_descriptor_set_layout(bespoke_set_layout, None);
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}
