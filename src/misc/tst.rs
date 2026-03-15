
// TODO: Implement Serde::Serialize/Deserialize for *`/memory/`*-Loading/Saving.

use crate::misc::bbst::BBST;
use crate::misc::kd::BBST;
use crate::misc::scheduler::timingwheel::TimingWheels;
use crate::misc::scheduler::calendarqueue::CalendarQueue;
use crate::math::minkowski::OrigoDelta;

#[derive(Default, Clone, Debug)]
pub struct Leaf { pub tree: BBST<AtomNode> }

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub root_id: Option<usize>,
    pub parent_id: Option<usize>,
    pub seed: usize,
    pub tickrate: usize,
    pub name: String,
	
    pub pre: Leaf,
    pub peri: Leaf,
    pub post: Leaf,
	
    pub near_queue: TimingWheels,
    pub far_queue: CalendarQueue,
	
    pub origo_delta: OrigoDelta,
	
    pub enabled: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self { pre: Leaf::default(), peri: Leaf::default(), post: Leaf::default(), origo_delta: crate::OrigoDelta::zero() }
    }
}

/// TST: Ternary Search Tree (each node has strictly ordered PRE/PERI/POST leaves).
#[derive(Default, Clone, Debug)]
pub struct TST {
    pub nodes: Vec<Node>,
}

impl TST {
	/*
	fn apply_boost(ct_xyz: &mut [f64; 4], boost: &Biquaternion<StrictInterval>) {
        let to_beta = |s: &StrictInterval| (s.to_f64() - 128.0) / 128.0;
        let bx = to_beta(&boost.x);
        let by = to_beta(&boost.y);
        let bz = to_beta(&boost.z);
        let b2 = bx*bx + by*by + bz*bz;
        if b2 >= 0.9999 { return; }
        let g = 1.0 / (1.0 - b2).sqrt();
        let t = ct_xyz[0];
        let x = ct_xyz[1];
        let y = ct_xyz[2];
        let z = ct_xyz[3];
        let b_dot_r = bx*x + by*y + bz*z;
        ct_xyz[0] = g * (t - b_dot_r);
        if b2 > 1e-9 {
            let k = (g - 1.0) / b2;
            ct_xyz[1] = x + k * b_dot_r * bx - g * t * bx;
            ct_xyz[2] = y + k * b_dot_r * by - g * t * by;
            ct_xyz[3] = z + k * b_dot_r * bz - g * t * bz;
        }
    }
	*/
	
    /// One iteration step over all nodes: PRE -> PERI -> POST
    pub fn iterate_step(&self) -> Vec<Animated> {
        let mut out = Vec::new();
        for n in &self.nodes {
            let obj = apply_pre(&n.pre, n.origo_delta);
            let vol = apply_peri(&n.peri, &obj);
            let ani = apply_post(&n.post, &vol);
            out.push(ani);
        }
        out
    }
}

// PRE: constraints -> 3D Object
fn apply_pre(pre: &Leaf, origo: crate::OrigoDelta) -> Object3D {
    // Combine all ATOM bounds via hull, offset by origo (u8 add)
    let mut acc = origo.dx; // start with x-extent as anchor; then hull in others
    for n in &pre.tree.nodes {
        acc = crate::core::ops::apply_interval_op("∪", acc, n.atom.bound()).num;
    }
    Object3D { support: acc }
}

// PERI: values -> volumetric object
fn apply_peri(peri: &Leaf, obj: &Object3D) -> Volume3D {
    // Intersect volume density contributions with object support
    let mut dens = obj.support;
    for n in &peri.tree.nodes {
        dens = crate::core::ops::apply_interval_op("∩", dens, n.atom.bound()).num;
    }
    Volume3D { support: obj.support, density: dens }
}

// POST: freedoms -> animated object (temporal structure)
fn apply_post(post: &Leaf, vol: &Volume3D) -> Animated {
    let mut time = Interval { lo: 0, hi: 0 };
    for n in &post.tree.nodes {
        time = crate::core::ops::apply_interval_op("∪", time, n.atom.bound()).num;
    }
    Animated { support: vol.support, density: vol.density, time }
}
