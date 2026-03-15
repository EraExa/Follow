
use crate::misc::bbst;

#[derive(Default, Clone, Debug)]
pub struct Leaf { pub tree: BBST }

#[derive(Clone, Debug)]
pub struct Node {
    pub pre: Leaf,
    pub peri: Leaf,
    pub post: Leaf,
    pub origo_delta: crate::math::minkowski::OrigoDelta,
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
