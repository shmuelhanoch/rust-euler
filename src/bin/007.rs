//By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

//What is the 10 001st prime number?
//
// Solution: we will implement a basic sieve (up to 1M, just in case),
// then count until the 10001 prime.

fn main() {
    let lim = 1000000;
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
    let mut i = 0;
    let mut cnt = 0;
    loop {
        i += 1;
        if ps[i] { cnt += 1; }
        if cnt == 10001 {
            println!("{}", i);
            break;
        }
    }
}
