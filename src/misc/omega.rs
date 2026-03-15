
// Ω approximation state (u8-coded)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OmegaStep {
    pub n: usize,         				// prefix length (meta; not used in arithmetic)
    pub delta: U8Interval,  			// u8 interval increment δ_n
}