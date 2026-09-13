//! Synthetic adversarial instrument tests. These are never Windows latency samples.
use serde_json::json;
use simthing_workshop::rehearsal_m16_displayed::*;

const ORIGIN: u64 = 10_000_000_000_000_000;
fn present(frame: u64) -> u64 {
    ORIGIN + frame * 100_000 + 50_000
}
fn fixture() -> (AppCapture, String) {
    let epoch = Epoch {
        capture: 1,
        scene: 7,
        resident: 9,
        client: "Egui".into(),
    };
    let inputs: Vec<_> = [0, 4, 0, 4, 0, 4]
        .into_iter()
        .enumerate()
        .map(|(trial, k)| {
            let frame = 65 + trial as u64 * 30;
            Input {
                response_id: trial as u64 + 1,
                trial,
                delay_k: k,
                input_qpc: present(frame) - 20_000,
                input_frame: frame,
                target_frame: frame + u64::from(k),
                epoch: epoch.clone(),
            }
        })
        .collect();
    let mut csv = "Application,ProcessID,SwapChainAddress,Runtime,Dropped,QPCTime,msUntilDisplayed,PresentMode\r\n".to_string();
    let frames = (0..240).map(|n| {
        let response = inputs.iter().rev().find(|i| i.target_frame <= n).map(|i| i.response_id);
        csv.push_str(&format!("\"simthing-studio.exe\",123,0xABC,DXGI,0,{},10.00000000000000,Hardware Composed: Independent Flip\r\n", present(n)));
        Span { ordinal: n as usize, main_frame: n, epoch: epoch.clone(), response_id: response,
            marker_in_render_data: response.is_some(), marker_pipeline_ready: response.is_some(), begin_qpc: present(n) - 10, end_qpc: present(n) + 10 }
    }).collect();
    let facts = json!({"scene":7,"resident_epoch":9,"source_identity":"synthetic-source", "profile_identity":"synthetic-profile",
        "dependencies":[],"native_enabled":false,"paused":true,"focused":true,"reset_pending":false,"window_pixels":[1920,1080],
        "scale_factor":1.0,"panels_hidden":false,"adapter":{"gpu_name":"synthetic model; no runtime claim","backend":"model"}});
    let mut metadata = serde_json::Map::new();
    for key in [
        "condition",
        "repetition",
        "code_revision",
        "instrument_revision",
        "seed",
        "hardware",
        "backend",
        "driver",
        "compiler",
        "build_flags",
        "cache_state",
        "warmup",
        "exact_command",
        "operation_sequence",
        "workload_cardinalities",
        "presentation_settings",
        "subscriptions",
    ] {
        metadata.insert(
            key.into(),
            json!("synthetic instrument witness; not a measurement"),
        );
    }
    metadata.insert("presentmon_version".into(), json!("2.5.1"));
    metadata.insert("presentmon_sha256".into(), json!(PRESENTMON_SHA256));
    (
        AppCapture {
            instrument: "m16-displayed-probe-v1".into(),
            process_id: 123,
            qpc_frequency: 10_000_000,
            config: json!({"expected_source_identity":"synthetic-source","metadata":metadata}),
            epoch,
            started_qpc: ORIGIN,
            stopped_qpc: Some(present(239) + 101_000),
            stop_reason: Some("operator stop".into()),
            initial_facts: facts.clone(),
            final_facts: facts,
            inputs,
            frames,
            rejected: vec![],
        },
        csv,
    )
}
fn mutate_csv(csv: &str, frame: usize, column: usize, value: &str) -> String {
    let mut rows: Vec<_> = csv.lines().map(str::to_string).collect();
    let mut fields: Vec<_> = rows[frame + 1].split(',').map(str::to_string).collect();
    fields[column] = value.into();
    rows[frame + 1] = fields.join(",");
    rows.join("\n") + "\n"
}

