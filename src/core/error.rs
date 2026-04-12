#[cfg(feature = "std")]
use std::fmt::Display;

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
impl Display for MeshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self {
            MeshError::OpenEdges => "Open edges",
            MeshError::NonManifold => "Non-manifold",
            MeshError::SelfIntersecting => "Self-intersecting",
            MeshError::ZeroAreaFace => "Zero area face",
            MeshError::InconsistentNormals => "Inconsistent normals",
            MeshError::InwardNormals => "Inward normals",
        };
        writeln!(f, "{}", err_str)
    }
}
