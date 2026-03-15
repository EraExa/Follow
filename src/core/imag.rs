// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Axel Schmidt Selmer-Anderssen
// AI-Generated-By: M365 Copilot (GPT-5 reasoning model), assistance date: 2026-01-23
// AI-Generation-Notes: Portions of this file were drafted or refactored with AI assistance.
//                      Review and modifications were performed by human maintainers.


use serde::{Serialize, Deserialize};

pub const IMAG_SIZE:    u8 = 13;
pub const IMAG_LEVELS:  u8 = 5;
pub const IMAG_COLUMNS: u8 = 3;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Imag {
    One = 0,
    I = 1, J = 2, K = 3,
    M = 4, N = 5, O = 6,
    R = 7, S = 8, T = 9,
    E = 10, F = 11, G = 12,
}

const IMAG_TO_CHAR: [char; 13] = [
    '1',
    'i','j','k',
    'm','n','o',
    'r','s','t',
    'e','f','g',
];

const IMAG_FROM_DISC: [Imag; 13] = [
    Imag::One,
    Imag::I, Imag::J, Imag::K,
    Imag::M, Imag::N, Imag::O,
    Imag::R, Imag::S, Imag::T,
    Imag::E, Imag::F, Imag::G,
];

#[inline]
pub const fn imag_discr(u: Imag) -> u8 {
    (u as u8) % IMAG_SIZE
}

#[inline]
pub const fn imag_is_one(u: Imag) -> u8 {
    (imag_discr(u) == 0) as u8
}

#[inline]
pub const fn imag_level(u: Imag) -> u8 {
    let d = imag_discr(u);
    (d + (IMAG_COLUMNS - 1)) / IMAG_COLUMNS
}

#[inline]
pub const fn imag_col(u: Imag) -> u8 {
    imag_discr(u) % IMAG_COLUMNS
}

#[inline]
pub const fn imag_from_level_col(level: u8, col: u8) -> Imag {
    let lvl = level % IMAG_LEVELS;
    if lvl == 0 { return Imag::One; }
    
    let c = col % IMAG_COLUMNS; // 0,1,2
    let base = lvl * IMAG_COLUMNS; // 3*L
    
    // Mapping: Col 0 -> base (e.g. 3->K), Col 1 -> base-2 (1->I), Col 2 -> base-1 (2->J)
    // Formula: if c==0 { base } else { base - 3 + c }
    // Since u8, base >= 3 because lvl >= 1.
    let disc = if c == 0 { base } else { base - 3 + c };
    
    IMAG_FROM_DISC[disc as usize]
}

#[inline]
pub const fn imag_as_char(u: Imag) -> char {
    IMAG_TO_CHAR[imag_discr(u) as usize]
}

#[inline]
pub fn imag_from_char(c: char) -> Imag {
    match c {
        '1' => Imag::One,
        'i' => Imag::I, 'j' => Imag::J, 'k' => Imag::K,
        'm' => Imag::M, 'n' => Imag::N, 'o' => Imag::O,
        'r' => Imag::R, 's' => Imag::S, 't' => Imag::T,
        'e' => Imag::E, 'f' => Imag::F, 'g' => Imag::G,
        _   => Imag::One,
    }
}

/// Branchless Hamilton product with promotion tracking (for ringed permutation groups).
/// Returns: (imag_out, flip, target_level, promotions_a, promotions_b)
/// flip ∈ {+1, -1} as i8
#[inline]
pub const fn mul_imags(a: Imag, b: Imag) -> (Imag, i8, u8, u8, u8) {
    let da = imag_discr(a);
    let db = imag_discr(b);

    let ma = (da != 0) as u8; // a != One
    let mb = (db != 0) as u8; // b != One

    let la = imag_level(a);
    let lb = imag_level(b);
    let ca = imag_col(a);
    let cb = imag_col(b);

    // lvl = max(la, lb) branchlessly: la ^ ((la ^ lb) & -(la < lb))
    let lt = (la < lb) as u8;
    let lvl = la ^ ((la ^ lb) & lt.wrapping_neg());

    // column relations
    let eq   = (ca == cb) as u8;
    let next = (cb == ((ca + 1) % IMAG_COLUMNS)) as u8;
    let prev = (cb == ((ca + 2) % IMAG_COLUMNS)) as u8;

    // case masks
    let gen_mask  = ma & mb;             // both not One
    let id_a_mask = ((!ma) & 1) & mb;    // a==One, b!=One
    let id_b_mask = ((!mb) & 1) & ma;    // b==One, a!=One
    let both_one  = ((!ma) & 1) & ((!mb) & 1);

    // flip for general: eq -> -1; next -> +1; prev -> -1
    let flip_general = (next as i8) - (prev as i8) - (eq as i8);

    // out col for general: if eq → One (handled by disc=0), else next? (ca+2)%3 : (ca+1)%3
    let oc_next = (ca + 2) % IMAG_COLUMNS;
    let oc_prev = (ca + 1) % IMAG_COLUMNS;
    let out_col_general = (next * oc_next) + (prev * oc_prev);

    // disc for general; when eq=1, force disc=0 (One)
    let disc_general = (lvl * IMAG_COLUMNS + out_col_general) * ((1 - eq) & 1);
    let disc_id_a    = db; // One * b = b
    let disc_id_b    = da; // a * One = a
    let disc_both    = 0;  // One * One

    // blend discs (mutually exclusive masks)
    let out_disc =
        (gen_mask * disc_general) |
        (id_a_mask * disc_id_a)   |
        (id_b_mask * disc_id_b)   |
        (both_one  * disc_both);

    // blend flips
    let flip =
        (gen_mask as i8) * flip_general +
        (id_a_mask as i8) * 1 +
        (id_b_mask as i8) * 1 +
        (both_one  as i8) * 1;

    // chosen level (needed by policies)
    let out_lvl =
        (gen_mask * lvl) |
        (id_a_mask * lb) |
        (id_b_mask * la) |
        (both_one  * 0);

    // promotions
    let pa = out_lvl.wrapping_sub(la);
    let pb = out_lvl.wrapping_sub(lb);

    (IMAG_FROM_DISC[out_disc as usize], flip, out_lvl, pa, pb)
}

#[inline]
pub const fn imag_inverse(u: Imag) -> (Imag, i8) {
    let non_one = (imag_discr(u) != 0) as u8;
    let flip = 1i8 - ((non_one as i8) << 1); // 1 or -1
    (u, flip)
}

impl Imag {
    #[inline] pub const fn discr(self)  -> u8  { imag_discr(self) }
    #[inline] pub const fn is_one(self) -> u8  { imag_is_one(self) }
    #[inline] pub const fn level(self)  -> u8  { imag_level(self) }
    #[inline] pub const fn col(self)    -> u8  { imag_col(self) }
    #[inline] pub const fn as_char(self)-> char{ imag_as_char(self) }

    #[inline] pub fn from_char(c: char) -> Self { imag_from_char(c) }
}

pub fn default_complex_basis() -> Imag { Imag::I }
pub fn default_quaternion_basis() -> (Imag, Imag, Imag) { (Imag::I, Imag::J, Imag::K) }
