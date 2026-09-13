//! Offline B0b instrument validation. No runtime capture, renderer, or simulator.
//! PresentMon 2.5.1 v1 CSV QPCTime is PresentStartTime; displayed QPC is that
//! integer plus the recorded msUntilDisplayed offset. Never use submission as display.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const PRESENTMON_SHA256: &str =
    "9bec3083069f58f911e6a512f4806db51a27bd096103087bc1d05ef54c80a191";
const INSTRUMENT: &str = "m16-displayed-probe-v1";
const CASES: [u32; 6] = [0, 4, 0, 4, 0, 4];

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Epoch {
    pub capture: u64,
    pub scene: u64,
    pub resident: u64,
    pub client: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub response_id: u64,
    pub trial: usize,
    pub delay_k: u32,
    pub input_qpc: u64,
    pub input_frame: u64,
    pub target_frame: u64,
    pub epoch: Epoch,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Span {
    pub ordinal: usize,
    pub main_frame: u64,
    pub epoch: Epoch,
    pub response_id: Option<u64>,
    pub marker_in_render_data: bool,
    pub marker_pipeline_ready: bool,
    pub begin_qpc: u64,
    pub end_qpc: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AppCapture {
    pub instrument: String,
    pub process_id: u32,
    pub qpc_frequency: u64,
    pub config: Value,
    pub epoch: Epoch,
    pub started_qpc: u64,
    pub stopped_qpc: Option<u64>,
    pub stop_reason: Option<String>,
    pub initial_facts: Value,
    pub final_facts: Value,
    pub inputs: Vec<Input>,
    pub frames: Vec<Span>,
    pub rejected: Vec<Value>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Present {
    pub csv_line: usize,
    pub present_qpc: u64,
    pub displayed_qpc: Option<u64>,
    pub swap_chain: String,
    pub runtime: String,
    pub mode: String,
    pub dropped: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct FrameJoin {
    pub ordinal: usize,
    pub main_frame: u64,
    pub matching_csv_lines: Vec<usize>,
    pub present: Option<Present>,
    pub rejection: Option<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Response {
    pub response_id: u64,
    pub trial: usize,
    pub delay_k: u32,
    pub input_qpc: u64,
    pub input_frame: u64,
    pub response_frame: u64,
    pub csv_line: usize,
    pub present_qpc: u64,
    pub displayed_qpc: u64,
    pub app_handling_ms: f64,
    pub submit_to_displayed_ms: f64,
    pub total_ms: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct DelayWitness {
    pub pair: usize,
    pub baseline_response: u64,
    pub delayed_response: u64,
    pub observed_k_frame_ms: f64,
    pub largest_observed_frame_ms: f64,
    pub total_shift_ms: f64,
    pub handling_shift_ms: f64,
    pub display_shift_ms: f64,
    pub residual_ms: f64,
    pub tolerance_ms: f64,
    pub pass: bool,
}
#[derive(Debug, Serialize)]
pub struct Report {
    pub instrument_valid: bool,
    pub instrument: String,
    pub config: Value,
    pub process_id: u32,
    pub qpc_frequency: u64,
    pub frames: Vec<FrameJoin>,
    pub responses: Vec<Response>,
    pub delay_witnesses: Vec<DelayWitness>,
    pub unmatched_present_csv_lines: Vec<usize>,
    pub failures: Vec<String>,
    pub retained_app_rejections: Vec<Value>,
}

fn csv_fields(line: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut closed = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                    closed = true;
                }
            } else {
                field.push(c);
            }
        } else if c == ',' {
            fields.push(std::mem::take(&mut field));
            closed = false;
        } else if c == '"' && field.is_empty() && !closed {
            quoted = true;
        } else if closed || c == '"' {
            return Err("malformed CSV quoting".into());
        } else {
            field.push(c);
        }
    }
    if quoted {
        return Err("unterminated CSV field".into());
    }
    fields.push(field);
    Ok(fields)
}

/// Decimal arithmetic avoids converting the absolute QPC clock through f64.
/// CSV rounding is qualified to one QPC tick; no clamping or guessed zero time.
pub fn displayed_delta_qpc(ms: &str, frequency: u64) -> Result<u64, String> {
    let (whole, fraction) = ms.split_once('.').unwrap_or((ms, ""));
    if whole.is_empty()
        || fraction.len() > 18
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return Err("display duration is not a finite nonnegative decimal".into());
    }
    let scale = 10u128.pow(fraction.len() as u32);
    let fraction_value = if fraction.is_empty() {
        0
    } else {
        fraction
            .parse::<u128>()
            .map_err(|_| "display fraction overflow")?
    };
    let numerator = whole
        .parse::<u128>()
        .ok()
        .and_then(|w| w.checked_mul(scale))
        .and_then(|w| w.checked_add(fraction_value))
        .and_then(|v| v.checked_mul(u128::from(frequency)))
        .ok_or("display duration overflow")?;
    let denominator = 1_000u128 * scale;
    let ticks = numerator
        .checked_add(denominator / 2)
        .ok_or("display rounding overflow")?
        / denominator;
    u64::try_from(ticks).map_err(|_| "display QPC delta overflow".into())
}

pub fn parse_presentmon(csv: &str, pid: u32, frequency: u64) -> Result<Vec<Present>, String> {
    if frequency == 0 {
        return Err("QPC frequency is zero".into());
    }
    let mut lines = csv.trim_start_matches('\u{feff}').lines();
    let header = csv_fields(
        lines
            .next()
            .ok_or("empty PresentMon CSV")?
            .trim_end_matches('\r'),
    )?;
    let columns: BTreeMap<_, _> = header
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();
    if columns.len() != header.len() {
        return Err("duplicate CSV header".into());
    }
    for field in [
        "Application",
        "ProcessID",
        "SwapChainAddress",
        "Runtime",
        "Dropped",
        "QPCTime",
        "msUntilDisplayed",
        "PresentMode",
    ] {
        if !columns.contains_key(field) {
            return Err(format!("missing PresentMon v1 field {field}"));
        }
    }
    let mut out = Vec::new();
    for (n, line) in lines.enumerate() {
        let row = csv_fields(line.trim_end_matches('\r'))?;
        let line_no = n + 2;
        if row.len() != header.len() {
            return Err(format!("CSV line {line_no}: wrong field count"));
        }
        let get = |name: &str| row[columns[name]].as_str();
        let actual_pid = get("ProcessID").parse::<u32>().map_err(|_| "invalid PID")?;
        if actual_pid != pid {
            return Err(format!(
                "CSV line {line_no}: foreign PID in process-filtered capture"
            ));
        }
        if !get("Application").eq_ignore_ascii_case("simthing-studio.exe") {
            return Err(format!("CSV line {line_no}: wrong application"));
        }
        let dropped = match get("Dropped") {
            "0" => false,
            "1" => true,
            _ => return Err(format!("CSV line {line_no}: unknown final state")),
        };
        let present_qpc = get("QPCTime")
            .parse::<u64>()
            .map_err(|_| "invalid integer present QPC")?;
        let displayed_qpc = if dropped {
            None
        } else {
            let delta = displayed_delta_qpc(get("msUntilDisplayed"), frequency)?;
            if delta == 0 {
                None
            } else {
                Some(
                    present_qpc
                        .checked_add(delta)
                        .ok_or("displayed QPC overflow")?,
                )
            }
        };
        out.push(Present {
            csv_line: line_no,
            present_qpc,
            displayed_qpc,
            swap_chain: get("SwapChainAddress").into(),
            runtime: get("Runtime").into(),
            mode: get("PresentMode").into(),
            dropped,
        });
    }
    if out.is_empty() {
        return Err("no process-filtered present rows".into());
    }
    out.sort_by_key(|p| p.present_qpc);
    Ok(out)
}

fn ms(ticks: u64, frequency: u64) -> f64 {
    ticks as f64 * 1_000.0 / frequency as f64
}
fn qualification(c: &AppCapture) -> Vec<String> {
    let mut errors = vec![];
    if c.instrument != INSTRUMENT {
        errors.push("unknown app instrument version".into());
    }
    if c.qpc_frequency == 0 {
        errors.push("QPC frequency unavailable".into());
    }
    if c.stopped_qpc.is_none_or(|q| q <= c.started_qpc)
        || c.stop_reason.as_deref() != Some("operator stop")
    {
        errors.push(
            "capture lacks a valid operator stop; reset/stopped/invalid capture refused".into(),
        );
    }
    if c.inputs.len() != CASES.len() {
        errors.push("six baseline/delayed inputs required".into());
    }
    if !matches!(c.epoch.client.as_str(), "Egui" | "Native") {
        errors.push("unknown client".into());
    }
    if c.initial_facts["scene"].as_u64() != Some(c.epoch.scene)
        || c.initial_facts["resident_epoch"].as_u64() != Some(c.epoch.resident)
        || c.initial_facts["native_enabled"].as_bool() != Some(c.epoch.client == "Native")
        || c.initial_facts["paused"].as_bool() != Some(true)
        || c.initial_facts["source_identity"] != c.config["expected_source_identity"]
    {
        errors.push("app epoch/source/client qualification mismatch".into());
    }
    for key in [
        "scene",
        "resident_epoch",
        "source_identity",
        "profile_identity",
        "dependencies",
        "native_enabled",
        "paused",
        "window_pixels",
        "scale_factor",
        "panels_hidden",
        "adapter",
    ] {
        if c.initial_facts[key].is_null() || c.initial_facts[key] != c.final_facts[key] {
            errors.push(format!("changed or missing runtime qualification: {key}"));
        }
    }
    for facts in [&c.initial_facts, &c.final_facts] {
        if facts["reset_pending"] != false || facts["focused"] != true {
            errors.push("reset/focus boundary refused".into());
        }
    }
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
        if c.config["metadata"][key]
            .as_str()
            .is_none_or(|s| s.trim().is_empty())
        {
            errors.push(format!("missing metadata: {key}"));
        }
    }
    if c.config["metadata"]["presentmon_version"] != "2.5.1"
        || c.config["metadata"]["presentmon_sha256"] != PRESENTMON_SHA256
    {
        errors.push("PresentMon version/hash qualification mismatch".into());
    }
    for rejection in &c.rejected {
        if rejection["reason"] != "in-flight frame from before this capture epoch; excluded" {
            errors.push(format!("app rejected an event: {rejection}"));
        }
    }
    errors
}

