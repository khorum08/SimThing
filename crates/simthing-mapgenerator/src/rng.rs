//! Pinned deterministic RNG for MapGeneratorCLI (producer-side only).
//!
//! Algorithm: SplitMix64 (Vigna, 2015) — stable across platforms, no system entropy.

/// User-facing seed for reproducible MapGen producer runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapGenSeed(pub u64);

impl MapGenSeed {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
}

/// Deterministic PRNG wrapper (SplitMix64 state machine).
#[derive(Debug, Clone)]
pub struct MapGenRng {
    state: u64,
}

impl MapGenRng {
    pub fn from_seed(seed: MapGenSeed) -> Self {
        Self { state: seed.0 }
    }

    /// Next u64 in the pinned SplitMix64 sequence.
    pub fn next_u64(&mut self) -> u64 {
        splitmix64(&mut self.state)
    }

    pub fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }

    /// Uniform `f64` in `[0, 1)` from the high 53 bits (no system RNG).
    pub fn next_f64(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / (1u64 << 53) as f64;
        (self.next_u64() >> 11) as f64 * SCALE
    }

    /// Uniform index in `[0, upper)`; returns `0` when `upper == 0`.
    pub fn gen_index(&mut self, upper: u32) -> u32 {
        if upper == 0 {
            return 0;
        }
        (self.next_u64() % upper as u64) as u32
    }
}

/// SplitMix64 — pinned for cross-run/cross-platform stability in tests.
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
