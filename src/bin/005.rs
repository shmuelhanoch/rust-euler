// Problem 5 - Smallest Multiple
//
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//
// Solution: just multiply the highset powers of each primes which smaller then 20!

fn main() {
    println!("{}", 16 * 9 * 5 * 7 * 11 * 13 * 17 * 19)
}
