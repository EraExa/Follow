
# The “Timeless” Lorentzian Frame‑Substitute via PID time geometry

We replace “coordinate time” with a control geometry Tempus ∈ HTC (Horn‑Torus‑Cone) and define:

	T_n : = Update_n/Error_n;

with strictly u8 interval functionals:

	Error_n​ (u8): geometric mismatch + info noise (interval widths, ECC failure rates).
	Update_n​ (u8): successful holonomy correction steps + Fisher‑like information gain (u8 LUT).

Timelessness: We do not assume a global time parameter. Instead, render order is governed by the control scalar T_n​ and a partial order from ECC acceptance.

The “Lorentzian frame” is replaced by the fiber state Tempus_n and update metric T_n​.
In flat regimes, T_n​ acts like uniform ticks (SR limit); in curved/noisy regimes, T_n dilates/contracts (GR‑like), matching our Tempus Fibrational HTC-Geometry intuition.

# Algorithm

1. Ω approximation → symbolization (u8)
	Maintain Ω_G(n) as u8 fixed-point intervals: each update yields a delta interval delta.
	Quantize via LUT: delta -> symbol(s) (e.g., 2 or 3 u8 symbols).

2. ECC validation
	Encode delta symbols into a block code with generator G (pre-baked u8 LUT).
	Simulate/read measurement symbols ~c; decode via syndrome LUT S; produce ECCWord{sym: corrected, ok}.
	If ok == 1, accept to QEvent; else discard (or route to noise accounting [TODO]).

3. Control geometry (PID-like) in u8 intervals
	Define at step n:
	- Error_n​:
		- ecc_err: fraction of undecodable words in a window (u8 via LUT),
		- geo_err: interval mismatch vs. local HTC fiber (e.g., width penalties),
		- Combined with saturating add: err = sat_add(ecc_err, geo_err).
	
	- Update_n​:
		- info_update: LUT of “Fisher‑like” gain from symbol distinctness,
		- holo_update: LUT from successful loop closures (if any structural constraints exist),
		- upd = sat_add(info_update, holo_update).
		
	Control scalar: T = update / error via a division LUT (u8 table for a/b with clamping; 0‑safe).

	Store T in QEvent.

4. Spatial mapping (u8 only)

	Map the codeword + delta to (x,y,z) intervals using precomputed, deterministic LUTs (this is our computable fractal kernel).

		xyz <- φ_k(xyz; sym_k) where φ_k is a small u8 affine map (scale/translate) guaranteed contractive in interval sense.

	Write into VoxelGrid: fuse via i_hull (accumulate) or i_intersect (agreement).

5. “Timeless” rendering order

	Maintain a priority queue keyed by descending T (higher control coherence first).

	For each dequeued QEvent:
	- Apply PRE: constrain region (BBST on constraints).
	- Apply PERI: write intensities/labels (values).
	- Apply POST: assign animation interval (time gate) from the control scalar T and Ω prefix n (u8 LUT from (T, n mod 256)).

	This creates a Lorentzian‑like foliation without a global time variable: frames are control‑consistent slices.
