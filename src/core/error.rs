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
