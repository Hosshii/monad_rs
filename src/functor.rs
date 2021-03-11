pub trait Functor {
    type Unwrapped;
    type Wrapped<A>;

    // f a -> (a -> b) -> f b
    fn fmap<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B;
}
