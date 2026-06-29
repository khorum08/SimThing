//! CPU-oracle threshold event fixtures for unit tests (not production minting).

#[cfg(test)]
pub(crate) mod fixtures {
    use simthing_gpu::{
        cpu_oracle_threshold_events, ThresholdEvent, ThresholdRegistration, DIR_UPWARD,
        THRESH_BUF_VALUES,
    };

    fn lane_index(slot: u32, col: u32, n_dims: u32) -> usize {
        (slot * n_dims + col) as usize
    }

    fn buffer_len(max_slot: u32, n_dims: u32) -> usize {
        ((max_slot + 1) * n_dims) as usize
    }

    fn max_slot(crossings: &[(u32, u32, f32, u32)]) -> u32 {
        crossings
            .iter()
            .map(|(slot, _, _, _)| *slot)
            .max()
            .unwrap_or(0)
    }

    pub fn upward_crossing(
        slot: u32,
        col: u32,
        curr_value: f32,
        event_kind: u32,
        n_dims: usize,
    ) -> ThresholdEvent {
        upward_crossings(&[(slot, col, curr_value, event_kind)], n_dims)
            .into_iter()
            .next()
            .expect("crossing event")
    }

    pub fn upward_crossings(
        crossings: &[(u32, u32, f32, u32)],
        n_dims: usize,
    ) -> Vec<ThresholdEvent> {
        let n_dims = n_dims as u32;
        let max_slot = max_slot(crossings);
        let len = buffer_len(max_slot, n_dims);
        let mut prev = vec![0.0f32; len];
        let mut curr = vec![0.0f32; len];
        let mut regs = Vec::with_capacity(crossings.len());
        for &(slot, col, curr_value, event_kind) in crossings {
            let idx = lane_index(slot, col, n_dims);
            let threshold = curr_value - 0.01;
            prev[idx] = threshold;
            curr[idx] = curr_value;
            regs.push(ThresholdRegistration {
                slot,
                col,
                threshold,
                direction: DIR_UPWARD,
                event_kind,
                buffer: THRESH_BUF_VALUES,
            });
        }
        cpu_oracle_threshold_events(&prev, &curr, &[], &[], n_dims, &regs)
    }

    pub fn duplicate_upward_crossing(
        slot: u32,
        col: u32,
        curr_value: f32,
        event_kind: u32,
        n_dims: usize,
    ) -> Vec<ThresholdEvent> {
        let n_dims = n_dims as u32;
        let idx = lane_index(slot, col, n_dims);
        let len = buffer_len(slot, n_dims);
        let mut prev = vec![0.0f32; len];
        let mut curr = vec![0.0f32; len];
        let threshold = curr_value - 0.01;
        prev[idx] = threshold;
        curr[idx] = curr_value;
        let reg = ThresholdRegistration {
            slot,
            col,
            threshold,
            direction: DIR_UPWARD,
            event_kind,
            buffer: THRESH_BUF_VALUES,
        };
        cpu_oracle_threshold_events(&prev, &curr, &[], &[], n_dims, &[reg, reg])
    }
}
