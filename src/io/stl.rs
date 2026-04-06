use crate::{Face, Mesh, Vertex, core::Float};

impl<T: Float> Mesh<T, Vertex<T>, Face> {
    /// Construct a [`Mesh`] from an STL file.
    #[inline]
    pub fn from_stl<R: std::io::Read + std::io::Seek>(reader: &mut R) -> std::io::Result<Self> {
        let stl_mesh = stl_io::read_stl(reader)?;
        Ok(stl_mesh.into())
    }
}

impl<T: Float> From<stl_io::IndexedMesh> for Mesh<T, Vertex<T>, Face> {
    fn from(indexed_mesh: stl_io::IndexedMesh) -> Self {
        let vertices: Vec<Vertex<T>> = indexed_mesh
            .vertices
            .into_iter()
            .map(|v| {
                Vertex(
                    T::from(v[0]).unwrap(),
                    T::from(v[1]).unwrap(),
                    T::from(v[2]).unwrap(),
                )
            })
            .collect();

        let faces: Vec<Face> = indexed_mesh
            .faces
            .into_iter()
            .map(|f| Face(f.vertices[0], f.vertices[1], f.vertices[2]))
            .collect();

        Self::new(vertices, faces)
    }
}
