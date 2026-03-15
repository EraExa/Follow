
use crate::core::interval::Interval;

#[derive(Copy, Clone, Debug)]
pub struct BrickKey { pub x: u8, pub y: u8, pub z: u8, pub t: u8 } // 4D brick index (256^4 tiling)

#[derive(Clone)]
pub struct Brick {
    // brick dims are small (e.g., 32^3), but indices/values remain u8
    pub key: BrickKey,
    pub dims: (u8, u8, u8),            // size in voxels
    pub data: Vec<Interval>,           // flattened; |data|=dx*dy*dz
}

pub trait DataSource {
    /// Pull next brick (streaming). Returns None when done.
    fn next_brick(&mut self) -> Option<Brick>;
}

pub trait Quantizer {
    /// Map raw measurement domain to u8 interval using LUTs only.
    /// Example: probability density → (lo,hi) bins; flux → exposure-corrected bins.
    fn quantize(&self, raw_code: u8) -> Interval;
}