pub fn analyze(c: &AppCapture, csv: &str) -> Result<Report, String> {
    let presents = parse_presentmon(csv, c.process_id, c.qpc_frequency)?;
    let mut report = Report {
        instrument_valid: false,
        instrument: c.instrument.clone(),
        config: c.config.clone(),
        process_id: c.process_id,
        qpc_frequency: c.qpc_frequency,
        frames: vec![],
        responses: vec![],
        delay_witnesses: vec![],
        unmatched_present_csv_lines: vec![],
        failures: qualification(c),
        retained_app_rejections: c.rejected.clone(),
    };
    let mut used = BTreeSet::new();
    let mut chains = BTreeSet::new();
    for (index, span) in c.frames.iter().enumerate() {
        let begin = presents.partition_point(|p| p.present_qpc < span.begin_qpc);
        let end = presents.partition_point(|p| p.present_qpc <= span.end_qpc);
        let matches = if begin <= end {
            &presents[begin..end]
        } else {
            &[]
        };
        let mut join = FrameJoin {
            ordinal: span.ordinal,
            main_frame: span.main_frame,
            matching_csv_lines: matches.iter().map(|p| p.csv_line).collect(),
            present: None,
            rejection: None,
        };
        let reason = if span.ordinal != index || span.epoch != c.epoch {
            Some("stale/replaced capture, scene or resident epoch")
        } else if span.end_qpc < span.begin_qpc || span.begin_qpc < c.started_qpc {
            Some("invalid app QPC span")
        } else if c.stopped_qpc.is_none_or(|q| span.end_qpc > q) {
            Some("present span crosses stopped capture")
        } else if index > 0
            && (span.main_frame <= c.frames[index - 1].main_frame
                || span.begin_qpc <= c.frames[index - 1].end_qpc)
        {
            Some("overlapping/reversed app frame spans")
        } else if matches.len() != 1 {
            Some("unmatched or ambiguous present span; nearest-neighbor join refused")
        } else {
            None
        };
        if let Some(reason) = reason {
            join.rejection = Some(reason.into());
        } else {
            let present = matches[0].clone();
            if !used.insert(present.csv_line) {
                join.rejection = Some("present row reused by another frame".into());
            }
            chains.insert(present.swap_chain.clone());
            if present.runtime != "DXGI" || present.mode.is_empty() || present.mode == "Unknown" {
                join.rejection = Some("unqualified Windows display path".into());
            } else if present.dropped {
                join.rejection = Some("dropped/not-displayed present".into());
            } else if present.displayed_qpc.is_none() {
                join.rejection = Some("DISPLAYED timestamp unavailable".into());
            } else if present
                .displayed_qpc
                .is_some_and(|q| c.stopped_qpc.is_none_or(|stop| q > stop))
            {
                join.rejection = Some("DISPLAYED occurs after capture stop".into());
            }
            join.present = Some(present);
        }
        if let Some(reason) = join.rejection.as_deref() {
            if !matches!(
                reason,
                "dropped/not-displayed present"
                    | "present span crosses stopped capture"
                    | "DISPLAYED occurs after capture stop"
            ) {
                report
                    .failures
                    .push(format!("span {}: {reason}", span.ordinal));
            }
        }
        report.frames.push(join);
    }
    if chains.len() != 1 {
        report
            .failures
            .push("one qualified swap chain required".into());
    }
    let complete_spans: Vec<_> = c
        .frames
        .iter()
        .filter(|f| c.stopped_qpc.is_some_and(|stop| f.end_qpc <= stop))
        .collect();
    if let (Some(first), Some(last)) = (complete_spans.first(), complete_spans.last()) {
        for p in &presents {
            if (first.begin_qpc..=last.end_qpc).contains(&p.present_qpc)
                && !used.contains(&p.csv_line)
            {
                report.unmatched_present_csv_lines.push(p.csv_line);
            }
        }
    } else {
        report.failures.push("no app present spans".into());
    }
    if !report.unmatched_present_csv_lines.is_empty() {
        report
            .failures
            .push("present rows outside exact app spans: correlation is not a bijection".into());
    }
    let ids: BTreeSet<_> = c.inputs.iter().map(|i| i.response_id).collect();
    if ids.len() != c.inputs.len() {
        report.failures.push("duplicate response identity".into());
    }
    if c.frames
        .iter()
        .any(|f| f.response_id.is_some_and(|id| !ids.contains(&id)))
    {
        report
            .failures
            .push("frame refers to a foreign response identity".into());
    }
    for (n, input) in c.inputs.iter().enumerate() {
        if input.epoch != c.epoch
            || input.trial != n
            || CASES.get(n) != Some(&input.delay_k)
            || input.input_frame.checked_add(u64::from(input.delay_k)) != Some(input.target_frame)
            || input.input_qpc < c.started_qpc
            || c.stopped_qpc.is_none_or(|q| input.input_qpc >= q)
            || (n > 0 && input.input_qpc <= c.inputs[n - 1].input_qpc)
        {
            report.failures.push(format!(
                "input {} has invalid identity/epoch/QPC/delay",
                input.response_id
            ));
            continue;
        }
        let mut found = None;
        let mut invalid = false;
        for (span, join) in c
            .frames
            .iter()
            .zip(&report.frames)
            .filter(|(s, _)| s.response_id == Some(input.response_id))
        {
            if !span.marker_in_render_data || !span.marker_pipeline_ready {
                continue;
            }
            if span.main_frame < input.target_frame || span.begin_qpc < input.input_qpc {
                report.failures.push(format!(
                    "response {} before input/target frame",
                    input.response_id
                ));
                invalid = true;
                break;
            }
            if join.rejection.as_deref() == Some("dropped/not-displayed present") {
                continue;
            }
            if let Some(reason) = &join.rejection {
                report
                    .failures
                    .push(format!("response {}: {reason}", input.response_id));
                invalid = true;
                break;
            }
            let p = join.present.as_ref().unwrap();
            let displayed = p.displayed_qpc.unwrap();
            if p.present_qpc < input.input_qpc || displayed < p.present_qpc {
                report.failures.push(format!(
                    "response {} has reversed causal timestamps",
                    input.response_id
                ));
                invalid = true;
                break;
            }
            found = Some(Response {
                response_id: input.response_id,
                trial: n,
                delay_k: input.delay_k,
                input_qpc: input.input_qpc,
                input_frame: input.input_frame,
                response_frame: span.main_frame,
                csv_line: p.csv_line,
                present_qpc: p.present_qpc,
                displayed_qpc: displayed,
                app_handling_ms: ms(p.present_qpc - input.input_qpc, c.qpc_frequency),
                submit_to_displayed_ms: ms(displayed - p.present_qpc, c.qpc_frequency),
                total_ms: ms(displayed - input.input_qpc, c.qpc_frequency),
            });
            break;
        }
        if !invalid {
            if let Some(response) = found {
                report.responses.push(response);
            } else {
                report.failures.push(format!(
                    "response {} never has a matched DISPLAYED present",
                    input.response_id
                ));
            }
        }
    }
    if report.responses.len() == CASES.len() {
        for pair in 0..3 {
            let baseline = &report.responses[pair * 2];
            let delayed = &report.responses[pair * 2 + 1];
            match delay_witness(pair, baseline, delayed, &report.frames, c.qpc_frequency) {
                Ok(w) => {
                    if !w.pass {
                        report
                            .failures
                            .push(format!("K-frame falsifier failed for pair {pair}"));
                    }
                    report.delay_witnesses.push(w);
                }
                Err(e) => report.failures.push(format!("pair {pair}: {e}")),
            }
        }
    }
    report.instrument_valid = report.failures.is_empty()
        && report.delay_witnesses.len() == 3
        && report.delay_witnesses.iter().all(|w| w.pass);
    Ok(report)
}

