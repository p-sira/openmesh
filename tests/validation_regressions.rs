use openmesh::{Mesh, MeshError, Vertex};

#[test]
fn invalid_face_index_returns_an_error() {
    let mesh = Mesh::<f64>::new([[0.0, 0.0, 0.0]], [[0, 1, 2]]);

    assert_eq!(
        mesh.validate(),
        Err(MeshError::InvalidVertexIndex {
            face_index: 0,
            vertex_index: 1,
        })
    );
    assert_eq!(
        mesh.check_mesh(0.0),
        Err(MeshError::InvalidVertexIndex {
            face_index: 0,
            vertex_index: 1,
        })
    );
}

#[test]
fn non_finite_unused_vertex_returns_an_error() {
    let mesh = Mesh::<f64> {
        vertices: vec![Vertex(f64::NAN, 0.0, 0.0)],
        faces: vec![],
    };

    assert_eq!(
        mesh.validate(),
        Err(MeshError::NonFiniteVertex { vertex_index: 0 })
    );
    assert_eq!(
        mesh.check_mesh(0.0),
        Err(MeshError::NonFiniteVertex { vertex_index: 0 })
    );
}

#[test]
fn invalid_tolerance_returns_an_error() {
    let mesh = Mesh::<f64> {
        vertices: vec![],
        faces: vec![],
    };

    assert_eq!(mesh.check_mesh(-1.0), Err(MeshError::InvalidTolerance));
    assert_eq!(
        mesh.validate_with_atol(f64::NAN),
        Err(MeshError::InvalidTolerance)
    );
}

#[test]
fn error_display_has_no_trailing_newline() {
    assert_eq!(MeshError::OpenEdges.to_string(), "open edges");
}

fn assert_error<E: core::error::Error>() {}

#[test]
fn mesh_error_implements_core_error() {
    assert_error::<MeshError>();
}
