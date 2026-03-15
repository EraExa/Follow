
pub struct VoxelGrid {
    pub brick_dim: u8, // e.g., 32
    // storage: on-demand via octree; here we keep it abstract
}

impl VoxelGrid {
    pub fn write_voxel(&mut self, bx: u8, by: u8, bz: u8, bt: u8,
                       vx: u8, vy: u8, vz: u8, val: Interval) {
        // find & write into brick (bx,by,bz,bt), voxel local (vx,vy,vz)
        // combine via interval policy (e.g., hull)
        todo!()
    }
}
