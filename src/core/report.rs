/// Report of mesh validation.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshValidationReport {
    pub self_intersecting: bool,
    pub inward_normals: bool,
    pub inconsistent_normals: bool,
    pub non_manifold: usize,
    pub open_edges: usize,
    pub zero_area_faces: usize,
}

impl MeshValidationReport {
    /// Check if the mesh is valid: has no self-intersecting,
    /// no inward-normal, no inconsistent-normal,
    /// no non-manifold, no open-edge, and no zero-area-face.
    #[inline]
    pub fn is_valid(&self) -> bool {
        !(self.self_intersecting
            || self.inward_normals
            || self.inconsistent_normals
            || self.non_manifold > 0
            || self.open_edges > 0
            || self.zero_area_faces > 0)
    }
}
