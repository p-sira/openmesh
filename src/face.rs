#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Face(pub usize, pub usize, pub usize);

impl Face {
    #[inline]
    pub fn to_inner(self) -> (usize, usize, usize) {
        (self.0, self.1, self.2)
    }
}

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

pub trait FaceView: crate::core::OptionalSync + crate::core::OptionalSend {
    fn indices(&self) -> (usize, usize, usize);

    #[inline]
    fn to_face(&self) -> Face {
        let (i0, i1, i2) = self.indices();
        Face(i0, i1, i2)
    }
}

impl FaceView for [usize; 3] {
    #[inline]
    fn indices(&self) -> (usize, usize, usize) {
        (self[0], self[1], self[2])
    }
}

impl FaceView for Face {
    #[inline]
    fn indices(&self) -> (usize, usize, usize) {
        (self.0, self.1, self.2)
    }
}
