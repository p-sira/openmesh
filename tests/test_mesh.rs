use openmesh::{Mesh, MeshError, MeshValidationReport};
use std::fs::File;

#[test]
fn test_mesh_validation_perfect() {
    let mut file = File::open("tests/test-data/perfect-suzanne.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert!(mesh.check_mesh(1e-4).is_valid());
    assert_eq!(mesh.validate(), Ok(()));
}

#[test]
fn test_mesh_validation_bad_normal() {
    let mut proper_report = MeshValidationReport::default();
    proper_report.inconsistent_normals = true;

    let mut file = File::open("tests/test-data/monkey-bad-normal.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert_eq!(mesh.check_mesh(1e-4), proper_report);
    assert_eq!(mesh.validate(), Err(MeshError::InconsistentNormals));
}

#[test]
fn test_mesh_validation_intersecting() {
    let mut proper_report = MeshValidationReport::default();
    proper_report.self_intersecting = true;

    let mut file = File::open("tests/test-data/monkey-intersecting.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert_eq!(mesh.check_mesh(1e-4), proper_report);
    assert_eq!(mesh.validate(), Err(MeshError::SelfIntersecting));
}

#[test]
fn test_mesh_validation_open_edges() {
    let mut proper_report = MeshValidationReport::default();
    proper_report.open_edges = 3;

    let mut file = File::open("tests/test-data/monkey-open-edges.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert_eq!(mesh.check_mesh(1e-4), proper_report);
    assert_eq!(mesh.validate(), Err(MeshError::OpenEdges));
}

#[test]
fn test_mesh_validation_non_manifold() {
    let mut proper_report = MeshValidationReport::default();
    proper_report.non_manifold = 2;
    proper_report.open_edges = 6;
    proper_report.inconsistent_normals = true;

    let mut file = File::open("tests/test-data/monkey-non-manifold.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert_eq!(mesh.check_mesh(1e-4), proper_report);
    let err = mesh.validate().unwrap_err();
    assert!(err == MeshError::NonManifold || err == MeshError::OpenEdges);
}

#[test]
fn test_mesh_validation_normal_inside() {
    let mut proper_report = MeshValidationReport::default();
    proper_report.inward_normals = true;

    let mut file = File::open("tests/test-data/monkey-normal-inward.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert_eq!(mesh.check_mesh(1e-4), proper_report);
    assert_eq!(mesh.validate(), Err(MeshError::InwardNormals));
}

#[test]
fn test_mesh_validation_zero_faces() {
    let mut proper_report = MeshValidationReport::default();
    proper_report.zero_area_faces = 20;

    let mut file = File::open("tests/test-data/monkey-zero-faces.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert_eq!(mesh.check_mesh(1e-4), proper_report);
    assert_eq!(mesh.validate(), Err(MeshError::ZeroAreaFace));
}

#[test]
fn test_mesh_validation_bad_suzanne() {
    // Default monkey from Blender
    let proper_report = MeshValidationReport {
        self_intersecting: true,
        open_edges: 42,
        ..Default::default()
    };

    let mut file = File::open("tests/test-data/bad-suzanne.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert!(mesh.validate().is_err());
    assert_eq!(mesh.check_mesh(1e-4), proper_report);
}
