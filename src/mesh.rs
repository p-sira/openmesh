use alloc::vec::Vec;

use ::core::marker::PhantomData;

use crate::{
    Face, FaceView, Vertex, VertexView,
    core::{self, Float, MeshError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Mesh with vertex-face data structure.
pub struct Mesh<T, V = Vertex<T>, F = Face>
where
    T: Float,
    V: VertexView<T>,
    F: FaceView,
{
    pub vertices: Vec<V>,
    pub faces: Vec<F>,
    _phantom: PhantomData<T>,
}

impl<T, V, F> Mesh<T, V, F>
where
    T: Float,
    V: VertexView<T>,
    F: FaceView,
{
    pub fn new<VI, FI>(vertices: VI, faces: FI) -> Self
    where
        VI: IntoIterator<Item = V>,
        FI: IntoIterator<Item = F>,
    {
        Self {
            vertices: vertices.into_iter().collect(),
            faces: faces.into_iter().collect(),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn to_inner(self) -> (Vec<V>, Vec<F>) {
        (self.vertices, self.faces)
    }

    #[inline]
    pub fn validate(&self) -> Result<(), MeshError> {
        core::validate_mesh(&self.vertices, &self.faces, T::from(1e-4).unwrap())
    }

    #[inline]
    pub fn check_mesh(&self, atol: T) -> core::MeshValidationReport {
        core::check_mesh(&self.vertices, &self.faces, atol)
    }

    #[inline]
    pub fn validate_with_atol(&self, atol: T) -> Result<(), MeshError> {
        core::validate_mesh(&self.vertices, &self.faces, atol)
    }

    #[inline]
    pub fn check_zero_area_faces(&self, atol: T) -> Result<(), MeshError> {
        if core::check_zero_area_faces(&self.vertices, &self.faces, atol) {
            Err(MeshError::ZeroAreaFace)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn check_manifold(&self) -> Result<(), MeshError> {
        let edge_map = core::EdgeMap::from_faces(&self.faces);
        core::check_manifold(&edge_map)
    }

    #[inline]
    pub fn check_consistent_normals(&self) -> Result<(), MeshError> {
        let edge_map = core::EdgeMap::from_faces(&self.faces);
        if core::check_consistent_normals(&edge_map) {
            Ok(())
        } else {
            Err(MeshError::InconsistentNormals)
        }
    }

    #[inline]
    pub fn check_orientation(&self) -> Result<(), MeshError> {
        if core::check_inward_orientation(&self.vertices, &self.faces) {
            Err(MeshError::InwardNormals)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn check_self_intersecting(&self) -> Result<(), MeshError> {
        if core::check_intersecting(&self.vertices, &self.faces) {
            Err(MeshError::SelfIntersecting)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_view_types() {
        // Test with [f32; 3] and [usize; 3]
        let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let faces = vec![[0, 1, 2]];

        let mesh: Mesh<f32, [f32; 3], [usize; 3]> = Mesh::new(vertices, faces);
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);

        // Should be able to call validation methods
        // (Note: this mesh is invalid because it's open, but we just want to see if it calls)
        assert!(mesh.validate().is_err());
    }

    #[test]
    fn test_default_types() {
        let vertices = vec![Vertex(0.0, 0.0, 0.0), Vertex(1.0, 0.0, 0.0), Vertex(0.0, 1.0, 0.0)];
        let faces = vec![Face(0, 1, 2)];

        let mesh = Mesh::new(vertices, faces);
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }
}
