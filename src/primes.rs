//! # prime_utils
//!
//! `prime_utils` is a some common functions realted to prime numbers.

use num::Integer;
use num::ToPrimitive;

/// Returns a sieve of primes until a given limit, reperesented as a
/// vector of boolean inidcating primeness.
///
/// # Arguments
///
/// * `n` - an integer indicates the required limit for the sieve, not inclusive.
///
pub fn gen_sieve<T>(n: T) -> Vec<bool>
where
    T: Clone + Copy + Integer + ToPrimitive,
{
    assert!(n > T::zero());
    let mut ps = vec![true; n.to_usize().unwrap()];
    ps[0] = false;
    if n > T::one() { ps[1] = false; }
    let mut p = T::zero();
    loop {
        p = p + T::one();
        while p < n && !ps[p.to_usize().unwrap()] { p = p + T::one(); }
        let mut i = p * p;
        while i < n {
            ps[i.to_usize().unwrap()] = false;
            i = i + p;
        }
        if p * p >= n { break; }
    }
    ps
}

/// Returns a vector containing all the primes until `n` (not inclusive).
pub fn primes_until<T>(n: T) -> Vec<T>
where
    T: Clone + Copy + Integer + ToPrimitive,
{
    gen_sieve(n)
        .iter()
        .zip(num::range(T::zero(), n))
        .filter(|&(&p, _)| p)
        .map(|(_, idx)| idx)
        .collect()
}


pub fn factorize<T, U>(n: T) -> Vec<(T, U)>
where
    T: Clone + Copy + Integer + ToPrimitive,
    U: Integer,
{
    let mut ret: Vec<(T, U)> = Vec::new();
    let mut n0 = n;
    let mut p = T::one() + T::one();
    while n0 > T::one() {
        let mut exp = U::zero();
        while n0 % p == T::zero() {
            n0 = n0 / p;
            exp = exp + U::one();
        }
        if exp > U::zero() {
            ret.push((p, exp));
        }
        p = p + T::one();
    }
    ret
}
