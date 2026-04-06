//! # OpenMesh
//!
//! **OpenMesh** is a Rust mesh validation library using face-vertex data structure.
//!
//! ## Quick Start
//!
//! ```
//! use openmesh::{Face, Mesh, MeshError, Vertex};
//!
//! let mesh = Mesh::new(
//!     vec![
//!         Vertex(0.0, 0.0, 0.0),
//!         Vertex(1.0, 0.0, 0.0),
//!         Vertex(0.0, 1.0, 0.0),
//!     ],
//!     vec![Face(0, 1, 2)],
//! );
//!
//! assert_eq!(mesh.validate(), Err(MeshError::OpenEdges));
//! ```
//!
//! Loading mesh from file (requires feature flags `stl`, `obj`, etc.):
//!
//! ```bash
//! cargo add openmesh --features stl
//! ```
//!
//! ```ignore
//! use openmesh::Mesh;
//! use std::fs::File;
//!
//! let mut file = File::open("mesh.stl").expect("Failed to open mesh.stl");
//! let mesh: Mesh = Mesh::from_stl(&mut file).expect("Failed to load mesh.stl");
//!
//! assert!(mesh.validate().is_ok());
//! ```
//!
//! OpenMesh also supports parallelization using `rayon`:
//!
//! ```bash
//! cargo add openmesh --features rayon
//! ```
//!
//! OpenMesh can be used in `no_std` environment:
//!
//! ```bash
//! cargo add openmesh --no-default-features --features libm
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

pub mod core;
mod face;
mod mesh;
mod vertex;

pub use core::{MeshError, MeshValidationReport};
pub use face::{Face, FaceView};
pub use mesh::Mesh;
pub use vertex::{Vertex, VertexView};

#[cfg(feature = "io")]
mod io;
