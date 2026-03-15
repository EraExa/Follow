use crate::core::u8lut_core::*;
use crate::core::u8lut_extended::*;
use crate::core::interval::Interval;

#[derive(Copy, Clone)]
pub enum Fusion {
    Hull,
    Intersect,
    Add,
}

#[inline]
pub fn fuse(a: Interval, b: Interval, policy: Fusion) -> (Interval, u8) {
    match policy {
        Fusion::Hull      => (i_hull(a,b), 1),
        Fusion::Intersect => i_intersect(a,b),
        Fusion::Add       => (i_add(a,b), 1),
    }
}

/*
Transfer function: Interval → RGBA(u8) using precomputed LUTs, e.g.,
	width w = wm1(i) to opacity via ALUT[w]
	location (lo/hi) into hue/saturation via RLUT/GLUT/BLUT
*/

pub struct TransferLUTs {
    pub rlut: [u8; 256],
    pub glut: [u8; 256],
    pub blut: [u8; 256],
    pub alut: [u8; 256],
}

impl TransferLUTs {
    #[inline]
    pub fn map(&self, iv: Interval) -> (u8,u8,u8,u8) {
        let w = wm1(iv);
        let r = self.rlut[w as usize];
        let g = self.glut[w as usize];
        let b = self.blut[w as usize];
        let a = self.alut[w as usize];
        (r,g,b,a)
    }
}
