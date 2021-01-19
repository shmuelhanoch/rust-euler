// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.
//
// Solution: use a sieve to generate all the primes, then sum them up.

fn main() {
    let lim = 2000000;
    let mut ps = vec![true; lim];
    ps[0] = false;
    ps[1] = false;
    let mut p = 0;
    loop {
        p += 1;
        while !ps[p] { p += 1; }
        let mut i = p * p;
        while i < lim {
            ps[i] = false;
            i += p;
        }
        if p * p >= lim { break; }
    }
    let sum = ps
        .iter()
        .enumerate()
        .fold(0, |acc, (i, is_p)| if *is_p {acc + i} else {acc});
    println!("{}", sum);
}
