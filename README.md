# OpenMesh

**OpenMesh** is a Rust mesh validation library using face-vertex data structure.

<p>
    <a href="https://opensource.org/license/BSD-3-clause" style="text-decoration:none">
        <img src="https://img.shields.io/badge/License-BSD--3--Clause-brightgreen.svg" alt="License">
    </a>
    <a href="https://crates.io/crates/openmesh" style="text-decoration:none">
        <img src="https://img.shields.io/crates/v/openmesh" alt="Crate">
    </a>
    <a href="https://crates.io/crates/openmesh" style="text-decoration: none">
        <img src="https://img.shields.io/crates/d/openmesh" alt="Total Downloads">
    </a>
    <a href="https://docs.rs/openmesh" style="text-decoration:none">
        <img src="https://img.shields.io/badge/Docs-docs.rs-blue" alt="Documentation">
    </a>
</p>

## Quick Start

Install **OpenMesh** with

```bash
cargo add openmesh
```

Example:

```rust
use openmesh::{Face, Mesh, MeshError, Vertex};

let mesh = Mesh {
    vertices: vec![
        Vertex(0.0, 0.0, 0.0),
        Vertex(1.0, 0.0, 0.0),
        Vertex(0.0, 1.0, 0.0),
    ],
    faces: vec![Face(0, 1, 2)],
};

assert_eq!(mesh.validate(), Err(MeshError::OpenEdges));
```

Loading mesh from file:

This feature requires enabling the `io` feature flags, such as `stl` and `obj`.

```bash
cargo add openmesh --features stl
```

```rust
use openmesh::Mesh;
use std::fs::File;

let mut file = File::open("mesh.stl").expect("Failed to open mesh.stl");
let mesh: Mesh = Mesh::from_stl(&mut file).expect("Failed to load mesh.stl");

assert!(mesh.validate().is_ok());
```

OpenMesh also supports parallelization using `rayon`:

```bash
cargo add openmesh --features rayon
```
