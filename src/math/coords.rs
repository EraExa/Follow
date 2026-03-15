
/*
	RA/Dec/z → XYZ using only 3x u8 LUTs:
		(I)		cos(θ),
		(II)	sin(θ), and
		(III)	a z→distance LUT
*/

/// RA,Dec (0..255) → direction (x,y,z) each in 0..255 via LUTs.
/// z_bin (0..255) → comoving distance bin (0..255) via LUT.
pub fn ra_dec_to_dir(ra: u8, dec: u8) -> (u8,u8,u8) {
    // lookups: cos/sin from 256-entry LUTs
    // Compose into xyz in u8 ring with wrapping semantics.
    todo!()
}

pub fn z_to_dist(z: u8) -> u8 {
    // Precomputed cosmology LUT (flat ΛCDM baked in) z_bin -> distance_bin
    todo!()
}
