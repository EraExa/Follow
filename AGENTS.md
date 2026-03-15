
# Overall Structure

This document defines the conceptual and programmatic contract for the **Quantum Gravity-Engine** that builds symbolic, interval-based structures over a strictly `u8` arithmetic core.

#### **`Types` used**:
 - **`BASUM`**: *FUZZY-ALGEBRAIC NUMBER BASIS*;
 - **`NUMBER`**: *STRICTLY-U8-ONLY-INTERVAL-ARITHMETIC STRUCTURE*;
 - **`REAL`**: *STRICTLY (NUMBER, BASUM)*;
 - **`IMAGINARY`**: *ENUM ARRAY OF 3-TUPLE IMAGINARY-TRIFECTAS*;
 - **`IMAG`**: *IMAGINARY TRIFECTA*;
 - **`BASE`**: *RINGED IMAG-ARRAY* / *PERMUTOHEDRON*;
 - **`MODULATION`**: *STRICTLY (REAL, BASE)*;
 - **`OP`**: *STRICTLY-INFIX SYMBOLIC OPERATOR (LHS, RHS)*;
 - **`TENSOR`**: *SYMBOLIC EXPRESSION WITH TRAITS THAT ITS EVALUATION RESULTS IN A MODULATION*;
 - **`ATOM`**: *STRICTLY (REAL-ARRAY, PAIR_OF_REALS-ARRAY, TENSOR-ARRAY, OPERATOR) FOR ARBITRARY SYMBOLIC OPERATORS OP*;
 - **`BBST`**: *BALANCED BINARY SEARCH TREE*;
 - **`TST`**: *TERNARY SEARCH TREE*;
 
# Overall Algorithm

