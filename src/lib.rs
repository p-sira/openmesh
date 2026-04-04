//! # OpenMesh
//!
//! **OpenMesh** is a Rust mesh validation library using face-vertex data structure.
//!
//! ## Quick Start
//!
//! ```
//! use openmesh::{Face, Mesh, MeshError, Vertex};
//!
//! let mesh = Mesh {
//!     vertices: vec![
//!         Vertex(0.0, 0.0, 0.0),
//!         Vertex(1.0, 0.0, 0.0),
//!         Vertex(0.0, 1.0, 0.0),
//!     ],
//!     faces: vec![Face(0, 1, 2)],
//! };
//!
//! assert_eq!(mesh.validate(), Err(MeshError::OpenEdges));
//! ```
//!
//! Loading mesh from file (requires feature flags `stl`, `obj`, etc.):
//!
//! ```bash
//! cargo add openmesh --features stl
//! ```
//! ```rust, no_compile
//! use openmesh::Mesh;
//!
//! let mesh = Mesh::from_stl("mesh.stl").unwrap();
//!
//! assert!(mesh.validate().is_ok());
//! ```
//!
//! OpenMesh also supports parallelization using `rayon`:
//!
//! ```bash
//! cargo add openmesh --features rayon
//! ```

pub mod core;
mod mesh;

pub use crate::mesh::{Face, Mesh, Vertex};
pub use core::{MeshError, MeshValidationReport};

#[cfg(feature = "io")]
mod io;
