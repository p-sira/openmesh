use openmesh::{Face, FaceTolerance, Mesh, MeshError, Vertex, core::EdgeMap};

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

#[test]
fn edge_counts_do_not_wrap_at_u8_boundary() {
    for face_count in [256, 257, 258] {
        let faces = vec![Face(0, 1, 2); face_count];
        let map = EdgeMap::from_faces(&faces);

        assert_eq!(map.counts[&(0, 1)], face_count);
        assert_eq!(map.directions[&(0, 1)], face_count);
        assert_eq!(
            openmesh::core::check_manifold(&map),
            Err(MeshError::NonManifold)
        );
    }
}

#[test]
fn area_tolerance_uses_triangle_area() {
    let mesh = Mesh::<f64>::new(
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.00015, 0.0]],
        [[0, 1, 2]],
    );

    assert_eq!(
        mesh.check_zero_area_faces(0.0001),
        Err(MeshError::ZeroAreaFace)
    );
}

#[test]
fn zero_tolerance_rejects_exact_degeneracy() {
    let mesh = Mesh::<f64>::new(
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        [[0, 1, 2]],
    );

    assert_eq!(
        mesh.check_zero_area_faces(0.0),
        Err(MeshError::ZeroAreaFace)
    );
}

#[test]
fn relative_tolerance_is_per_face_and_scale_invariant() {
    let tolerance = FaceTolerance::new(0.0, 0.01).unwrap();

    for scale in [1e-6, 1.0, 1e6] {
        let mesh = Mesh::<f64>::new(
            [[0.0, 0.0, 0.0], [scale, 0.0, 0.0], [0.0, scale, 0.0]],
            [[0, 1, 2]],
        );
        assert_eq!(mesh.check_zero_area_faces_with_tolerance(tolerance), Ok(()));
    }
}

#[test]
fn relative_tolerance_includes_equality() {
    let mesh = Mesh::<f64>::new(
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        [[0, 1, 2]],
    );
    let tolerance = FaceTolerance::new(0.0, 0.5).unwrap();

    assert_eq!(
        mesh.check_zero_area_faces_with_tolerance(tolerance),
        Err(MeshError::ZeroAreaFace)
    );
}

#[test]
fn intersection_is_preserved_when_geometry_is_scaled_up() {
    for scale in [1.0, 1e8] {
        let vertices = [
            [-2.0, -2.0, 0.0],
            [2.0, -2.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.2, 0.0, -1.0],
            [0.2, 0.0, 1.0],
            [0.5, 1.0, 1.0],
        ]
        .map(|vertex| vertex.map(|coordinate| coordinate * scale));
        let mesh = Mesh::<f64>::new(vertices, [[0, 1, 2], [3, 4, 5]]);

        assert_eq!(
            mesh.check_self_intersecting(),
            Err(MeshError::SelfIntersecting)
        );
    }
}

#[test]
fn coplanar_containment_is_an_intersection() {
    let mesh = Mesh::<f64>::new(
        [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.2, 0.2, 0.0],
            [0.8, 0.2, 0.0],
            [0.2, 0.8, 0.0],
        ],
        [[0, 1, 2], [3, 4, 5]],
    );

    assert_eq!(
        mesh.check_self_intersecting(),
        Err(MeshError::SelfIntersecting)
    );
}

#[test]
fn disjoint_coplanar_triangles_do_not_intersect() {
    let mesh = Mesh::<f64>::new(
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [2.0, 2.0, 0.0],
            [3.0, 2.0, 0.0],
            [2.0, 3.0, 0.0],
        ],
        [[0, 1, 2], [3, 4, 5]],
    );

    assert_eq!(mesh.check_self_intersecting(), Ok(()));
}

fn disjoint_triangle_grid(face_count: usize) -> Mesh<f64> {
    let width = ((face_count as f64).sqrt().ceil() as usize).max(1);
    let mut vertices = Vec::with_capacity(3 * face_count);
    let mut faces = Vec::with_capacity(face_count);

    for index in 0..face_count {
        let x = 2.0 * (index % width) as f64;
        let y = 2.0 * (index / width) as f64;
        let base = vertices.len();
        vertices.extend([[x, y, 0.0], [x + 0.75, y, 0.0], [x, y + 0.75, 0.0]]);
        faces.push([base, base + 1, base + 2]);
    }

    Mesh::new(vertices, faces)
}

