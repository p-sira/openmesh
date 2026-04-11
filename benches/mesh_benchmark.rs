use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use openmesh::Mesh;
use openmesh::core::EdgeMap;
use std::fs::File;

fn load_mesh_stl(path: &str) -> Mesh<f64> {
    let mut file = File::open(path).expect(&format!("Failed to open {}", path));
    Mesh::from_stl(&mut file).expect(&format!("Failed to parse {}", path))
}

fn load_mesh_obj(path: &str) -> Mesh<f64> {
    let mut file = File::open(path).expect(&format!("Failed to open {}", path));
    Mesh::from_obj(&mut file).expect(&format!("Failed to parse {}", path))
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

criterion_group!(benches, bench_validation_suites, bench_large_mesh);
criterion_main!(benches);
