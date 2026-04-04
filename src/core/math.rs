use num_traits::Float as NumFloat;

pub trait Float: NumFloat + Send + Sync + std::iter::Sum {}

impl<T: NumFloat + Send + Sync + std::iter::Sum> Float for T {}
