# 0.3

## 0.3.0

### Breaking changes

- `Mesh::check_mesh` and `core::check_mesh` now return
  `Result<MeshValidationReport, MeshError>`. Existing callers must handle or
  unwrap the result before accessing the report.
- `core::check_intersecting`, `core::check_zero_area_faces`, and
  `core::check_inward_orientation` now return `Result<bool, MeshError>` instead
  of `bool`.
- `EdgeMap::counts` and `EdgeMap::directions` now store `usize` values instead
  of `u8`. Code naming either concrete map type must be updated.
- `MeshError` adds `InvalidVertexIndex`, `NonFiniteVertex`,
  `InvalidTolerance`, and `NumericalFailure`. Exhaustive matches must handle
  these variants.
- Absolute zero-area tolerance now means triangle area. The previous
  implementation compared the parallelogram area against `atol`, making its
  effective triangle-area threshold `atol / 2`. A supplied `atol` can
  therefore classify more faces as degenerate in 0.3.0.
- Degeneracy comparisons now use `<=`. Exactly collinear or coincident faces
  are rejected when the tolerance is zero.
- `MeshError` display messages are lowercase and no longer end with a newline.

### Added

- Add `FaceTolerance` for mixed per-face degeneracy checks:
  `cross_norm <= absolute_double_area + relative * max_edge_length²`.
- Add `check_mesh_with_tolerance`, `validate_with_tolerance`, and
  `check_zero_area_faces_with_tolerance` methods and matching core functions.
- Return structured validation errors for out-of-range face indices,
  non-finite coordinates, invalid tolerances, and numerical failures.
- Implement `core::error::Error` for `MeshError`, including in `no_std` builds.

### Fixed

- Prevent edge and direction counts from overflowing at 256 incident faces.
- Normalize intersection plane normals correctly so intersection results do
  not depend on triangle area.
- Detect overlap and containment between non-adjacent coplanar triangles.
- Use scaled per-face calculations for mixed tolerances, avoiding dependence
  on a global mesh scale and raw-area overflow in ordinary finite inputs.

### Migration

Handle the new result from report generation:

```rust
let report = mesh.check_mesh(1e-4)?;
if report.is_valid() {
    // ...
}
```

For relative or mixed degeneracy checks, construct a validated tolerance and
use a tolerance-specific method:

```rust
use openmesh::FaceTolerance;

let tolerance = FaceTolerance::new(0.0, 1e-6)?;
let report = mesh.check_mesh_with_tolerance(tolerance)?;
```

# 0.2

## 0.2.3

- Implement `Display` for `MeshError`

## 0.2.2

- Implement more `From` and `Into` for `Vertex` struct
- Inline more functions to improve performance

## 0.2.1

- Allow mesh to be built from generic iterators
- Add type conversion for std containers
- Support nalgebra type conversion

## 0.2.0

- Add no_std support

# 0.1

## 0.1.1

- Add keywords and categories metadata

## 0.1.0

- Initial release
