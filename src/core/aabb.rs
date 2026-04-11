use crate::{Vertex, core::Float};

#[allow(clippy::upper_case_acronyms)]
pub(crate) struct AABB<T: Float> {
    min: [T; 3],
    max: [T; 3],
}

impl<T: Float> AABB<T> {
    /// Construct an axis-aligned bounding box from a triangle.
    ///
    /// Note: The bounding box is expanded by a small epsilon to account for floating-point precision errors.
    #[inline]
    pub fn from_triangle(v0: &Vertex<T>, v1: &Vertex<T>, v2: &Vertex<T>) -> Self {
        let eps = T::epsilon() * T::from(10.0).unwrap();

        let min_x = v0.0.min(v1.0.min(v2.0)) - eps;
        let min_y = v0.1.min(v1.1.min(v2.1)) - eps;
        let min_z = v0.2.min(v1.2.min(v2.2)) - eps;

        let max_x = v0.0.max(v1.0.max(v2.0)) + eps;
        let max_y = v0.1.max(v1.1.max(v2.1)) + eps;
        let max_z = v0.2.max(v1.2.max(v2.2)) + eps;

        AABB {
            min: [min_x, min_y, min_z],
            max: [max_x, max_y, max_z],
        }
    }

    #[inline]
    pub fn intersects(&self, other: &AABB<T>) -> bool {
        if self.max[0] < other.min[0] || self.min[0] > other.max[0] {
            return false;
        }
        if self.max[1] < other.min[1] || self.min[1] > other.max[1] {
            return false;
        }
        if self.max[2] < other.min[2] || self.min[2] > other.max[2] {
            return false;
        }
        true
    }
}