fn append_triangle(mesh: &mut Mesh<f64>, vertices: [[f64; 3]; 3]) {
    let base = mesh.vertices.len();
    mesh.vertices.extend(vertices.map(Vertex::from));
    mesh.faces.push(Face(base, base + 1, base + 2));
}

#[test]
fn sparse_triangle_grids_have_no_self_intersections() {
    for face_count in [0, 1, 2, 4, 16, 64, 127, 128, 129, 256, 1023, 1024, 1025] {
        let mesh = disjoint_triangle_grid(face_count);
        assert_eq!(
            mesh.check_self_intersecting(),
            Ok(()),
            "face_count={face_count}"
        );
    }
}

#[test]
fn pruning_finds_non_adjacent_crossings_regardless_of_face_order() {
    for face_count in [4, 16, 64, 127, 128, 129, 256, 1023, 1024] {
        let mut mesh = disjoint_triangle_grid(face_count);
        // The new facet cuts through the interior of grid face 0 without
        // sharing any vertex indices with it.
        append_triangle(
            &mut mesh,
            [[0.25, 0.25, -1.0], [0.25, 0.25, 1.0], [0.6, 0.25, 1.0]],
        );
        let crossing = mesh.faces.pop().unwrap();

        for position in [0, face_count / 2, face_count] {
            let mut reordered = mesh.clone();
            reordered.faces.insert(position, crossing.clone());
            assert_eq!(
                reordered.check_self_intersecting(),
                Err(MeshError::SelfIntersecting),
                "face_count={face_count}, crossing at {position}"
            );
            reordered.faces.reverse();
            assert_eq!(
                reordered.check_self_intersecting(),
                Err(MeshError::SelfIntersecting),
                "face_count={face_count}, reverse of {position}"
            );
        }
    }
}

#[test]
fn identical_face_bounds_do_not_hide_coplanar_intersections() {
    for face_count in [0, 4, 64, 127, 128, 129, 256, 1023, 1024] {
        let mut mesh = disjoint_triangle_grid(face_count);
        // Both facets span x=-4..-2 and y=-4..-2, with overlapping
        // interiors, but use distinct vertex indices.
        append_triangle(
            &mut mesh,
            [[-4.0, -4.0, 0.0], [-2.0, -4.0, 0.0], [-4.0, -2.0, 0.0]],
        );
        append_triangle(
            &mut mesh,
            [[-2.0, -2.0, 0.0], [-4.0, -4.0, 0.0], [-2.0, -4.0, 0.0]],
        );

        let last = mesh.faces.pop().unwrap();
        for position in [0, face_count / 2, face_count + 1] {
            let mut reordered = mesh.clone();
            reordered.faces.insert(position, last.clone());
            assert_eq!(
                reordered.check_self_intersecting(),
                Err(MeshError::SelfIntersecting),
                "face_count={face_count}, insertion={position}"
            );
            reordered.faces.reverse();
            assert_eq!(
                reordered.check_self_intersecting(),
                Err(MeshError::SelfIntersecting),
                "face_count={face_count}, reversed insertion={position}"
            );
        }
    }
}

#[test]
fn overlapping_face_bounds_can_be_geometrically_disjoint() {
    let mut mesh = disjoint_triangle_grid(64);
    // These two coplanar right triangles have overlapping AABBs, but their
    // interiors and boundaries are separated by more than half a unit.
    append_triangle(
        &mut mesh,
        [[-4.0, -4.0, 0.0], [-3.0, -4.0, 0.0], [-4.0, -3.0, 0.0]],
    );
    append_triangle(
        &mut mesh,
        [[-3.1, -3.1, 0.0], [-2.1, -3.1, 0.0], [-3.1, -2.1, 0.0]],
    );

    assert_eq!(mesh.check_self_intersecting(), Ok(()));
    mesh.faces.reverse();
    assert_eq!(mesh.check_self_intersecting(), Ok(()));
}
