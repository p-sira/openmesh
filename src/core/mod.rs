//! Algorithms and internal data structures.

mod aabb;
mod edge_map;
mod error;
mod math;
mod report;
mod tolerance;
mod validation;

use aabb::AABB;
pub use edge_map::EdgeMap;
pub use error::MeshError;
pub use math::Float;
pub use report::MeshValidationReport;
pub use tolerance::FaceTolerance;
pub(crate) use validation::validate_mesh_input;
pub use validation::{
    check_consistent_normals, check_intersecting, check_inward_orientation, check_manifold,
    check_mesh, check_mesh_with_tolerance, check_zero_area_faces,
    check_zero_area_faces_with_tolerance, validate_mesh, validate_mesh_with_tolerance,
};