fn delay_witness(
    pair: usize,
    baseline: &Response,
    delayed: &Response,
    frames: &[FrameJoin],
    frequency: u64,
) -> Result<DelayWitness, String> {
    let clock = |frame| {
        frames
            .iter()
            .find(|f| f.main_frame == frame)
            .and_then(|f| f.present.as_ref())
            .map(|p| p.present_qpc)
            .ok_or("missing rendered frame in the K-frame interval")
    };
    let mut largest = 0;
    for r in [baseline, delayed] {
        for f in r.input_frame..=r.response_frame {
            let step = clock(f + 1)?
                .checked_sub(clock(f)?)
                .ok_or("reversed observed frame clock")?;
            if step == 0 {
                return Err("zero observed frame interval".into());
            }
            largest = largest.max(step);
        }
    }
    let expected = clock(delayed.input_frame + u64::from(delayed.delay_k))?
        .checked_sub(clock(delayed.input_frame)?)
        .ok_or("reversed K-frame clock")?;
    let expected_ms = ms(expected, frequency);
    let largest_ms = ms(largest, frequency);
    // Predeclared bound: <= one frame of input phase difference plus <= one frame
    // of display-queue phase difference; two QPC ticks cover endpoint rounding.
    // Refuse an interval so noisy that zero injected effect could satisfy it.
    let tolerance_ms = 2.0 * largest_ms + ms(2, frequency);
    let total_shift_ms = delayed.total_ms - baseline.total_ms;
    let residual_ms = total_shift_ms - expected_ms;
    Ok(DelayWitness {
        pair,
        baseline_response: baseline.response_id,
        delayed_response: delayed.response_id,
        observed_k_frame_ms: expected_ms,
        largest_observed_frame_ms: largest_ms,
        total_shift_ms,
        handling_shift_ms: delayed.app_handling_ms - baseline.app_handling_ms,
        display_shift_ms: delayed.submit_to_displayed_ms - baseline.submit_to_displayed_ms,
        residual_ms,
        tolerance_ms,
        pass: expected_ms > tolerance_ms
            && total_shift_ms > 0.0
            && residual_ms.abs() <= tolerance_ms,
    })
}
