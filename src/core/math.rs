use num_traits::Float as NumFloat;

pub trait Float: NumFloat + Send + Sync + core::iter::Sum {}

impl<T: NumFloat + Send + Sync + core::iter::Sum> Float for T {}

macro_rules! paste_meta {
    ($($meta:meta => { $($item:item)* }),*) => {
        $($(#[$meta] $item)*)*
    };
}

paste_meta! {
    cfg(feature = "rayon") => {
        pub trait OptionalSync: Sync {}
        impl<T: Sync> OptionalSync for T {}
        pub trait OptionalSend: Send {}
        impl<T: Send> OptionalSend for T {}
    },
    cfg(not(feature = "rayon")) => {
        pub trait OptionalSync {}
        impl<T> OptionalSync for T {}
        pub trait OptionalSend {}
        impl<T> OptionalSend for T {}
    }
}
