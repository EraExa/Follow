
# CORE LIBRARY of the QUANTUM GRAVITY ENGINE

TODO:

 - CORE/BASIC: Implement basic core functionality for these files, with a `*`-Type (Replace `*` with Name of Type).
 - ADDITIONAL: Add `U8_*`-, `U16_*`-, `U32_*`- and `U64_*`-Types to ALL OF THESE CORE LIBRARY FILES (Replace `*` with Name of Type).

Contains:
 - *`basum.rs`*: A Minimal `string` to `isize`-Wrapper Library from `strings` on the form:
	- `"^[+-][1-9]?[0-9]*$"`; Stores Number Bases/Exponents as an `isize`.
 - *`num.rs`*: Implements a `(lower: u8, upper: u8)`-STRICTLY-u8-ONLY-INTERVAL-ARITHMETIC STRUCTURE;
	- Uses `u8ops.rs` and `u8lut.rs`.
 - *`real.rs`*: Implementation of a richer `(Num: NUMBER, Exponent: BASUM, Basis: BASUM)`-Number Structure;
	- Uses `num.rs` and `basum.rs`.
 - *`imag.rs`*: A Branchless Hamilton's-Rules Extended Imaginary Units Library using Column-Arithmetic with Promotion Tracking;
	- Stand-alone library.
 - *`base.rs`*: Implementation of Ringed (Extended) Imaginary Bases `(Values: IMAG[], Ring: usize)`;
	- Uses `imag.rs` for Promotion Tracking of Hamilton's Rules Column-Arithmetic Multiplication.
 - *`interval.rs`*: Implementation of a richer `(lo: REAL, hi: REAL)`-Interval Structure;
	- Uses `real.rs`.
 - *`modulation.rs`*: Implementation of a Complex `(Real: INTERVAL, Base: BASE)`-Interval Arithmetic Structure;
	- Uses `interval.rs` and `base.rs`.
 - *`ore.rs`*: Implementation of `OreOps` - An Ore-Algebraic Skew-Polynomial Field-Extension Library for usage as Operators;
	- Used in `ops.rs`.
 - *`ops.rs`*: Implementation of Strictly Infix-Only Arbitrary Symbolic Operators or Symbolic OreOps from `ore.rs`;
	- Uses `modulation.rs` and `ore.rs`.
 - *`expr.rs`*: A Symbolic Expression-Library (specification only, implementation handled by `tensor.rs`);
	- For usage in `tensor.rs` Tensor Networks.
 - *`tensor.rs`*: A `SYMBOLIC EXPRESSION` with the trait that its EVALUATION results in a MODULATION;
	- Uses `expr.rs` and `modulation.rs`.
 - *`atom.rs`*: The Fully-Fledged Atomic Unit `{ Core: MODULATION[], Interval: (MODULATION, MODULATION)[], Base: TENSOR[], Op: Operator }` for Arbitrary Symbolic Operator (by-default: OreOps);
	- Implementation of the Whole Core Library in one Atomic Unit.
	- Uses `modulation.rs`, `tensor.rs`, `ops.rs`.

TODO - LUTs:
 - *`u8lut_extended.rs`*: tables for sin/cos over 0..255, trig LUTs (RA/Dec → XYZ)
 - *`../util/zorder.rs`*: crate::utils::zorder.rs for morton Z-ordering of (x,y,z) brick coordinates using bit-interleaving via LUTs, Z-order for bricks (u8-safe).