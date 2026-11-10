# Changelog

## 0.3.1
### Performance
- Speed up self-intersection checks on large meshes with adaptive AABB pruning
  while preserving early exits on small meshes ([#2](https://github.com/p-sira/openmesh/pull/2)).

## 0.3.0
### Breaking changes
- `Mesh::check_mesh` and `core::check_mesh` return
  `Result<MeshValidationReport, MeshError>`; use `?` or handle the error before
  accessing the report. The core intersection, zero-area, and orientation checks
  return `Result<bool, MeshError>`.
- `EdgeMap::counts` and `EdgeMap::directions` store `usize` instead of `u8`.
- `MeshError` adds `InvalidVertexIndex`, `NonFiniteVertex`, `InvalidTolerance`,
  and `NumericalFailure`; update exhaustive matches. Display messages are
  lowercase with no trailing newline.
- Absolute zero-area tolerance measures triangle area, doubling the previous
  effective threshold for the same `atol`. Comparisons use `<=`, rejecting
  collinear and coincident faces even at zero tolerance.

### Added
- Add `FaceTolerance` and tolerance-specific mesh and core validation functions
  for absolute, relative, or mixed per-face degeneracy checks.
- Implement `core::error::Error` for `MeshError`, including in `no_std` builds.

### Fixed
- Return structured errors for invalid indices, non-finite coordinates, invalid
  tolerances, and numerical failures.
- Prevent edge and direction counts from overflowing at 256 incident faces.
- Correct intersection plane normalization and detect overlap and containment
  between non-adjacent coplanar triangles.
- Scale mixed-tolerance calculations per face to avoid global-scale dependence
  and intermediate area overflow.

## 0.2.3
- Implement `Display` for `MeshError`.

## 0.2.2
- Add `From` and `Into` conversions for `Vertex`.
- Inline more functions to improve performance.

## 0.2.1
- Allow mesh construction from generic iterators.
- Add conversions for standard library containers and `nalgebra` types.

## 0.2.0
- Add `no_std` support.

## 0.1.1
- Add package keywords and categories.

## 0.1.0
- Initial release.
