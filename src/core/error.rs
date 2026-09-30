#[derive(Debug, Clone, PartialEq, Eq)]
/// Error type for mesh validation.
pub enum MeshError {
    InvalidVertexIndex {
        face_index: usize,
        vertex_index: usize,
    },
    NonFiniteVertex {
        vertex_index: usize,
    },
    InvalidTolerance,
    NumericalFailure {
        face_index: usize,
    },
    OpenEdges,
    NonManifold,
    SelfIntersecting,
    ZeroAreaFace,
    InconsistentNormals,
    InwardNormals,
}

impl core::fmt::Display for MeshError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MeshError::InvalidVertexIndex {
                face_index,
                vertex_index,
            } => write!(
                f,
                "face {face_index} references missing vertex {vertex_index}"
            ),
            MeshError::NonFiniteVertex { vertex_index } => {
                write!(f, "vertex {vertex_index} contains a non-finite coordinate")
            }
            MeshError::InvalidTolerance => write!(f, "tolerance must be finite and non-negative"),
            MeshError::NumericalFailure { face_index } => {
                write!(f, "numerical failure while processing face {face_index}")
            }
            MeshError::OpenEdges => write!(f, "open edges"),
            MeshError::NonManifold => write!(f, "non-manifold"),
            MeshError::SelfIntersecting => write!(f, "self-intersecting"),
            MeshError::ZeroAreaFace => write!(f, "zero area face"),
            MeshError::InconsistentNormals => write!(f, "inconsistent normals"),
            MeshError::InwardNormals => write!(f, "inward normals"),
        }
    }
}

impl core::error::Error for MeshError {}
