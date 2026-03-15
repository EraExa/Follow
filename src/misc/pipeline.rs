
use crate::math::minkowski::OrigoDelta;
use crate::misc::bbst::BBST;
use crate::misc::tst::TST;

pub struct Builder {
    pre: BBST,
    peri: BBST,
    post: BBST,
    origo: OrigoDelta,
}

impl Builder {
    pub fn new(origo: OrigoDelta) -> Self {
        Self { pre: BBST::default(), peri: BBST::default(), post: BBST::default(), origo }
    }
    pub fn push_pre(&mut self, key: KdKey, atom: Atom) { self.pre.insert(key, atom); }
    pub fn push_peri(&mut self, key: KdKey, atom: Atom) { self.peri.insert(key, atom); }
    pub fn push_post(&mut self, key: KdKey, atom: Atom) { self.post.insert(key, atom); }

    pub fn build_node(self) -> Node {
        Node {
            pre: Leaf { tree: self.pre },
            peri: Leaf { tree: self.peri },
            post: Leaf { tree: self.post },
            origo_delta: self.origo,
        }
    }
}

impl TST {
    pub fn single_node(origo: OrigoDelta, pre: BBST, peri: BBST, post: BBST) -> Self {
        Self { nodes: vec![ Node {
            pre: Leaf { tree: pre },
            peri: Leaf { tree: peri },
            post: Leaf { tree: post },
            origo_delta: origo,
        }]}
    }
}
