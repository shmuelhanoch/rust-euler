fn max_prime_factor(n: u64) -> u64 {
    let mut p = 2;
    let mut ret = n;
    while p * p <= ret {
        while ret % p == 0 {
            ret /= p;
        }
        p += 1;
    }
    ret
}

fn main() {
    println!("{}", max_prime_factor(600851475143));
}
