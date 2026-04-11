use alloc::vec::Vec;

use crate::{
    Face, Vertex,
    core::{self, Float, MeshError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Mesh with vertex-face data structure.
pub struct Mesh<T: Float = f32> {
    pub vertices: Vec<Vertex<T>>,
    pub faces: Vec<Face>,
}

impl<T: Float> Mesh<T> {
    #[inline]
    pub fn new<V, F>(vertices: V, faces: F) -> Self
    where
        V: IntoIterator,
        V::Item: Into<Vertex<T>>,
        F: IntoIterator,
        F::Item: Into<Face>,
    {
        Self {
            vertices: vertices.into_iter().map(Into::into).collect(),
            faces: faces.into_iter().map(Into::into).collect(),
        }
    }
}

impl<T: Float> Mesh<T> {
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
