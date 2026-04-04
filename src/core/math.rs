use num_traits::Float as NumFloat;

pub trait Float: NumFloat + Send + Sync + core::iter::Sum {}

impl<T: NumFloat + Send + Sync + core::iter::Sum> Float for T {}