## High-level Language `DEFINITIONS` of `Types used`:

 - BASIS
	- Each **`BASUM`** is a `isize` generated from a `"^[+-][1-9]?[0-9]*$"`-Valued STRING denoting NUMBER BASIS.

 - NUMBER / REAL / INTERVAL
	- Each **`NUMBER`** is a `(lower: u8, upper: u8)`-STRICTLY-u8-ONLY-INTERVAL-ARITHMETIC STRUCTURE; with Lookup-Table Operators (LUT-Operators): U8OPERATORS specified (add/sub/mul/div/intersect/hull/...).
	- Each **`REAL`** is a `(Num: NUMBER, Exponent: BASUM, Basis: BASUM)`.
	- Each **`INTERVAL`** is a `(lo: REAL, hi: REAL)`-RICH INTERVAL-ARITHMETIC STRUCTURE.

 - IMAGINARY / IMAG / BASE & MODULATiON
	- Each **`IMAGINARY`** is `\[(i=1, j=2, k=3), (m=4, n=5, o=6), (r=7, s=8, t=9), (e=10, f=11, g=12)\]`;
	- Each **`IMAG`** is a Hamilton's Rules-"Column Arithmetic"-"Extended Imaginary Bases"; IMAGINARY TRIFECTAS (IMAG): `(id: u8, Value: IMAGINARY, Ring: usize)`;
	- Each **`BASE`** is a RINGED IMAG-ARRAY (Ring >= 0) permutation group (PERMUTOHEDRON): `(Values: IMAG-Array, Ring: usize)`.
	- Each **`MODULATION`** is a `(Real: INTERVAL, Base: BASE)`.

 - OP / TENSOR
	- Each **`OP`** is a STRICTLY-INFIX SYMBOLIC OPERATOR containing `(ID: String, LHS: Expr, RHS: Expr)` - Operator semantics map to safe interval combinators;
	- Each **`TENSOR`** is a `SYMBOLIC EXPRESSION` with the trait that its EVALUATION results in a MODULATION.

 - ATOM
	- Each **`ATOM`** is a `{ Core: MODULATION-Array, Interval: (MODULATION, MODULATION)-Array [PAIR], Base: TENSOR-Array, Op: Operator }`, for arbitrary SYMBOLIC operators OP.
 
  - BBST & TST
	- Each **`BBST`** is a kd-mapped Balanced Binary Search Tree of `ATOMs`.
	- Each **`TST`** is a Ternary Search Tree with Relative Minkowski Deltas per Edge, containing three (3x) 3+1-Dimensionally kd-mapped Balanced Binary Search Trees (`BBSTs`) as its LEAVES.

 - NODES / LEAVES
	- Each **`NODE`** is a `Node` in the Ternary Search Tree (`TST`), containing Strictly-ordered PRE-/PERI-/POST-Processing LEAVES.
	- Each **`NODE`** contains an Quaternionic-Minkowski Spacetime-Interval Delta `ORIGO` relative to the Gravity Well's/Parental Node's `ORIGO`.
	- Each **`LEAF`** contains a kd-mapped Balanced Binary Search Tree (`BBST`) of `ATOMS` such that with `(PRE-Node: ATOMIC-CONSTRAINTS / PERI-Node: ATOMIC-VALUES / POST-Node: ATOMIC-FREEDOMS)` the following applies:
		1. **`PRE-Node`**: ATOMIC-CONSTRAINTS; of which is applied to the `NODE's` `ORIGO-Interval` ("relative to the NODE's ORIGO"), resulting in a kd-mapped 3-Dimensional `3D-OBJECT`.
		2. **`PERI-Node`**: ATOMIC-VALUES; of which is applied to the `3D-OBJECT` (relative to the NODE's ORIGO), resulting in a Volumetric kd-mapped 3-Dimensional `VOLUMETRIC-OBJECT`.
		3. **`POST-Node`**: ATOMIC-FREEDOMS; of which is applied to the `VOLUMETRIC-OBJECT` (relative to the NODE's ORIGO), now resulting in a Temporally-Animated 3D Volumetric `ANIMATED-OBJECT`.
 
 - STEP-WISE ITERATION
	- For each step-wise **`ITERATION`** of the `TST`:
		1. Apply the `PRE NODE's` kd-mapped `BBST` to the `Origo` into a new `Volumetric Lorentzian-Frame`.
		2. Apply the `PERI NODE's` kd-mapped `BBST` to the `Volumetric Lorentzian-Frame` as additional Topological/Geometrical Data: `A Volumetric Topological Spacetime Object`.
		3. Apply the `POST NODE's` kd-mapped `BBST` to the `Spacetime Object`, as additional Temporal Structure: `A Temporally-Animated Volumetric Topological Spacetime Object`.

### Nodes, Leaves, and Origo

- **`NODE`**: A TST node containing **strictly ordered** PRE/PERI/POST leaves and a relative **`ORIGO`** (Quaternionic-Minkowski spacetime-interval delta) to its parent.
- **`LEAF`**: Contains a kd-mapped `BBST` of `ATOMs` interpreted as:
- **PRE**: ATOMIC-CONSTRAINTS → applied to `ORIGO-Interval` to yield a *3D-OBJECT*,
- **PERI**: ATOMIC-VALUES → enrich *3D-OBJECT* into *VOLUMETRIC-OBJECT*,
- **POST**: ATOMIC-FREEDOMS → add temporal structure to produce *ANIMATED-OBJECT*.

### Iteration Semantics (per TST step)

1. Apply PRE’s `BBST` to `Origo` ⇒ **Volumetric Lorentzian Frame** (constraints resolved as intervals).
2. Apply PERI’s `BBST` to that frame ⇒ **Volumetric Topological Spacetime Object** (values).
3. Apply POST’s `BBST` ⇒ **Temporally-Animated Volumetric Topological Spacetime Object** (freedoms/temporal DOFs).

## Invariants

- All numeric operations are **u8-only**, performed through **LUT operators**.
- Interval semantics use **wrapping circular** definitions.
- `TENSOR::eval()` returns a `MODULATION` with a **well-defined** `BASE` (permuted by operator) and `REAL` (interval-composed).

## Outputs

Outputs the `ATOM`-Values above, in a **COMPLETELY STAND-ALONE** (containing *all* necessary definitions) `output.qge`-file containing a custom JSON-Object with versioning, the Values specified of which use this pseudo-form:
```pseudo-ebnf
%ignore COMMENTS;
// WHITESPACE ignored
%ignore WHITESPACE;
%start ATOM;
// +3.14159263
ELEM = ([^[0-9]$] | [^[+-][1-9]?[0-9]*$]) ("." (^[0-9]+$))?;
// +0.785_4.0^1.0 = +0.785_+4.0^+1.0
NUM = ELEM "_" ELEM "^" ELEM;
// i (Imaginary Unit), _reservedCandela (?), kN (implementation-dependent)...
ID = ASCII_CHAR+;
UNIT = ID ":" ^[a-zA-Z]+$;
// Reserved UNITs (Imaginary Units & Reserved Physical Units):
IMAG = %UNIT{"i (Imaginary Unit)": "i", ... };
PHYS = %UNIT{"N (Newton)": "N", ... };
// WHITESPACE & Leading COMMA ignored:
ATOM = [ "(" NUM "," "(" NUM "," NUM ")" "," IMAG ")" ","? ]+ PHYS;
```