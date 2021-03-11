use crate::applicative::Applicative;
pub trait Monad: Applicative {
    fn bind<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>;
}
