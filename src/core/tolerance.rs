use crate::{Vertex, core::Float};

use super::MeshError;

/// Per-face tolerance for classifying degenerate triangles.
///
/// `absolute_double_area` has squared-length units and is compared with twice
/// the triangle area. `relative` is dimensionless and limits the ratio between
/// the altitude and the longest edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaceTolerance<T: Float> {
    absolute_double_area: T,
    relative: T,
}

impl<T: Float> FaceTolerance<T> {
    /// Create a validated mixed absolute and relative tolerance.
    #[inline]
    pub fn new(absolute_double_area: T, relative: T) -> Result<Self, MeshError> {
        if !absolute_double_area.is_finite()
            || absolute_double_area < T::zero()
            || !relative.is_finite()
            || relative < T::zero()
        {
            return Err(MeshError::InvalidTolerance);
        }

        Ok(Self {
            absolute_double_area,
            relative,
        })
    }

    /// Create an absolute tolerance expressed as triangle area.
    #[inline]
    pub fn from_area(absolute_area: T) -> Result<Self, MeshError> {
        if !absolute_area.is_finite() || absolute_area < T::zero() {
            return Err(MeshError::InvalidTolerance);
        }

        let absolute_double_area = absolute_area + absolute_area;
        if !absolute_double_area.is_finite() {
            return Err(MeshError::InvalidTolerance);
        }

        Self::new(absolute_double_area, T::zero())
    }

    #[inline]
    pub fn absolute_double_area(self) -> T {
        self.absolute_double_area
    }

    #[inline]
    pub fn relative(self) -> T {
        self.relative
    }
}

/// Classify a face after its coordinates and tolerance have been validated.
///
/// Scaling edge components keeps intermediate cross products bounded. `None`
/// means finite input coordinates produced a non-finite edge difference.
#[inline]
pub(crate) fn is_degenerate<T: Float>(
    v0: &Vertex<T>,
    v1: &Vertex<T>,
    v2: &Vertex<T>,
    tolerance: FaceTolerance<T>,
) -> Option<bool> {
    let e01 = v1.sub(v0);
    let e02 = v2.sub(v0);
    let e12 = v2.sub(v1);

    let scale = max_component(&e01)
        .max(max_component(&e02))
        .max(max_component(&e12));
    if !scale.is_finite() {
        return None;
    }
    if scale == T::zero() {
        return Some(true);
    }

    let e01 = scaled(&e01, scale);
    let e02 = scaled(&e02, scale);
    let e12 = scaled(&e12, scale);
    let cross = e01.cross(&e02);
    let cross_norm = cross.0.hypot(cross.1).hypot(cross.2);
    let max_edge_length_sq = e01.dot(&e01).max(e02.dot(&e02)).max(e12.dot(&e12));
    let scaled_absolute = tolerance.absolute_double_area / scale / scale;
    let threshold = scaled_absolute + tolerance.relative * max_edge_length_sq;

    Some(cross_norm <= threshold)
}

#[inline]
fn max_component<T: Float>(v: &Vertex<T>) -> T {
    v.0.abs().max(v.1.abs()).max(v.2.abs())
}

#[inline]
fn scaled<T: Float>(v: &Vertex<T>, scale: T) -> Vertex<T> {
    Vertex(v.0 / scale, v.1 / scale, v.2 / scale)
}
