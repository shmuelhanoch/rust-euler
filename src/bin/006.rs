// Problem 6 - Sum square difference
//
// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.
//
// Solution: Its a known fact that (1+...+n)^2 = (1^3+...n^3). So:
// (1+...+n)^2 - (1^2+...+n^2) = (1^3-1^2) + ... + (n^3-n^2)

fn main() {
    let res: i64 = (1..101).map(|k: i64| k*k*(k-1)).sum();
    println!("{}", res);
}
