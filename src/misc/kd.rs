
/// kd-key in 3+1 (u8)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct KdKey { pub x: u8, pub y: u8, pub z: u8, pub t: u8 }

#[derive(Clone, Debug)]
pub struct AtomNode {
    pub key: KdKey,
    pub atom: Atom,
}
