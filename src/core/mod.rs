//! Algorithms and internal data structures.

mod aabb;
mod edge_map;
mod error;
mod math;
mod report;
mod validation;

use aabb::AABB;
pub use edge_map::EdgeMap;
pub use error::MeshError;
pub use math::{Float, OptionalSend, OptionalSync};
pub use report::MeshValidationReport;
pub use validation::{
    check_consistent_normals, check_intersecting, check_inward_orientation, check_manifold,
    check_mesh, check_zero_area_faces, validate_mesh,
};