#[test]
fn rehearsal_perf_m16_qpc_decimal_and_first_displayed_components_are_exact() {
    assert_eq!(
        displayed_delta_qpc("0.00010000000000", 10_000_000).unwrap(),
        1
    );
    for bad in ["NaN", "NA", "-1", "1e3", "inf", "", "+2"] {
        assert!(displayed_delta_qpc(bad, 10_000_000).is_err());
    }
    let (capture, csv) = fixture();
    let result = analyze(&capture, &csv).unwrap();
    assert!(result.instrument_valid, "{:?}", result.failures);
    assert_eq!(result.responses.len(), 6);
    assert_eq!(result.delay_witnesses.len(), 3);
    assert_eq!(result.responses[0].present_qpc, present(65));
    assert_eq!(result.responses[0].displayed_qpc, present(65) + 100_000);
    for response in &result.responses {
        assert_eq!(
            response.total_ms,
            response.app_handling_ms + response.submit_to_displayed_ms
        );
    }
    for pair in &result.delay_witnesses {
        assert_eq!(pair.observed_k_frame_ms, 40.0);
        assert_eq!(pair.total_shift_ms, 40.0);
    }
    // A known dropped first present is retained; the endpoint is the next
    // matching present actually DISPLAYED, never that dropped submission.
    let dropped = analyze(&capture, &mutate_csv(&csv, 65, 4, "1")).unwrap();
    assert!(dropped.instrument_valid, "{:?}", dropped.failures);
    assert_eq!(dropped.responses[0].response_frame, 66);
    assert_eq!(
        dropped.frames[65].rejection.as_deref(),
        Some("dropped/not-displayed present")
    );
    assert!(parse_presentmon(&csv, 124, 10_000_000).is_err());
    assert!(parse_presentmon(&csv.replace("QPCTime", "CPUStartQPC"), 123, 10_000_000).is_err());
}

#[test]
fn rehearsal_perf_m16_known_delay_falsifier_rejects_zero_effect_and_retains_negative_residual() {
    let (mut capture, csv) = fixture();
    // A broken input timestamp channel erases the injected four-frame delay.
    // The validity witness must fail even though the rows still join uniquely.
    for input in capture.inputs.iter_mut().filter(|i| i.delay_k == 4) {
        input.input_qpc += 400_000;
    }
    let broken = analyze(&capture, &csv).unwrap();
    assert!(!broken.instrument_valid);
    assert_eq!(broken.delay_witnesses.len(), 3);
    assert!(broken
        .delay_witnesses
        .iter()
        .all(|w| !w.pass && w.total_shift_ms == 0.0 && w.residual_ms == -40.0));
    let (mut capture, csv) = fixture();
    for input in capture.inputs.iter_mut().filter(|i| i.delay_k == 4) {
        input.input_qpc += 10_000;
    }
    let signed = analyze(&capture, &csv).unwrap();
    assert!(signed.instrument_valid, "{:?}", signed.failures);
    assert!(signed.delay_witnesses.iter().all(|w| w.residual_ms == -1.0));
}

#[test]
fn rehearsal_perf_m16_refuses_ambiguous_missing_unknown_display_and_response_before_input() {
    let (capture, csv) = fixture();
    let missing = mutate_csv(&csv, 65, 5, &(present(65) + 100).to_string());
    let result = analyze(&capture, &missing).unwrap();
    assert!(!result.instrument_valid);
    assert!(!result.unmatched_present_csv_lines.is_empty());
    let duplicate = csv.clone() + csv.lines().nth(66).unwrap() + "\n";
    let result = analyze(&capture, &duplicate).unwrap();
    assert!(!result.instrument_valid);
    assert_eq!(result.frames[65].matching_csv_lines.len(), 2);
    let unavailable = analyze(&capture, &mutate_csv(&csv, 65, 6, "0.00000000000000")).unwrap();
    assert!(!unavailable.instrument_valid);
    assert!(unavailable
        .failures
        .iter()
        .any(|s| s.contains("DISPLAYED timestamp unavailable")));
    let (mut capture, csv) = fixture();
    capture.inputs[0].input_qpc = present(65) + 100;
    let before = analyze(&capture, &csv).unwrap();
    assert!(!before.instrument_valid);
    assert!(before.failures.iter().any(|s| s.contains("before input")));
}

#[test]
fn rehearsal_perf_m16_refuses_scene_resident_capture_reset_and_stopped_display() {
    for component in 0..3 {
        let (mut capture, csv) = fixture();
        match component {
            0 => capture.frames[65].epoch.scene += 1,
            1 => capture.frames[65].epoch.resident += 1,
            _ => capture.frames[65].epoch.capture += 1,
        }
        let result = analyze(&capture, &csv).unwrap();
        assert!(!result.instrument_valid);
        assert!(result.frames[65]
            .rejection
            .as_ref()
            .unwrap()
            .contains("stale/replaced"));
    }
    let (mut capture, csv) = fixture();
    capture.stop_reason = Some("resident reset".into());
    assert!(!analyze(&capture, &csv).unwrap().instrument_valid);
    capture.stop_reason = Some("operator stop".into());
    capture.stopped_qpc = Some(present(65) + 500);
    let stopped = analyze(&capture, &csv).unwrap();
    assert!(!stopped.instrument_valid);
    assert_eq!(
        stopped.frames[65].rejection.as_deref(),
        Some("DISPLAYED occurs after capture stop")
    );
    capture.stopped_qpc = None;
    assert!(!analyze(&capture, &csv).unwrap().instrument_valid);
}
