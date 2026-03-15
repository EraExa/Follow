
// TODO: IMPLEMENT 3D INTERVAL ARITHMETIC-POINT CLOUD DATA; WITH RGBAColor-data field
//			`../core/u8lut_rgbacolor.rs`
//			using R/G/B/A-LUTs: RLUT/GLUT/BLUT/ALUT (x,y,z) + width.
//			R/G/B-LUT:	hue/sat mapped from (x, y, z) with morton z-ordering.
//			ALUT:		Alpha channel mapped from `width of Interval`: Interval::width();

#[derive(Clone, Debug)]
pub struct Object3D {
    pub support: Interval, // placeholder: spatial bound encoded as interval
}

#[derive(Clone, Debug)]
pub struct Volume3D {
    pub support: Interval, // volumetric bound
    pub density: Interval, // interval-coded density
}

#[derive(Clone, Debug)]
pub struct Animated {
    pub support: Interval,
    pub density: Interval,
    pub time: Interval, // animation time interval
}
