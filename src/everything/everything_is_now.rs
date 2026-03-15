
use crate::quantum::quantum_events;
use crate::misc::omega;
use crate::misc::ecc;

// Ω-step -> event -> render
fn process_step(mut state: OmegaState, grids: &mut VoxelGrid, TST: &mut TSTGraph) {
    // 1) Ω prefix update
    let delta: Interval = state.next_delta_u8();		// monotone lower semicomputable in u8 intervals

    // 2) ECC encode & read
    let sym: Vec<u8> = symbol_lut(delta);				// u8 quantization
    let code = ecc_encode(sym.clone());					// LUT
    let meas = noisy_channel(code.clone());				// simulated/actual
    let (ok, corrected) = ecc_decode(meas);				// syndrome LUT

    if ok == 0 { state.account_noise_u8(); return; }	// TODO: state.account_soice_u8()-Implementation

    // 3) Control scalars
    let ecc_err = ecc_error_lut(state.recent_fail_rate_u8());
    let geo_err = geo_error_lut(state.local_fiber_mismatch_u8());
    let err    = sat_add(ecc_err, geo_err);

    let info_upd = info_update_lut(&corrected);
    let hol_upd  = holonomy_update_lut(state.holonomy_context_u8());
    let upd      = sat_add(info_upd, hol_upd);

    let T        = div_u8(upd, err);					// via LUT (0-safe)

    // 4) Spatial mapping (IFS-like)
    let (x,y,z)  = spatial_map_lut(&corrected, state.embedding_state_u8());
    let qevt = QEvent {
        omega: OmegaStep { n: state.n, delta },
        code: ECCWord { sym: corrected, ok: 1 },
        T,
        xyz: (x, y, z),
    };

    // 5) TST PRE/PERI/POST (TST layer)
    let node = TST.select_node_lut(&qevt);
    TST.apply_pre(node, &qevt);							// constraints -> Object3D
    TST.apply_peri(node, &qevt);						// values     -> Volume3D
    TST.apply_post(node, &qevt);						// freedoms   -> Animated

    // 6) Voxel write (u8 intervals)
    grids.write(qevt.xyz, qevt.T, fusion_policy_lut());
}
