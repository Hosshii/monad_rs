use crate::functor::Functor;

pub trait Applicative: Functor {
    fn pure(t: Self::Unwrapped) -> Self::Wrapped<Self::Unwrapped>;

    // lift_a1 :: f a -> f (a -> b) -> f b
    fn lift_a1<F, B>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B;
}
