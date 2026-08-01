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
fn eml_resource_class_supported_adapter_capability_census() {
    let entry = unsafe { Entry::load() }.expect("load the system Vulkan loader");
    let app_name = CString::new("simthing-eml-resource-class-capability-census").unwrap();
    let app_info = vk::ApplicationInfo::default()
        .application_name(&app_name)
        .api_version(vk::API_VERSION_1_3);
    let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);
    let instance = unsafe { entry.create_instance(&create_info, None) }
        .expect("create census-only Vulkan instance");

    let physical_devices = unsafe { instance.enumerate_physical_devices() }
        .expect("enumerate Vulkan physical devices");
    let target = physical_devices.into_iter().find(|physical_device| {
        let properties = unsafe { instance.get_physical_device_properties(*physical_device) };
        properties.vendor_id == TARGET_VENDOR_ID
            && c_char_array(&properties.device_name) == TARGET_DEVICE_NAME
    });

    let Some(physical_device) = target else {
        println!(
            "EML_RC_CAPABILITY_CENSUS status=SKIP target_adapter={TARGET_DEVICE_NAME:?} reason=target_not_present"
        );
        unsafe { instance.destroy_instance(None) };
        return;
    };

    let extension_properties =
        unsafe { instance.enumerate_device_extension_properties(physical_device) }
            .expect("enumerate target device extensions");
    let extensions = extension_properties
        .iter()
        .map(|property| c_char_array(&property.extension_name))
        .collect::<Vec<_>>();
    let has_extension = |name: &CStr| {
        let name = name.to_string_lossy();
        extensions
            .iter()
            .any(|extension| extension == name.as_ref())
    };

    let mut pipeline_features =
        vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR::default();
    let mut performance_features = vk::PhysicalDevicePerformanceQueryFeaturesKHR::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut pipeline_features)
        .push_next(&mut performance_features);
    unsafe { instance.get_physical_device_features2(physical_device, &mut features2) };

    let mut driver_properties = vk::PhysicalDeviceDriverProperties::default();
    let mut performance_properties = vk::PhysicalDevicePerformanceQueryPropertiesKHR::default();
    let mut properties2 = vk::PhysicalDeviceProperties2::default()
        .push_next(&mut driver_properties)
        .push_next(&mut performance_properties);
    unsafe { instance.get_physical_device_properties2(physical_device, &mut properties2) };
    let properties = properties2.properties;

    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let compute_queue = queue_families
        .iter()
        .enumerate()
        .find(|(_, family)| family.queue_flags.contains(vk::QueueFlags::COMPUTE))
        .map(|(index, _)| index as u32)
        .expect("target exposes a compute queue");

    let pipeline_extension = has_extension(ash::khr::pipeline_executable_properties::NAME);
    let performance_extension = has_extension(ash::khr::performance_query::NAME);
    let diagnostics_config_extension = has_extension(ash::nv::device_diagnostics_config::NAME);
    let diagnostic_checkpoints_extension =
        has_extension(ash::nv::device_diagnostic_checkpoints::NAME);
    let shader_sm_builtins_extension = has_extension(ash::nv::shader_sm_builtins::NAME);

    println!(
        "EML_RC_CAPABILITY_CENSUS adapter={:?} vendor_id=0x{:04x} device_id=0x{:04x} api={}.{}.{} driver_name={:?} driver_info={:?} driver_version={}",
        c_char_array(&properties.device_name),
        properties.vendor_id,
        properties.device_id,
        vk::api_version_major(properties.api_version),
        vk::api_version_minor(properties.api_version),
        vk::api_version_patch(properties.api_version),
        c_char_array(&driver_properties.driver_name),
        c_char_array(&driver_properties.driver_info),
        properties.driver_version,
    );
    println!(
        "EML_RC_CAPABILITY_CENSUS interface=VK_KHR_pipeline_executable_properties advertised={} pipelineExecutableInfo_feature={}",
        pipeline_extension,
        pipeline_features.pipeline_executable_info == vk::TRUE,
    );
    println!(
        "EML_RC_CAPABILITY_CENSUS interface=VK_KHR_performance_query advertised={} performanceCounterQueryPools_feature={} performanceCounterMultipleQueryPools_feature={} allowCommandBufferQueryCopies={}",
        performance_extension,
        performance_features.performance_counter_query_pools == vk::TRUE,
        performance_features.performance_counter_multiple_query_pools == vk::TRUE,
        performance_properties.allow_command_buffer_query_copies == vk::TRUE,
    );
    println!(
        "EML_RC_CAPABILITY_CENSUS interface=VK_NV_device_diagnostics_config advertised={diagnostics_config_extension} purpose=diagnostic_shader_debug_info_not_occupancy_counter"
    );
    println!(
        "EML_RC_CAPABILITY_CENSUS interface=VK_NV_device_diagnostic_checkpoints advertised={diagnostic_checkpoints_extension} purpose=execution_checkpoint_not_occupancy_counter"
    );
    println!(
        "EML_RC_CAPABILITY_CENSUS interface=VK_NV_shader_sm_builtins advertised={shader_sm_builtins_extension} purpose=shader_builtin_sm_warp_identification_not_resource_counter"
    );

    for tool in [
        "vulkaninfo",
        "nvidia-smi",
        "ncu",
        "nv-nsight-cu-cli",
        "renderdoccmd",
        "spirv-dis",
        "spirv-val",
    ] {
        println!(
            "EML_RC_CAPABILITY_CENSUS external_tool={tool} available={}",
            tool_available(tool)
        );
    }

    let mut total_counter_count = 0usize;
    let mut relevant_counter_count = 0usize;
    if performance_extension {
        let performance = ash::khr::performance_query::Instance::new(&entry, &instance);
        let counter_count = unsafe {
            performance.enumerate_physical_device_queue_family_performance_query_counters_len(
                physical_device,
                compute_queue,
            )
        }
        .expect("enumerate target compute performance counter count");
        total_counter_count = counter_count;
        let mut counters = vec![vk::PerformanceCounterKHR::default(); counter_count];
        let mut descriptions = vec![vk::PerformanceCounterDescriptionKHR::default(); counter_count];
        unsafe {
            performance.enumerate_physical_device_queue_family_performance_query_counters(
                physical_device,
                compute_queue,
                &mut counters,
                &mut descriptions,
            )
        }
        .expect("enumerate target compute performance counters");

        for (index, (counter, description)) in counters.iter().zip(descriptions.iter()).enumerate()
        {
            let name = c_char_array(&description.name);
            let category = c_char_array(&description.category);
            let detail = c_char_array(&description.description);
            if relevant_counter(&name, &category, &detail) {
                relevant_counter_count += 1;
                if relevant_counter_count <= RELEVANT_COUNTER_LIMIT {
                    println!(
                        "EML_RC_PERFORMANCE_COUNTER index={index} name={name:?} category={category:?} description={detail:?} unit={:?} scope={:?} storage={:?} flags={:?}",
                        counter.unit,
                        counter.scope,
                        counter.storage,
                        description.flags,
                    );
                }
            }
        }
    }
    println!(
        "EML_RC_CAPABILITY_CENSUS compute_queue_family={compute_queue} total_performance_counters={total_counter_count} relevant_counter_matches={relevant_counter_count} relevant_counter_rows_capped_at={RELEVANT_COUNTER_LIMIT}"
    );

    assert!(
        pipeline_extension && pipeline_features.pipeline_executable_info == vk::TRUE,
        "the named NVIDIA/Vulkan adapter must expose the candidate compiled-resource door"
    );

    unsafe { instance.destroy_instance(None) };
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
