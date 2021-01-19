// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2

// For example, 32 + 42 = 9 + 16 = 25 = 52.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

#[macro_use] extern crate mdo;

fn main() {
    use mdo::iter::{bind, ret, mzero};
    use std::cmp;

    let l = mdo! {
        z =<< 1..1000;
        x =<< 1..cmp::max(z, 1000-z);
        let y = 1000 - z - x;
        when x * x + y * y == z * z;
        ret ret(x * y * z)
    }.collect::<Vec<_>>();
    println!("{}", l[0]);
}
