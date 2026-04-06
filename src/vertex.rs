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

    #[inline]
    pub fn to_inner(self) -> (T, T, T) {
        (self.0, self.1, self.2)
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
impl<T: Float> From<nalgebra::Vector3<T>> for Vertex<T> {
    #[inline]
    fn from(v: nalgebra::Vector3<T>) -> Self {
        let arr = v.as_slice();
        Self(arr[0], arr[1], arr[2])
    }
}

pub trait VertexView<T: Float>: crate::core::OptionalSync {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;

    #[inline]
    fn to_vertex(&self) -> Vertex<T> {
        Vertex(self.x(), self.y(), self.z())
    }
}

impl<T: Float> VertexView<T> for [T; 3] {
    #[inline]
    fn x(&self) -> T {
        self[0]
    }
    #[inline]
    fn y(&self) -> T {
        self[1]
    }
    #[inline]
    fn z(&self) -> T {
        self[2]
    }
}

impl<T: Float> VertexView<T> for Vertex<T> {
    #[inline]
    fn x(&self) -> T {
        self.0
    }
    #[inline]
    fn y(&self) -> T {
        self.1
    }
    #[inline]
    fn z(&self) -> T {
        self.2
    }
}

#[cfg(feature = "nalgebra")]
impl<T: Float> VertexView<T> for nalgebra::Vector3<T> {
    #[inline]
    fn x(&self) -> T {
        self[0]
    }
    #[inline]
    fn y(&self) -> T {
        self[1]
    }
    #[inline]
    fn z(&self) -> T {
        self[2]
    }
}
