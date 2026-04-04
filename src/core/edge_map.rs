use crate::Face;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Map of edges to their counts and directions.
pub struct EdgeMap {
    /// Key: (v1, v2) sorted, Value: number of faces sharing this edge
    pub counts: FxHashMap<(usize, usize), u8>,
    /// Key: (v1, v2) directed, Value: number of times this specific direction occurs
    pub directions: FxHashMap<(usize, usize), u8>,
}

#[cfg(feature = "rayon")]
use rayon::prelude::*;

impl EdgeMap {
    pub fn from_faces(faces: &[Face]) -> Self {
        #[cfg(feature = "rayon")]
        {
            let (counts, directions) = faces
                .par_iter()
                .fold(
                    || (FxHashMap::default(), FxHashMap::default()),
                    |(mut counts, mut directions), f| {
                        Self::update_maps(&mut counts, &mut directions, f);
                        (counts, directions)
                    },
                )
                .reduce(
                    || (FxHashMap::default(), FxHashMap::default()),
                    |(mut c1, mut d1), (c2, d2)| {
                        for (k, v) in c2 {
                            *c1.entry(k).or_insert(0) += v;
                        }
                        for (k, v) in d2 {
                            *d1.entry(k).or_insert(0) += v;
                        }
                        (c1, d1)
                    },
                );
            Self { counts, directions }
        }
        #[cfg(not(feature = "rayon"))]
        {
            let mut counts = FxHashMap::default();
            let mut directions = FxHashMap::default();

            for f in faces {
                Self::update_maps(&mut counts, &mut directions, f);
            }
            Self { counts, directions }
        }
    }

    fn update_maps(
        counts: &mut FxHashMap<(usize, usize), u8>,
        directions: &mut FxHashMap<(usize, usize), u8>,
        f: &Face,
    ) {
        let edges = [(f.0, f.1), (f.1, f.2), (f.2, f.0)];
        for &(v1, v2) in &edges {
            // Directed: track the winding order
            *directions.entry((v1, v2)).or_insert(0) += 1;

            // Undirected: track topological connectivity
            let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            *counts.entry(key).or_insert(0) += 1;
        }
    }
}
