use crate::{Face, Mesh, Vertex, core::Float};

impl<T: Float> Mesh<T, Vertex<T>, Face> {
    /// Construct a [`Mesh`] from an OBJ file.
    #[inline]
    pub fn from_obj<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let data = obj::ObjData::load_buf(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(data.into())
    }
}

impl<T: Float> From<obj::ObjData> for Mesh<T, Vertex<T>, Face> {
    fn from(data: obj::ObjData) -> Self {
        let vertices: Vec<Vertex<T>> = data
            .position
            .into_iter()
            .map(|pos| {
                Vertex(
                    T::from(pos[0]).unwrap(),
                    T::from(pos[1]).unwrap(),
                    T::from(pos[2]).unwrap(),
                )
            })
            .collect();

        let mut faces: Vec<Face> = Vec::new();
        for object in data.objects {
            for group in object.groups {
                for poly in group.polys {
                    if poly.0.len() >= 3 {
                        let first = poly.0[0].0;
                        for i in 1..poly.0.len() - 1 {
                            faces.push(Face(first, poly.0[i].0, poly.0[i + 1].0));
                        }
                    }
                }
            }
        }

        Self::new(vertices, faces)
    }
}
