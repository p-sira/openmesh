use hashbrown::HashMap;
use rustc_hash::FxBuildHasher;

use crate::Face;

pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Map of edges to their counts and directions.
pub struct EdgeMap {
    /// Key: (v1, v2) sorted, Value: number of faces sharing this edge
    pub counts: FxHashMap<(usize, usize), usize>,
    /// Key: (v1, v2) directed, Value: number of times this specific direction occurs
    pub directions: FxHashMap<(usize, usize), usize>,
}

#[cfg(feature = "rayon")]
use rayon::prelude::*;

impl EdgeMap {
    #[inline]
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
                            let count = c1.entry(k).or_insert(0);
                            *count = count.saturating_add(v);
                        }
                        for (k, v) in d2 {
                            let count = d1.entry(k).or_insert(0);
                            *count = count.saturating_add(v);
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

    #[inline]
    fn update_maps(
        counts: &mut FxHashMap<(usize, usize), usize>,
        directions: &mut FxHashMap<(usize, usize), usize>,
        f: &Face,
    ) {
        let edges = [(f.0, f.1), (f.1, f.2), (f.2, f.0)];
        for &(v1, v2) in &edges {
            // Directed: track the winding order
            let direction_count = directions.entry((v1, v2)).or_insert(0);
            *direction_count = direction_count.saturating_add(1);

            // Undirected: track topological connectivity
            let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            let edge_count = counts.entry(key).or_insert(0);
            *edge_count = edge_count.saturating_add(1);
        }
    }
}
