use crate::core::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vertex<T: Float>(pub T, pub T, pub T);

impl<T: Float> Vertex<T> {
    #[inline]
    pub fn sub(&self, other: &Self) -> Self {
        Vertex(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Vertex(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl<T: Float> From<[T; 3]> for Vertex<T> {
    #[inline]
    fn from(arr: [T; 3]) -> Self {
        Self(arr[0], arr[1], arr[2])
    }
}

impl<T: Float> From<(T, T, T)> for Vertex<T> {
    #[inline]
    fn from(tup: (T, T, T)) -> Self {
        Self(tup.0, tup.1, tup.2)
    }
}

#[cfg(feature = "nalgebra")]
mod nalgebra_impl {
    use super::*;
    use nalgebra::{Point3, RealField, Vector3};

    impl<T: Float + RealField> From<Point3<T>> for Vertex<T> {
        #[inline]
        fn from(p: Point3<T>) -> Self {
            Self(p.x, p.y, p.z)
        }
    }

    impl<T: Float + RealField> From<Vector3<T>> for Vertex<T> {
        #[inline]
        fn from(v: Vector3<T>) -> Self {
            Self(v.x, v.y, v.z)
        }
    }

    impl<T: Float + RealField> From<Vertex<T>> for Point3<T> {
        #[inline]
        fn from(v: Vertex<T>) -> Self {
            Point3::new(v.0, v.1, v.2)
        }
    }
}
