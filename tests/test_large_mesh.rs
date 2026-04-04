use openmesh::{Mesh, MeshError};
use std::fs::File;

#[test]
#[ignore]
fn test_large_mesh_skull() {
    let mut file = File::open("tests/test-data/skull.obj").expect("Failed to open skull.obj");
    let mesh: Mesh = Mesh::from_obj(&mut file).expect("Failed to load skull.obj");

    println!(
        "Loaded skull mesh with {} vertices and {} faces",
        mesh.vertices.len(),
        mesh.faces.len()
    );

    let result = mesh.validate();
    assert_eq!(result, Err(MeshError::SelfIntersecting));
}
