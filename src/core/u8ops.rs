#![allow(non_upper_case_globals)]

pub use crate::core::interval::Interval;

// ---- LUT declarations (provide or generate at build time) ----
// Assume these are 256x256 u8 tables (or consts from a build script).
pub static LUT_MIN: [[u8; 256]; 256] = [[0; 256]; 256];
pub static LUT_MAX: [[u8; 256]; 256] = [[0; 256]; 256];
pub static LUT_ADD: [[u8; 256]; 256] = [[0; 256]; 256];      // wrapping add
pub static LUT_SUB: [[u8; 256]; 256] = [[0; 256]; 256];      // wrapping sub
pub static LUT_SAT_ADD: [[u8; 256]; 256] = [[0; 256]; 256];  // saturating add
pub static LUT_SAT_SUB: [[u8; 256]; 256] = [[0; 256]; 256];  // saturating sub
pub static LUT_LE: [[u8; 256]; 256] = [[0; 256]; 256];       // a<=b ? 1:0
pub static LUT_GE: [[u8; 256]; 256] = [[0; 256]; 256];       // a>=b ? 1:0
pub static LUT_LT: [[u8; 256]; 256] = [[0; 256]; 256];       // a<b  ? 1:0

#[inline] pub fn min_u8(a: u8, b: u8) -> u8 { LUT_MIN[a as usize][b as usize] }
#[inline] pub fn max_u8(a: u8, b: u8) -> u8 { LUT_MAX[a as usize][b as usize] }
#[inline] pub fn sat_add(a: u8, b: u8) -> u8 { LUT_SAT_ADD[a as usize][b as usize] }
#[inline] pub fn sat_sub(a: u8, b: u8) -> u8 { LUT_SAT_SUB[a as usize][b as usize] }

// distance forward on the circle (x -> y; inclusive indices from x)
#[inline] pub fn dfwd(x: u8, y: u8) -> u8 { LUT_SUB[y as usize][x as usize] }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Interval { pub lo: u8, pub hi: u8 }

#[inline] pub fn wm1(x: Interval) -> u8 { LUT_SUB[x.hi as usize][x.lo as usize] }

// membership: x ∈ arc(lo..hi) iff dfwd(lo,x) <= wm1
#[inline] pub fn contains_i(a: Interval, x: u8) -> u8 {
    let d = dfwd(a.lo, x);
    let w = wm1(a);
    LUT_LE[d as usize][w as usize]
}

#[inline] pub fn i_add(a: Interval, b: Interval) -> Interval {
    let lo = LUT_ADD[a.lo as usize][b.lo as usize];
    let w  = LUT_SAT_ADD[wm1(a) as usize][wm1(b) as usize];
    let hi = LUT_ADD[lo as usize][w as usize];
    Interval { lo, hi }
}

#[inline] pub fn i_neg(b: Interval) -> Interval {
    let lo = LUT_SUB[0][b.hi as usize];
    let hi = LUT_SUB[0][b.lo as usize];
    Interval { lo, hi }
}

#[inline] pub fn i_sub(a: Interval, b: Interval) -> Interval { i_add(a, i_neg(b)) }

// intersection and an ok-flag (ok==0 means empty)
#[inline]
pub fn i_intersect(a: Interval, b: Interval) -> (Interval, u8) {
    let s = a.lo;
    let a_w = wm1(a);
    let b_w = wm1(b);

    let off_b_lo = dfwd(s, b.lo);
    let off_b_hi = dfwd(s, b.hi);

    let in_a_blo = LUT_LE[off_b_lo as usize][a_w as usize];
    let off_s_in_b = dfwd(b.lo, s);
    let in_b_s = LUT_LE[off_s_in_b as usize][b_w as usize];

    // any overlap?
    let sel_blo = in_a_blo;
    let sel_s = (in_b_s) & (sel_blo ^ 1);
    let any = sel_blo | sel_s;

    let start_off = if sel_blo == 1 { off_b_lo } else { 0 };
    let end_off = min_u8(a_w, off_b_hi);

    // end_off >= start_off ?
    let ok = any & LUT_GE[end_off as usize][start_off as usize];

    let start = LUT_ADD[s as usize][start_off as usize];
    let span = LUT_SUB[end_off as usize][start_off as usize];

    let out_lo = if ok == 1 { start } else { 0 };
    let out_hi = if ok == 1 { LUT_ADD[start as usize][span as usize] } else { 0 };

    (Interval { lo: out_lo, hi: out_hi }, ok ^ 1)
}

#[inline]
pub fn i_hull(a: Interval, b: Interval) -> Interval {
    let a_w = wm1(a);
    let b_w = wm1(b);

    let off_b_lo = dfwd(a.lo, b.lo);
    let off_a_lo = dfwd(b.lo, a.lo);

    let end1 = max_u8(a_w, LUT_ADD[off_b_lo as usize][b_w as usize]);
    let end2 = max_u8(b_w, LUT_ADD[off_a_lo as usize][a_w as usize]);

    let choose2 = LUT_LT[end2 as usize][end1 as usize];

    let lo = if choose2 == 1 { b.lo } else { a.lo };
    let w  = if choose2 == 1 { end2 } else { end1 };
    let w  = min_u8(w, 255);

    let hi = LUT_ADD[lo as usize][w as usize];
    Interval { lo, hi }
}
