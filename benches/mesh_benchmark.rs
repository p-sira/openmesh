use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use openmesh::Mesh;
use openmesh::core::EdgeMap;
use std::fs::File;
use std::hint::black_box;
use std::io::Cursor;

fn load_mesh_stl(path: &str) -> Mesh<f64> {
    let mut file = File::open(path).expect(&format!("Failed to open {}", path));
    Mesh::from_stl(&mut file).expect(&format!("Failed to parse {}", path))
}

fn load_mesh_obj(path: &str) -> Mesh<f64> {
    let mut file = File::open(path).expect(&format!("Failed to open {}", path));
    Mesh::from_obj(&mut file).expect(&format!("Failed to parse {}", path))
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

fn bench_self_intersection_grids(c: &mut Criterion) {
    let mut group = c.benchmark_group("self_intersection_disjoint_grid");
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(30);

    for face_count in [4, 16, 64, 128, 256, 1024] {
        let mesh = disjoint_triangle_grid(face_count);
        assert_eq!(mesh.check_self_intersecting(), Ok(()));
        group.bench_with_input(BenchmarkId::from_parameter(face_count), &mesh, |b, mesh| {
            b.iter(|| black_box(mesh.check_self_intersecting()));
        });
    }

    group.finish();

    // Exercise the early-exit path as well as the sparse no-collision path.
    let mut group = c.benchmark_group("self_intersection_colliding_grid");
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(30);

    for face_count in [4, 64, 128, 1024] {
        let mut mesh = disjoint_triangle_grid(face_count);
        let first_face = [mesh.vertices[0], mesh.vertices[1], mesh.vertices[2]];
        mesh.vertices[3..6].copy_from_slice(&first_face);
        assert!(mesh.check_self_intersecting().is_err());
        group.bench_with_input(BenchmarkId::from_parameter(face_count), &mesh, |b, mesh| {
            b.iter(|| black_box(mesh.check_self_intersecting()));
        });
    }

    group.finish();
}

fn bench_validation_suites(c: &mut Criterion) {
    // Load all representative meshes
    let good = load_mesh_stl("tests/test-data/perfect-suzanne.stl");
    let bad_all = load_mesh_stl("tests/test-data/bad-suzanne.stl");
    let bad_zero = load_mesh_stl("tests/test-data/monkey-zero-faces.stl");
    let bad_manifold = load_mesh_stl("tests/test-data/monkey-non-manifold.stl");
    let bad_normals = load_mesh_stl("tests/test-data/monkey-bad-normal.stl");
    let bad_orientation = load_mesh_stl("tests/test-data/monkey-normal-inward.stl");
    let bad_intersection = load_mesh_stl("tests/test-data/monkey-intersecting.stl");

    let mut group = c.benchmark_group("mesh_validation");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(200);

    // 1. Full Validation
    for (name, mesh) in [("good", &good), ("bad_combined", &bad_all)] {
        group.bench_with_input(BenchmarkId::new("validate_full", name), mesh, |b, m| {
            b.iter(|| {
                let _ = m.validate();
            })
        });
    }

    // 2. Zero Area Faces
    for (name, mesh) in [("good", &good), ("bad_zero_area", &bad_zero)] {
        group.bench_with_input(
            BenchmarkId::new("check_zero_area_faces", name),
            mesh,
            |b, m| {
                b.iter(|| {
                    let _ = m.check_zero_area_faces(1e-4);
                })
            },
        );
    }

    // 3. Edge Map Construction
    for (name, mesh) in [("good", &good), ("bad_manifold", &bad_manifold)] {
        group.bench_with_input(
            BenchmarkId::new("edge_map_construction", name),
            mesh,
            |b, m| {
                b.iter(|| {
                    let _ = EdgeMap::from_faces(&m.faces);
                })
            },
        );
    }

    // 4. Manifold Check
    for (name, mesh) in [("good", &good), ("bad_non_manifold", &bad_manifold)] {
        group.bench_with_input(BenchmarkId::new("check_manifold", name), mesh, |b, m| {
            b.iter(|| {
                let _ = m.check_manifold();
            })
        });
    }

    // 5. Consistent Normals
    for (name, mesh) in [("good", &good), ("bad_normals", &bad_normals)] {
        group.bench_with_input(
            BenchmarkId::new("check_consistent_normals", name),
            mesh,
            |b, m| {
                b.iter(|| {
                    let _ = m.check_consistent_normals();
                })
            },
        );
    }

    // 6. Orientation
    for (name, mesh) in [("good", &good), ("bad_orientation", &bad_orientation)] {
        group.bench_with_input(BenchmarkId::new("check_orientation", name), mesh, |b, m| {
            b.iter(|| {
                let _ = m.check_orientation();
            })
        });
    }

    // 7. Self-Intersection
    for (name, mesh) in [("good", &good), ("bad_intersection", &bad_intersection)] {
        group.bench_with_input(
            BenchmarkId::new("check_self_intersecting", name),
            mesh,
            |b, m| {
                b.iter(|| {
                    let _ = m.check_self_intersecting();
                })
            },
        );
    }

    group.finish();
}

fn bench_large_mesh(c: &mut Criterion) {
    let mut group = c.benchmark_group("mesh_validation");
    group.measurement_time(std::time::Duration::from_secs(20));
    group.sample_size(20);

    let bad_intersection_large = load_mesh_obj("tests/test-data/skull.obj");
    group.bench_with_input(
        BenchmarkId::new("check_self_intersecting", "large_mesh"),
        &bad_intersection_large,
        |b, m| {
            b.iter(|| {
                let _ = m.check_self_intersecting();
            })
        },
    );

    group.finish();
}

fn bench_mesh_report(c: &mut Criterion) {
    let good = load_mesh_stl("tests/test-data/perfect-suzanne.stl");
    let bad_all = load_mesh_stl("tests/test-data/bad-suzanne.stl");

    let mut group = c.benchmark_group("mesh_report");
    for (name, mesh) in [("good", &good), ("bad_combined", &bad_all)] {
        group.bench_with_input(BenchmarkId::new("check_mesh", name), mesh, |b, m| {
            b.iter(|| black_box(m.check_mesh(1e-4)))
        });
    }
    group.finish();
}

fn bench_io(c: &mut Criterion) {
    let stl_bytes = std::fs::read("tests/test-data/perfect-suzanne.stl")
        .expect("Failed to read tests/test-data/perfect-suzanne.stl");
    let obj_bytes = std::fs::read("tests/test-data/skull.obj")
        .expect("Failed to read tests/test-data/skull.obj");

    let mut group = c.benchmark_group("mesh_io");
    group.bench_function("from_stl", |b| {
        b.iter(|| {
            let mut reader = Cursor::new(black_box(stl_bytes.as_slice()));
            black_box(Mesh::<f64>::from_stl(&mut reader).unwrap())
        })
    });
    group.bench_function("from_obj_large", |b| {
        b.iter(|| {
            let mut reader = Cursor::new(black_box(obj_bytes.as_slice()));
            black_box(Mesh::<f64>::from_obj(&mut reader).unwrap())
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_validation_suites,
    bench_large_mesh,
    bench_mesh_report,
    bench_io,
    bench_self_intersection_grids
);
criterion_main!(benches);
