use monad_rs::{types::maybe::Maybe, Applicative, Functor, Monad};
use Maybe::*;
fn main() {
    let plus3 = |x| x + 3;
    let m_plus3 = |x| Just(x + 3);
    println!("{:?}", Just(3).fmap(plus3));
    println!("{:?}", Nothing.fmap(plus3));
    println!("{:?}", Just(3).lift_a1(Just(plus3)));
    println!("{:?}", Nothing.lift_a1(Just(plus3)));
    println!("{:?}", Just(3).lift_a1(Nothing::<fn(i32) -> i32>));
    println!("{:?}", Just(3).bind(m_plus3).bind(m_plus3));
    println!("{:?}", Nothing.bind(m_plus3));

    println!();

    let success = Maybe::pure((0, 0))
        .bind(|x| land_left(1, x))
        .bind(|x| land_right(2, x));

    println!("{:?}", success);

    let oops = Maybe::pure((0, 0))
        .bind(|x| land_left(1, x))
        .bind(|x| land_right(4, x))
        .bind(|x| land_left(-1, x).bind(|x| land_right(-2, x)));
    println!("{:?}", oops);
}

type Birds = isize;

type Pole = (Birds, Birds);

fn land_left(n: Birds, (l, r): Pole) -> Maybe<Pole> {
    match ((l + n) - r).abs() {
        x if x < 4 => Just((l + n, r)),
        _ => Nothing,
    }
}

fn land_right(n: Birds, (l, r): Pole) -> Maybe<Pole> {
    match (l - (r + n)).abs() {
        x if x < 4 => Just((l, r + n)),
        _ => Nothing,
    }
}
