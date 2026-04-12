#[derive(Debug, Clone, PartialEq, Eq)]
/// Error type for mesh validation.
pub enum MeshError {
    OpenEdges,
    NonManifold,
    SelfIntersecting,
    ZeroAreaFace,
    InconsistentNormals,
    InwardNormals,
}

#[cfg(feature = "std")]
impl ToString for MeshError {
    fn to_string(&self) -> String {
        match self {
            MeshError::OpenEdges => "Open edges",
            MeshError::NonManifold => "Non-manifold",
            MeshError::SelfIntersecting => "Self-intersecting",
            MeshError::ZeroAreaFace => "Zero area face",
            MeshError::InconsistentNormals => "Inconsistent normals",
            MeshError::InwardNormals => "Inward normals",
        }.to_string()
    }
}
