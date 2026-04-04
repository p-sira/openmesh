use openmesh::Mesh;
use std::fs::File;

#[test]
#[cfg(feature = "stl")]
fn test_stl() {
    let mut file = File::open("tests/io/cube.stl").unwrap();
    let mesh: Mesh = Mesh::from_stl(&mut file).unwrap();
    assert_eq!(mesh.vertices.len(), 8);
}

#[test]
#[cfg(feature = "obj")]
fn test_obj() {
    let mut file = File::open("tests/io/cube.obj").unwrap();
    let mesh: Mesh = Mesh::from_obj(&mut file).unwrap();
    assert_eq!(mesh.vertices.len(), 8);
}
