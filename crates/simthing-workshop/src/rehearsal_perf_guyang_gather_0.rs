//! 0088-BASELINE-0 descriptive statistics. Timing never decides semantic admission.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Distribution {
    pub raw_ns: Vec<f64>,
    pub median_ns: f64,
    pub p95_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub population_stddev_ns: f64,
}

/// Preserve signed paired residuals; aggregate only after pairing each sample.
pub fn distribution(raw_ns: Vec<f64>) -> Distribution {
    assert!(raw_ns.len() >= 3 && raw_ns.iter().all(|x| x.is_finite()));
    let mut sorted = raw_ns.clone();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    let mean = sorted.iter().sum::<f64>() / n as f64;
    Distribution {
        median_ns: (sorted[(n - 1) / 2] + sorted[n / 2]) / 2.0,
        p95_ns: sorted[(0.95 * n as f64).ceil() as usize - 1],
        min_ns: sorted[0],
        max_ns: sorted[n - 1],
        population_stddev_ns: (sorted.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64)
            .sqrt(),
        raw_ns,
    }
}
