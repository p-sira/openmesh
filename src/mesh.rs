use alloc::vec::Vec;

use crate::core::{self, Float, MeshError};

// MARK: Face

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face(pub usize, pub usize, pub usize);

// MARK: Vertex

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vertex<T: Float>(pub T, pub T, pub T);

impl<T: Float> Vertex<T> {
    pub fn sub(&self, other: &Self) -> Self {
        Vertex(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vertex(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn dot(&self, other: &Self) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

// MARK: Mesh

#[derive(Debug, Clone, PartialEq, Eq)]
/// Mesh with vertex-face data structure.
pub struct Mesh<T: Float = f32> {
    pub vertices: Vec<Vertex<T>>,
    pub faces: Vec<Face>,
}

impl<T: Float> Mesh<T> {
    pub fn new(vertices: Vec<Vertex<T>>, faces: Vec<Face>) -> Self {
        Self { vertices, faces }
    }
}

impl<T: Float> Mesh<T> {
    pub fn validate(&self) -> Result<(), MeshError> {
        core::validate_mesh(&self.vertices, &self.faces, T::from(1e-4).unwrap())
    }

    pub fn check_mesh(&self, atol: T) -> core::MeshValidationReport {
        core::check_mesh(&self.vertices, &self.faces, atol)
    }

    pub fn validate_with_atol(&self, atol: T) -> Result<(), MeshError> {
        core::validate_mesh(&self.vertices, &self.faces, atol)
    }

    pub fn check_zero_area_faces(&self, atol: T) -> Result<(), MeshError> {
        if core::check_zero_area_faces(&self.vertices, &self.faces, atol) {
            Err(MeshError::ZeroAreaFace)
        } else {
            Ok(())
        }
    }

    pub fn check_manifold(&self) -> Result<(), MeshError> {
        let edge_map = core::EdgeMap::from_faces(&self.faces);
        core::check_manifold(&edge_map)
    }

    pub fn check_consistent_normals(&self) -> Result<(), MeshError> {
        let edge_map = core::EdgeMap::from_faces(&self.faces);
        if core::check_consistent_normals(&edge_map) {
            Ok(())
        } else {
            Err(MeshError::InconsistentNormals)
        }
    }

    pub fn check_orientation(&self) -> Result<(), MeshError> {
        if core::check_inward_orientation(&self.vertices, &self.faces) {
            Err(MeshError::InwardNormals)
        } else {
            Ok(())
        }
    }

    pub fn check_self_intersecting(&self) -> Result<(), MeshError> {
        if core::check_intersecting(&self.vertices, &self.faces) {
            Err(MeshError::SelfIntersecting)
        } else {
            Ok(())
        }
    }
}
