
# IO LIBRARY of the QUANTUM GRAVITY ENGINE

TODO - Features:
 - *`qge_follow.rs`*: Creates New and/or Handles Editing of `.fwp`-, `.fwh`-, `.fwb`-, `.fws`-, `.qge`-files.
 - *`hdf5.rs`*: Create Crate-Wrapper → use hdf5 crate for LIGO frames; off by default → *`qge_follow.rs`*::Files/Project/Header/Body/Source.
 - *`fits.rs`*: Create Crate-Wrapper → use fitsio for JWST FITS; off by default → *`qge_follow.rs`*::Files/Project/Header/Body/Source.

TODO:
 - DataSource → Stream<Brick>
	- *`ligo.rs`*: Advanced LIGO (GWOSC frames/HDF5/sky maps) search in `../../data`.
	- *`jwst.rs`*: FITS imagery, catalogs; search in `../../data`.
	- *`adapters.rs`*: ETL to u8 (quantizers, histogrammers).

Contains:
 - *`traits.rs`*: DataSource → Stream<Brick>
 - *`ligo.rs`*: Advanced LIGO (GWOSC frames/HDF5/sky maps) search in `../../data`.
 - *`jwst.rs`*: FITS imagery, catalogs; search in `../../data`.
 - *`adapters.rs`*: ETL to u8 (quantizers, histogrammers).