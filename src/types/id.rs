use crate::type_class::*;

#[derive(Debug)]
pub struct Id<M>(pub M);

impl<A> Functor for Id<A> {
    type Unwrapped = A;
    type Wrapped<B> = Id<B>;
    fn fmap<F, B>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B,
    {
        Id(f(self.0))
    }
}

impl<A> Applicative for Id<A> {
    fn pure(t: Self::Unwrapped) -> Self::Wrapped<Self::Unwrapped> {
        Id(t)
    }
    fn lift_a1<F, B>(self, mut f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B,
    {
        Id(f.0(self.0))
    }
}

impl<A> Monad for Id<A> {
    fn bind<F, B>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        f(self.0)
    }
}
