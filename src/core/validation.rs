use alloc::vec::Vec;

use crate::{
    Face, Vertex,
    core::{AABB, EdgeMap, Float, MeshError, MeshValidationReport},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Check if the mesh has intersecting faces.
pub fn check_intersecting<T: Float>(vertices: &[Vertex<T>], faces: &[Face]) -> bool {
    let compute_aabb_and_normal = |f: &Face| {
        let v0 = &vertices[f.0];
        let v1 = &vertices[f.1];
        let v2 = &vertices[f.2];

        let aabb = AABB::from_triangle(v0, v1, v2);

        let n = (v2.sub(v0)).cross(&v1.sub(v0));
        let n_sq = n.0 * n.0 + n.1 * n.1 + n.2 * n.2;

        let normal = if n_sq < T::from(1e-12).unwrap() {
            None
        } else {
            let inv_n_sq = T::one() / n_sq;
            Some(Vertex(n.0 * inv_n_sq, n.1 * inv_n_sq, n.2 * inv_n_sq))
        };

        (aabb, normal)
    };

    #[cfg(feature = "rayon")]
    let (aabbs, normals): (Vec<_>, Vec<_>) = faces.par_iter().map(compute_aabb_and_normal).unzip();

    #[cfg(not(feature = "rayon"))]
    let (aabbs, normals): (Vec<_>, Vec<_>) = faces.iter().map(compute_aabb_and_normal).unzip();

    check_intersecting_internal(vertices, faces, &aabbs, &normals)
}

/// Internal implementation of self-intersection check using pre-calculated AABBs and normals.
fn check_intersecting_internal<T: Float>(
    vertices: &[Vertex<T>],
    faces: &[Face],
    aabbs: &[AABB<T>],
    normals: &[Option<Vertex<T>>],
) -> bool {
    let num_faces = faces.len();

    let check_face_pair = |i: usize| {
        let aabb1 = &aabbs[i];
        for j in (i + 1)..num_faces {
            let aabb2 = &aabbs[j];

            if !aabb1.intersects(aabb2) {
                continue;
            }

            let f1 = &faces[i];
            let f2 = &faces[j];

            // Skip adjacent faces
            if f1.0 == f2.0
                || f1.0 == f2.1
                || f1.0 == f2.2
                || f1.1 == f2.0
                || f1.1 == f2.1
                || f1.1 == f2.2
                || f1.2 == f2.0
                || f1.2 == f2.1
                || f1.2 == f2.2
            {
                continue;
            }

            let t1 = [&vertices[f1.0], &vertices[f1.1], &vertices[f1.2]];
            let t2 = [&vertices[f2.0], &vertices[f2.1], &vertices[f2.2]];

            let n1 = &normals[i];
            let n2 = &normals[j];

            if let Some(n2) = n2
                && triangle_intersects_facet(t1, t2, n2)
            {
                return true;
            }

            if let Some(n1) = n1
                && triangle_intersects_facet(t2, t1, n1)
            {
                return true;
            }
        }
        false
    };

    #[cfg(feature = "rayon")]
    {
        (0..num_faces).into_par_iter().any(check_face_pair)
    }
    #[cfg(not(feature = "rayon"))]
    {
        (0..num_faces).any(check_face_pair)
    }
}

/// Check if triangle 1 intersects with the facet of triangle 2.
fn triangle_intersects_facet<T: Float>(
    t1: [&Vertex<T>; 3],
    t2: [&Vertex<T>; 3],
    n2: &Vertex<T>,
) -> bool {
    let eps = T::from(1e-6).unwrap();

    // 1. Calculate signed distances of all t1 vertices to t2's plane ONCE
    let d0 = n2.dot(&t1[0].sub(t2[2]));
    let d1 = n2.dot(&t1[1].sub(t2[2]));
    let d2 = n2.dot(&t1[2].sub(t2[2]));

    // 2. Early exit: If all points are on the exact same side of the plane, it cannot intersect.
    if (d0 > eps && d1 > eps && d2 > eps) || (d0 < -eps && d1 < -eps && d2 < -eps) {
        return false;
    }

    // 3. Test the 3 segments using pre-calculated distances
    check_segment_against_facet(t1[0], t1[1], d0, d1, t2, eps)
        || check_segment_against_facet(t1[1], t1[2], d1, d2, t2, eps)
        || check_segment_against_facet(t1[2], t1[0], d2, d0, t2, eps)
}

/// Check a single segment against a facet.
///
/// # Arguments
///
/// - `s0`: The first endpoint of the segment.
/// - `s1`: The second endpoint of the segment.
/// - `d0`: The signed distance of the first endpoint to the facet.
/// - `d1`: The signed distance of the second endpoint to the facet.
/// - `t2`: The vertices of the facet.
/// - `eps`: The tolerance for the signed distances.
fn check_segment_against_facet<T: Float>(
    s0: &Vertex<T>,
    s1: &Vertex<T>,
    d0: T,
    d1: T,
    t2: [&Vertex<T>; 3],
    eps: T,
) -> bool {
    if d0 * d1 > T::zero() {
        return false;
    }
    if d0.abs() <= eps || d1.abs() <= eps {
        return false;
    }

    let s0_sub_s1 = s0.sub(s1);
    let t2_sub_s1 = [t2[0].sub(s1), t2[1].sub(s1), t2[2].sub(s1)];

    let mut v = [T::zero(); 3];
    for (i, j) in [(0, 1), (1, 2), (2, 0)] {
        // Utilize the pre-subtracted vectors to save 6 function calls per segment
        let sv = t2_sub_s1[i].dot(&t2_sub_s1[j].cross(&s0_sub_s1));
        if sv.abs() < eps {
            return false;
        }
        v[i] = sv;
    }

    (v[0] > T::zero() && v[1] > T::zero() && v[2] > T::zero())
        || (v[0] < T::zero() && v[1] < T::zero() && v[2] < T::zero())
}

/// Check if there are any zero-area faces in the mesh using geometric approach.
///
/// `atol`: The tolerance for the area.
pub fn check_zero_area_faces<T: Float>(vertices: &[Vertex<T>], faces: &[Face], atol: T) -> bool {
    let atol_sq = atol * atol;

    let check_face = |face: &Face| {
        let v0 = &vertices[face.0];
        let v1 = &vertices[face.1];
        let v2 = &vertices[face.2];
        let cross = v1.sub(v0).cross(&v2.sub(v0));
        let area_sq = cross.0 * cross.0 + cross.1 * cross.1 + cross.2 * cross.2;
        area_sq < atol_sq
    };

    #[cfg(feature = "rayon")]
    {
        faces.par_iter().any(check_face)
    }
    #[cfg(not(feature = "rayon"))]
    {
        faces.iter().any(check_face)
    }
}

/// Check if the mesh is manifold.
///
/// # Returns
///
/// - `Ok(())` if the mesh is manifold
/// - `Err(MeshError::NonManifold)` if the mesh is non-manifold (more than 2 edges incident to a vertex)
/// - `Err(MeshError::OpenEdges)` if the mesh has open edges (isolated vertices)
pub fn check_manifold(map: &EdgeMap) -> Result<(), MeshError> {
    let check_counts = |(&(_v1, _v2), &count): (&(usize, usize), &u8)| {
        if count > 2 {
            return Some(Err(MeshError::NonManifold));
        }
        if count == 1 {
            return Some(Err(MeshError::OpenEdges));
        }
        None
    };

    #[cfg(feature = "rayon")]
    let result = map.counts.par_iter().find_map_any(check_counts);
    #[cfg(not(feature = "rayon"))]
    let result = map.counts.iter().find_map(check_counts);

    result.unwrap_or(Ok(()))
}

/// Check if the mesh has consistent normals.
#[inline]
pub fn check_consistent_normals(map: &EdgeMap) -> bool {
    let check_dir = |((_v1, _v2), &count): (&(usize, usize), &u8)| count <= 1;

    #[cfg(feature = "rayon")]
    {
        map.directions.par_iter().all(check_dir)
    }
    #[cfg(not(feature = "rayon"))]
    {
        map.directions.iter().all(check_dir)
    }
}

/// Check if the mesh has inward or outward orientation.
///
/// # Returns
///
/// `true` if the mesh has inward orientation, `false` otherwise.
pub fn check_inward_orientation<T: Float>(vertices: &[Vertex<T>], faces: &[Face]) -> bool {
    let calc_vol = |face: &Face| {
        let v0 = &vertices[face.0];
        let v1 = &vertices[face.1];
        let v2 = &vertices[face.2];
        let cross = v1.sub(v0).cross(&v2.sub(v0));
        v0.dot(&cross)
    };

    #[cfg(feature = "rayon")]
    let total_vol: T = faces.par_iter().map(calc_vol).sum();
    #[cfg(not(feature = "rayon"))]
    let total_vol: T = faces.iter().map(calc_vol).sum();

    total_vol < T::zero()
}

/// Check mesh properties and return a [MeshValidationReport].
///
/// This function is faster than calling individual checks because of loop fusion.
pub fn check_mesh<T: Float>(
    vertices: &[Vertex<T>],
    faces: &[Face],
    atol: T,
) -> MeshValidationReport {
    let mut report = MeshValidationReport::default();

    let atol_sq = atol * atol;

    // Combined pass 1: AABB, Normals (for intersection),
    // Volume (for inward check), Zero Area check
    let compute_all = |f: &Face| {
        let v0 = &vertices[f.0];
        let v1 = &vertices[f.1];
        let v2 = &vertices[f.2];

        let cross = v1.sub(v0).cross(&v2.sub(v0));
        let norm_sq = cross.0 * cross.0 + cross.1 * cross.1 + cross.2 * cross.2;

        let aabb = AABB::from_triangle(v0, v1, v2);
        let normal = if norm_sq < atol_sq {
            None
        } else {
            let inv_norm_sq = T::one() / norm_sq;
            Some(Vertex(
                cross.0 * inv_norm_sq,
                cross.1 * inv_norm_sq,
                cross.2 * inv_norm_sq,
            ))
        };

        let vol = v0.dot(&cross);
        let is_zero = norm_sq < atol_sq;

        (aabb, normal, vol, is_zero)
    };

    #[cfg(feature = "rayon")]
    let (aabbs, (normals, (volumes, zero_area_flags))): (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))) =
        faces
            .par_iter()
            .map(compute_all)
            .map(|(a, n, v, z)| (a, (n, (v, z))))
            .unzip();

    #[cfg(not(feature = "rayon"))]
    let (aabbs, (normals, (volumes, zero_area_flags))): (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))) =
        faces
            .iter()
            .map(compute_all)
            .map(|(a, n, v, z)| (a, (n, (v, z))))
            .unzip();

    let total_vol: T = volumes.into_iter().sum();
    report.inward_normals = total_vol < T::zero();
    report.zero_area_faces = zero_area_flags.into_iter().filter(|&b| b).count();

    // Pass 2: Edge map construction
    let edge_map = EdgeMap::from_faces(faces);

    // Pass 3: Edge map validation (manifold, consistent normals, open edges)
    // Check manifold and open edges together
    for &count in edge_map.counts.values() {
        if count > 2 {
            report.non_manifold += 1;
        } else if count == 1 {
            report.open_edges += 1;
        }
    }

    // Check inconsistent normals
    report.inconsistent_normals = edge_map.directions.values().any(|&count| count > 1);

    // Pass 4: Self-intersection (heavy)
    report.self_intersecting = check_intersecting_internal(vertices, faces, &aabbs, &normals);

    report
}

/// Validate the mesh and return a [MeshError] if it is not valid.
pub fn validate_mesh<T: Float>(
    vertices: &[Vertex<T>],
    faces: &[Face],
    atol: T,
) -> Result<(), MeshError> {
    if check_zero_area_faces(vertices, faces, atol) {
        return Err(MeshError::ZeroAreaFace);
    }

    let edge_map = EdgeMap::from_faces(faces);
    check_manifold(&edge_map)?;

    if !check_consistent_normals(&edge_map) {
        return Err(MeshError::InconsistentNormals);
    }

    if check_inward_orientation(vertices, faces) {
        return Err(MeshError::InwardNormals);
    }

    if check_intersecting(vertices, faces) {
        return Err(MeshError::SelfIntersecting);
    }

    Ok(())
}
