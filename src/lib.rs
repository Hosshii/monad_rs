#![feature(generic_associated_types)]
#![allow(incomplete_features)]

pub mod applicative;
pub mod functor;
pub mod monad;
pub mod types;

pub use applicative::Applicative;
pub use functor::Functor;
pub use monad::Monad;

pub mod type_class {
    pub use super::{Applicative, Functor, Monad};
}
