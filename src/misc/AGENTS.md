
# MISC LIBRARY of the QUANTUM GRAVITY ENGINE

TODO - Genetic Engineering & Reservoir Computing Library: *`/gen/`*, *`/reservoir/`* which uses the *`../io/memory.rs`*-Memory Offloading Library.

Contains:
 - *`redblack.rs`*: Contains Balancing-structure for BBSTs;
	- Used in `bbst.rs`.
 - *`bbst.rs`*: Contains Balanced Binary Search-Tree implementing Red-Black Balancing of Atomic Units;
	- Uses `redblack.rs`, `../core/atom.rs`.
	- Used in `kd.rs` as kd-mapped Red-Black Balanced Binary Search-Trees;
 - *`kd.rs`*: 3+1 kd-mapped BBSTs implementing Red-Black;
	- Uses `bbst.rs`.
	- Used in `oxygen.rs`.
 - *`tst.rs`*: Contains Ternary Search-Tree implementation with Relative Minkowski Deltas of Atomic Units per Edge, containing three (3x) 3+1-Dimensionally kd-mapped Balanced Binary Search Trees (`BBSTs`) as its LEAVES;
	- Uses `bbst.rs`, `kd.rs`, `../core/atom.rs`, `../math/minkowski.rs`.
 - *`ecc.rs`*: Contains a basic Error-Correcting Code-Structure (e.g Linear Block Codes);
	- Used in `../quantum/quantum_events.rs` for ECC-Based Quantum Events.
 - *`omega.rs`*: Contains an Omega Number-Approximation Engine.
	- Used in `../quantum/quantum_events.rs` for whenever the approximation is fulfilled as a Minkowski Spacetime-Position.
