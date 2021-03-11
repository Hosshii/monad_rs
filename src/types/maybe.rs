use crate::type_class::*;

#[derive(Debug)]
pub enum Maybe<A> {
    Nothing,
    Just(A),
}

impl<A> Functor for Maybe<A> {
    type Unwrapped = A;
    type Wrapped<B> = Maybe<B>;
    fn fmap<F, B>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B,
    {
        use Maybe::*;
        match self {
            Nothing => Nothing,
            Just(x) => Just(f(x)),
        }
    }
}

impl<A> Applicative for Maybe<A> {
    fn pure(t: Self::Unwrapped) -> Self::Wrapped<Self::Unwrapped> {
        Self::Just(t)
    }

    fn lift_a1<F, B>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B,
    {
        use Maybe::*;
        match f {
            Nothing => Nothing,
            Just(x) => self.fmap(x),
        }
    }
}

impl<A> Monad for Maybe<A> {
    fn bind<F, B>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        use Maybe::*;
        match self {
            Nothing => Nothing,
            Just(x) => f(x),
        }
    }
}
