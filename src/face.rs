#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face(pub usize, pub usize, pub usize);

impl From<[usize; 3]> for Face {
    #[inline]
    fn from(arr: [usize; 3]) -> Self {
        Self(arr[0], arr[1], arr[2])
    }
}

impl From<(usize, usize, usize)> for Face {
    #[inline]
    fn from(tup: (usize, usize, usize)) -> Self {
        Self(tup.0, tup.1, tup.2)
    }
}

#[cfg(feature = "nalgebra")]
impl From<nalgebra::Vector3<usize>> for Face {
    #[inline]
    fn from(indices: nalgebra::Vector3<usize>) -> Self {
        Self(indices.x, indices.y, indices.z)
    }
}
